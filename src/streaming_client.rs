use super::*;
use futures::Stream;
use reqwest::multipart;
use reqwest::{Client as ReqwestClient, Error as ReqwestError, Response};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// Gotenberg Streaming API client. Available with the `streaming` feature enabled.
#[derive(Clone)]
pub struct StreamingClient {
    client: ReqwestClient,
    base_url: String,
    username: Option<String>,
    password: Option<String>,
}

impl Drop for StreamingClient {
    fn drop(&mut self) {
        // Securely zeroize the username and password
        #[cfg(feature = "zeroize")]
        {
            if let Some(username) = &mut self.username {
                username.zeroize();
            }
            if let Some(password) = &mut self.password {
                password.zeroize();
            }
        }
    }
}

impl Debug for StreamingClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamingClient")
            .field("base_url", &self.base_url)
            .field("username", &self.username)
            .finish()
    }
}

impl StreamingClient {
    /// Create a new instance of the API client.
    pub fn new(base_url: &str) -> Self {
        // Strip trailing slashes
        let base_url = base_url.trim_end_matches('/');

        StreamingClient {
            client: ReqwestClient::new(),
            base_url: base_url.to_string(),
            username: None,
            password: None,
        }
    }

    /// Create a new instance of the API client with basic auth.
    /// You can set the username and password on the Gotenberg server by starting it with `--api-enable-basic-auth` and supplying `GOTENBERG_API_BASIC_AUTH_USERNAME` and `GOTENBERG_API_BASIC_AUTH_PASSWORD` environment variables.
    pub fn new_with_auth(base_url: &str, username: &str, password: &str) -> Self {
        // Strip trailing slashes
        let base_url = base_url.trim_end_matches('/');

        StreamingClient {
            client: ReqwestClient::new(),
            base_url: base_url.to_string(),
            username: Some(username.to_string()),
            password: Some(password.to_string()),
        }
    }

    async fn post_stream(
        &self,
        endpoint: &str,
        form: multipart::Form,
        trace: Option<String>,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = self.client.post(&url).multipart(form);

        if let Some(trace) = trace {
            req = req.header("Gotenberg-Trace", trace);
        }

        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            req = req.basic_auth(username, Some(password));
        }

        let response = req.send().await.map_err(Into::into)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::RenderingError(format!(
                "Failed to render PDF: {} - {}",
                status, body
            )));
        }

        Ok(response.bytes_stream())
    }

    /// Generic POST method that takes a multipart form and sends it.
    /// Used for utility methods that don't require streaming.
    async fn post(
        &self,
        endpoint: &str,
        form: multipart::Form,
        trace: Option<String>,
    ) -> Result<Bytes, Error> {
        let url = format!("{}/{}", self.base_url, endpoint);

        let mut req = self.client.post(&url).multipart(form);
        if let Some(trace) = trace {
            req = req.header("Gotenberg-Trace", trace);
        }

        // Add basic auth if username and password are provided
        if let (Some(username), Some(password)) = (&self.username, &self.password) {
            req = req.basic_auth(username, Some(password));
        }

        let response: Response = req.send().await.map_err(Into::into)?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::RenderingError(format!(
                "Failed to render PDF: {} - {}",
                status, body
            )));
        }

        response.bytes().await.map_err(Into::into)
    }

    /// Convert a URL to a PDF using the Chromium engine.
    pub async fn pdf_from_url(
        &self,
        url: &str,
        options: WebOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();
        let form = multipart::Form::new().text("url", url.to_string());
        let form = options.fill_form(form);

        self.post_stream("forms/chromium/convert/url", form, trace)
            .await
    }

    /// Convert HTML to a PDF using the Chromium engine.
    pub async fn pdf_from_html(
        &self,
        html: &str,
        options: WebOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();
        let file_bytes = html.to_string().into_bytes();
        let part = multipart::Part::bytes(file_bytes)
            .file_name("index.html")
            .mime_str("text/html")
            .unwrap();

        let form = multipart::Form::new().part("index.html", part);
        let form = options.fill_form(form);

        self.post_stream("forms/chromium/convert/html", form, trace)
            .await
    }

    /// Convert Markdown to a PDF using the Chromium engine.
    ///
    /// The HTML template should in the following format:
    ///
    /// ```html
    /// <!doctype html>
    /// <html lang="en">
    ///  <head>
    ///    <meta charset="utf-8">
    ///    <title>My PDF</title>
    ///  </head>
    ///  <body>
    ///    {{ toHTML "file.md" }}
    ///  </body>
    /// </html>
    /// ```
    ///
    /// The markdown files should be in a "filename" => "content" format. The filename key string must end with `.md`.
    pub async fn pdf_from_markdown(
        &self,
        html_template: &str,
        markdown: HashMap<&str, &str>,
        options: WebOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();

        let file_bytes = html_template.to_string().into_bytes();
        let html_part = multipart::Part::bytes(file_bytes)
            .file_name("index.html")
            .mime_str("text/html")
            .unwrap();

        let mut form = multipart::Form::new().part("index.html", html_part);
        for (filename, content) in markdown {
            if !filename.ends_with(".md") {
                return Err(Error::FilenameError(
                    "Markdown filename must end with '.md'".to_string(),
                ));
            }
            let part = multipart::Part::bytes(content.as_bytes().to_vec())
                .file_name(filename.to_string())
                .mime_str("text/markdown")
                .unwrap();
            form = form.part(filename.to_string(), part);
        }

        let form = options.fill_form(form);
        self.post_stream("forms/chromium/convert/markdown", form, trace)
            .await
    }

    /// Take a screenshot of a webpage using the Chromium engine.
    pub async fn screenshot_url(
        &self,
        url: &str,
        options: ScreenshotOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();
        let form = multipart::Form::new().text("url", url.to_string());
        let form = options.fill_form(form);

        self.post_stream("forms/chromium/screenshot/url", form, trace)
            .await
    }

    /// Take a screenshot of an HTML page using the Chromium engine.
    pub async fn screenshot_html(
        &self,
        html: &str,
        options: ScreenshotOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();
        let file_bytes = html.to_string().into_bytes();
        let part = multipart::Part::bytes(file_bytes)
            .file_name("index.html")
            .mime_str("text/html")
            .unwrap();

        let form = multipart::Form::new().part("index.html", part);
        let form = options.fill_form(form);

        self.post_stream("forms/chromium/screenshot/html", form, trace)
            .await
    }

    /// Take a screenshot of a set of markdown files using the Chromium engine.
    pub async fn screenshot_markdown(
        &self,
        html_template: &str,
        markdown: HashMap<&str, &str>,
        options: ScreenshotOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();

        let file_bytes = html_template.to_string().into_bytes();
        let html_part = multipart::Part::bytes(file_bytes)
            .file_name("index.html")
            .mime_str("text/html")
            .unwrap();

        let mut form = multipart::Form::new().part("index.html", html_part);
        for (filename, content) in markdown {
            if !filename.ends_with(".md") {
                return Err(Error::FilenameError(
                    "Markdown filename must end with '.md'".to_string(),
                ));
            }
            let part = multipart::Part::bytes(content.as_bytes().to_vec())
                .file_name(filename.to_string())
                .mime_str("text/markdown")
                .unwrap();
            form = form.part(filename.to_string(), part);
        }

        let form = options.fill_form(form);

        self.post_stream("forms/chromium/screenshot/markdown", form, trace)
            .await
    }

    /// Convert a document to a PDF using the LibreOffice engine.
    ///
    /// Supports the following file formats:
    /// ```txt
    /// .123 .602 .abw .bib .bmp .cdr .cgm .cmx .csv .cwk .dbf .dif .doc
    /// .docm .docx .dot .dotm .dotx .dxf .emf .eps .epub .fodg .fodp .fods
    /// .fodt .fopd .gif .htm .html .hwp .jpeg .jpg .key .ltx .lwp .mcw .met
    /// .mml .mw .numbers .odd .odg .odm .odp .ods .odt .otg .oth .otp .ots .ott
    /// .pages .pbm .pcd .pct .pcx .pdb .pdf .pgm .png .pot .potm .potx .ppm .pps
    /// .ppt .pptm .pptx .psd .psw .pub .pwp .pxl .ras .rtf .sda .sdc .sdd .sdp .sdw
    /// .sgl .slk .smf .stc .std .sti .stw .svg .svm .swf .sxc .sxd .sxg .sxi .sxm
    /// .sxw .tga .tif .tiff .txt .uof .uop .uos .uot .vdx .vor .vsd .vsdm .vsdx
    /// .wb2 .wk1 .wks .wmf .wpd .wpg .wps .xbm .xhtml .xls .xlsb .xlsm .xlsx .xlt
    /// .xltm .xltx .xlw .xml .xpm .zabw
    /// ```
    pub async fn pdf_from_doc(
        &self,
        filename: &str,
        bytes: Vec<u8>,
        options: DocumentOptions,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let trace = options.trace_id.clone();

        let part = multipart::Part::bytes(bytes).file_name(filename.to_string());
        let form = multipart::Form::new().part("files", part);
        let form = options.fill_form(form);

        self.post_stream("forms/libreoffice/convert", form, trace)
            .await
    }

    /// Transforms a PDF file into the requested PDF/A format and/or PDF/UA.
    pub async fn convert_pdf(
        &self,
        pdf_bytes: Vec<u8>,
        pdfa: Option<PDFFormat>,
        pdfua: bool,
    ) -> Result<impl Stream<Item = Result<Bytes, ReqwestError>>, Error> {
        let pdf_part = multipart::Part::bytes(pdf_bytes).file_name("file.pdf".to_string());
        let mut form = multipart::Form::new().part("file.pdf", pdf_part);

        if let Some(pdfa) = pdfa {
            form = form.text("pdfa", pdfa.to_string());
        }

        form = form.text("pdfua", pdfua.to_string());

        self.post_stream("forms/pdfengines/convert", form, None)
            .await
    }

    /// Read the metadata of a PDF file
    pub async fn read_metadata(
        &self,
        pdf_bytes: Vec<u8>,
    ) -> Result<HashMap<String, serde_json::Value>, Error> {
        let form = multipart::Form::new();
        let part = multipart::Part::bytes(pdf_bytes).file_name("file.pdf".to_string());
        let form = form.part("file.pdf", part);

        #[derive(Debug, Deserialize)]
        pub struct MeatadataContainer {
            #[serde(rename = "file.pdf")]
            pub filepdf: HashMap<String, serde_json::Value>,
        }

        let bytes = self
            .post("forms/pdfengines/metadata/read", form, None)
            .await?;
        let metadata: MeatadataContainer = serde_json::from_slice(&bytes).map_err(|e| {
            Error::ParseError(
                "Metadata".to_string(),
                String::from_utf8_lossy(&bytes).to_string(),
                e.to_string(),
            )
        })?;

        Ok(metadata.filepdf)
    }

    /// Write metadata to a PDF file
    pub async fn write_metadata(
        &self,
        pdf_bytes: Vec<u8>,
        metadata: HashMap<String, serde_json::Value>,
    ) -> Result<Bytes, Error> {
        let form = multipart::Form::new();
        let part = multipart::Part::bytes(pdf_bytes).file_name("file.pdf".to_string());
        let form = form.part("file.pdf", part);
        let metadata = serde_json::to_string(&metadata).map_err(|e| {
            Error::ParseError("Metadata".to_string(), "".to_string(), e.to_string())
        })?;
        let part = multipart::Part::text(metadata);
        let form = form.part("metadata", part);
        self.post("forms/pdfengines/metadata/write", form, None)
            .await
    }

    /// Get the health status of the Gotenberg server.
    pub async fn health_check(&self) -> Result<health::Health, Error> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await.map_err(Into::into)?;
        let body = response.text().await.map_err(Into::into)?;
        serde_json::from_str(&body)
            .map_err(|e| Error::ParseError("Health".to_string(), body, e.to_string()))
    }

    /// Get the version of the Gotenberg server.
    pub async fn version(&self) -> Result<String, Error> {
        let url = format!("{}/version", self.base_url);
        let response = self.client.get(&url).send().await.map_err(Into::into)?;
        let body = response.text().await.map_err(Into::into)?;
        Ok(body)
    }

    /// Get the metrics of the Gotenberg server in prometheus format.
    /// The results will not be parsed and are returned as a multi-line string.
    ///
    /// By default the namespace is `gotenberg`, but this can be changed by passing `--prometheus-namespace` to the Gotenberg server on startup.
    ///
    /// - `{namespace}_chromium_requests_queue_size`    Current number of Chromium conversion requests waiting to be treated.
    /// - `{namespace}_chromium_restarts_count`	        Current number of Chromium restarts.
    /// - `{namespace}_libreoffice_requests_queue_size`	Current number of LibreOffice conversion requests waiting to be treated.
    /// - `{namespace}_libreoffice_restarts_count`	    Current number of LibreOffice restarts.
    pub async fn metrics(&self) -> Result<String, Error> {
        let url = format!("{}/prometheus/metrics", self.base_url);
        let response = self.client.get(&url).send().await.map_err(Into::into)?;
        let body = response.text().await.map_err(Into::into)?;
        Ok(body)
    }
}