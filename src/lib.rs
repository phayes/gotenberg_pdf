#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// Gotenberg server health status. See [`Client::health_check`].
pub mod health;

mod client;
mod page_range;
mod paper_format;

#[cfg(feature = "stream")]
mod streaming_client;

#[cfg(feature = "blocking")]
mod blocking_client;

#[cfg(feature = "stream")]
#[cfg_attr(docsrs, doc(cfg(feature = "stream")))]
pub use crate::streaming_client::StreamingClient;

#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub use crate::blocking_client::BlockingClient;

pub use crate::paper_format::*;
/// Re-exported from the `bytes` crate (See [`bytes::Bytes`]).
pub use bytes::Bytes;
pub use client::*;
pub use page_range::*;
use reqwest::multipart;
use reqwest::Error as ReqwestError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::str::FromStr;

/// Gotenberg server URL for integration tests.
/// Override with the `GOTENBERG_URL` environment variable.
#[cfg(test)]
pub(crate) static BASE_URL: std::sync::LazyLock<String> = std::sync::LazyLock::new(|| {
    std::env::var("GOTENBERG_URL").unwrap_or_else(|_| "http://localhost:3000".to_string())
});

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;

#[cfg(all(test, feature = "stream", not(target_arch = "wasm32")))]
mod streaming_tests;

#[cfg(all(test, feature = "blocking", not(target_arch = "wasm32")))]
mod blocking_tests;

#[cfg(all(test, target_arch = "wasm32"))]
mod wasm_tests;

/// Error type for the Gotenberg API.
#[derive(Debug)]
pub enum Error {
    /// Filename Error
    FilenameError(String),

    /// Error communicating with the gotenberg server.
    CommunicationError(ReqwestError),

    /// PDF rendering error.
    RenderingError(String),

    /// Error parsing a string into a type
    // (Type, Subject, Message)
    ParseError(String, String, String),
}

impl From<ReqwestError> for Error {
    fn from(error: ReqwestError) -> Self {
        Error::CommunicationError(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FilenameError(e) => write!(f, "gotenberg_pdf: Filename Error: {}", e),
            Error::CommunicationError(e) => write!(
                f,
                "gotenberg_pdf: Error communicating with the guotenberg server: {}",
                e
            ),
            Error::RenderingError(e) => {
                write!(f, "gotenberg_pdf: PDF / Image Rendering Error: {}", e)
            }
            Error::ParseError(t, s, e) => {
                write!(f, "gotenberg_pdf: Error Parsing {} from `{}`: {}", t, s, e)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::CommunicationError(e) => Some(e),
            _ => None,
        }
    }
}

/// Configuration for rendering PDF from web content using the Chromium engine.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebOptions {
    /// By default, the API assigns a unique UUID trace to every request. However, you also have the option to specify the trace for each request.
    /// This trace will show up on the end server as a `Gotenberg-Trace` header.
    pub trace_id: Option<String>,

    /// Define whether to print the entire content on one single page.
    /// Default: `false`
    pub single_page: Option<bool>,

    /// Specify paper width using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `8.5` (inches)
    pub paper_width: Option<LinearDimention>,

    /// Specify paper height using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `11` (inches)
    pub paper_height: Option<LinearDimention>,

    /// Specify top margin width using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `0.39` (inches)
    pub margin_top: Option<LinearDimention>,

    /// Specify bottom margin width using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `0.39` (inches)
    pub margin_bottom: Option<LinearDimention>,

    /// Specify left margin width using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `0.39` (inches)
    pub margin_left: Option<LinearDimention>,

    /// Specify right margin width using units like 72pt, 96px, 1in, 25.4mm, 2.54cm, or 6pc.
    /// Default: `0.39` (inches)
    pub margin_right: Option<LinearDimention>,

    /// Define whether to prefer page size as defined by CSS.
    /// Default: `false`
    pub prefer_css_page_size: Option<bool>,

    /// Define whether the document outline should be embedded into the PDF.
    /// Default: `false`
    pub generate_document_outline: Option<bool>,

    /// Print the background graphics.
    /// Default: `false`
    pub print_background: Option<bool>,

    /// Hide the default white background and allow generating PDFs with transparency.
    /// Default: `false`
    pub omit_background: Option<bool>,

    /// Set the page orientation to landscape.
    /// Default: `false`
    pub landscape: Option<bool>,

    /// The scale of the page rendering.
    /// Default: `1.0`
    pub scale: Option<f64>,

    /// Page ranges to print, e.g., '1-5, 8, 11-13' - empty means all pages.
    /// Default: `All pages`
    pub native_page_ranges: Option<PageRange>,

    /// HTML content containing the header.
    ///
    /// The following classes allow you to inject printing values into the header:
    ///   date - formatted print date.
    ///   title - document title.
    ///   url - document location.
    ///   pageNumber - current page number.
    ///   totalPages - total pages in the document.
    ///
    /// Caveats: No JavaScript or external resources.
    pub header_html: Option<String>,

    /// HTML content containing the footer.
    ///
    /// The following classes allow you to inject printing values into the footer:
    ///   date - formatted print date.
    ///   title - document title.
    ///   url - document location.
    ///   pageNumber - current page number.
    ///   totalPages - total pages in the document.
    ///
    /// Caveats: No JavaScript or external resources.
    pub footer_html: Option<String>,

    /// Duration to wait when loading an HTML document before converting it into PDF.
    pub wait_delay: Option<std::time::Duration>,

    /// The JavaScript expression to wait before converting an HTML document into PDF until it returns true.
    ///
    /// For example:
    ///    ```text
    ///    # Somewhere in the HTML document.
    ///    var globalVar = 'notReady'
    ///    await promises()
    ///    window.globalVar = 'ready'
    ///    ```
    ///
    ///    ```text
    ///    request_options.wait_until = Some("window.globalVar === 'ready'".to_string());
    ///    ```
    pub wait_for_expression: Option<String>,

    /// The media type to emulate, either "screen" or "print". Default: "print".
    pub emulated_media_type: Option<MediaType>,

    /// Cookies to store in the Chromium cookie jar
    pub cookies: Option<Vec<Cookie>>,

    /// Do not wait for Chromium network to be idle. Default: true.
    ///
    /// If you are having problems where the page is not fully rendered, try setting this to false.
    pub skip_network_idle_events: Option<bool>,

    /// Override the default User-Agent HTTP header.
    pub user_agent: Option<String>,

    /// Extra HTTP headers to send by Chromium.
    pub extra_http_headers: Option<HashMap<String, String>>,

    /// Convert the resulting PDF into the given PDF/A format
    pub pdfa: Option<PDFFormat>,

    /// Enable PDF for Universal Access for optimal accessibility.
    pub pdfua: Option<bool>,

    /// Write PDF metadata.
    /// Not all metadata are writable. Consider taking a look at <https://exiftool.org/TagNames/XMP.html#pdf> for an (exhaustive?) list of available metadata.
    /// Caution: Writing metadata may compromise PDF/A compliance.
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// Fail on these HTTP status codes.
    /// Fail a response if the HTTP status code from the main page is not acceptable.
    /// An X99 entry means every HTTP status codes between X00 and X99 (e.g., 499 means every HTTP status codes between 400 and 499).
    /// Default: `[499,599]` (all 4XX and 5XX status codes)
    pub fail_on_http_status_codes: Option<Vec<u32>>,

    /// Fail on these HTTP status codes on resources.
    /// Fail a response if any of the resources loaded in the page have a status code that is not acceptable.
    /// An X99 entry means every HTTP status codes between X00 and X99 (e.g., 499 means every HTTP status codes between 400 and 499).
    /// Default: None
    pub fail_on_resource_http_status_codes: Option<Vec<u32>>,

    /// Fail a response if Chromium fails to load at least one resource. Default: `false`.
    pub fail_on_resource_loading_failed: Option<bool>,

    /// Fail a response if there are exceptions in the Chromium console.
    pub fail_on_console_exceptions: Option<bool>,
}

impl WebOptions {
    /// Set the paper format. If a custom paper size is needed, set the `paper_width` and `paper_height` fields manually.
    pub fn set_paper_format(&mut self, format: PaperFormat) {
        self.paper_width = Some(format.width());
        self.paper_height = Some(format.height());
    }

    fn fill_form(self, form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        let mut form = form;

        if let Some(single_page) = self.single_page {
            form = form.text("singlePage", single_page.to_string());
        }

        if let Some(paper_width) = self.paper_width {
            form = form.text("paperWidth", format!("{}", paper_width));
        }

        if let Some(paper_height) = self.paper_height {
            form = form.text("paperHeight", format!("{}", paper_height));
        }

        if let Some(margin_top) = self.margin_top {
            form = form.text("marginTop", margin_top.to_string());
        }

        if let Some(margin_bottom) = self.margin_bottom {
            form = form.text("marginBottom", margin_bottom.to_string());
        }

        if let Some(margin_left) = self.margin_left {
            form = form.text("marginLeft", margin_left.to_string());
        }

        if let Some(margin_right) = self.margin_right {
            form = form.text("marginRight", margin_right.to_string());
        }

        if let Some(prefer_css_page_size) = self.prefer_css_page_size {
            form = form.text("preferCssPageSize", prefer_css_page_size.to_string());
        }

        if let Some(generate_document_outline) = self.generate_document_outline {
            form = form.text(
                "generateDocumentOutline",
                generate_document_outline.to_string(),
            );
        }

        if let Some(print_background) = self.print_background {
            form = form.text("printBackground", print_background.to_string());
        }

        if let Some(omit_background) = self.omit_background {
            form = form.text("omitBackground", omit_background.to_string());
        }

        if let Some(landscape) = self.landscape {
            form = form.text("landscape", landscape.to_string());
        }

        if let Some(scale) = self.scale {
            form = form.text("scale", scale.to_string());
        }

        if let Some(native_page_ranges) = self.native_page_ranges {
            form = form.text("nativePageRanges", native_page_ranges.to_string());
        }

        if let Some(header_html) = self.header_html {
            let file_bytes = header_html.into_bytes();
            let part = multipart::Part::bytes(file_bytes)
                .file_name("header.html")
                .mime_str("text/html")
                .unwrap();
            form = form.part("header.html", part);
        }

        if let Some(footer_html) = self.footer_html {
            let file_bytes = footer_html.into_bytes();
            let part = multipart::Part::bytes(file_bytes)
                .file_name("footer.html")
                .mime_str("text/html")
                .unwrap();
            form = form.part("footer.html", part);
        }

        if let Some(wait_delay) = self.wait_delay {
            form = form.text("waitDelay", format!("{}ms", wait_delay.as_millis()));
        }

        if let Some(wait_for_expression) = self.wait_for_expression {
            form = form.text("waitForExpression", wait_for_expression);
        }

        if let Some(emulated_media_type) = self.emulated_media_type {
            form = form.text("emulatedMediaType", emulated_media_type.to_string());
        }

        if let Some(cookies) = self.cookies {
            form = form.text("cookies", serde_json::to_string(&cookies).unwrap());
        }

        if let Some(skip_network_idle_events) = self.skip_network_idle_events {
            form = form.text(
                "skipNetworkIdleEvents",
                skip_network_idle_events.to_string(),
            );
        }

        if let Some(user_agent) = self.user_agent {
            form = form.text("userAgent", user_agent);
        }

        if let Some(extra_http_headers) = self.extra_http_headers {
            form = form.text(
                "extraHttpHeaders",
                serde_json::to_string(&extra_http_headers).unwrap(),
            );
        }

        if let Some(pdfa) = self.pdfa {
            form = form.text("pdfa", pdfa.to_string());
        }

        if let Some(pdfua) = self.pdfua {
            form = form.text("pdfua", pdfua.to_string());
        }

        if let Some(metadata) = self.metadata {
            form = form.text("metadata", serde_json::to_string(&metadata).unwrap());
        }

        if let Some(fail_on_http_status_codes) = self.fail_on_http_status_codes {
            form = form.text(
                "failOnHttpStatusCodes",
                serde_json::to_string(&fail_on_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_http_status_codes) = self.fail_on_resource_http_status_codes {
            form = form.text(
                "failOnResourceHttpStatusCodes",
                serde_json::to_string(&fail_on_resource_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_loading_failed) = self.fail_on_resource_loading_failed {
            form = form.text(
                "failOnResourceLoadingFailed",
                fail_on_resource_loading_failed.to_string(),
            );
        }

        if let Some(fail_on_console_exceptions) = self.fail_on_console_exceptions {
            form = form.text(
                "failOnConsoleExceptions",
                fail_on_console_exceptions.to_string(),
            );
        }

        form
    }

    #[cfg(feature = "blocking")]
    fn fill_form_blocking(
        self,
        form: reqwest::blocking::multipart::Form,
    ) -> reqwest::blocking::multipart::Form {
        let mut form = form;

        if let Some(single_page) = self.single_page {
            form = form.text("singlePage", single_page.to_string());
        }

        if let Some(paper_width) = self.paper_width {
            form = form.text("paperWidth", format!("{}", paper_width));
        }

        if let Some(paper_height) = self.paper_height {
            form = form.text("paperHeight", format!("{}", paper_height));
        }

        if let Some(margin_top) = self.margin_top {
            form = form.text("marginTop", margin_top.to_string());
        }

        if let Some(margin_bottom) = self.margin_bottom {
            form = form.text("marginBottom", margin_bottom.to_string());
        }

        if let Some(margin_left) = self.margin_left {
            form = form.text("marginLeft", margin_left.to_string());
        }

        if let Some(margin_right) = self.margin_right {
            form = form.text("marginRight", margin_right.to_string());
        }

        if let Some(prefer_css_page_size) = self.prefer_css_page_size {
            form = form.text("preferCssPageSize", prefer_css_page_size.to_string());
        }

        if let Some(generate_document_outline) = self.generate_document_outline {
            form = form.text(
                "generateDocumentOutline",
                generate_document_outline.to_string(),
            );
        }

        if let Some(print_background) = self.print_background {
            form = form.text("printBackground", print_background.to_string());
        }

        if let Some(omit_background) = self.omit_background {
            form = form.text("omitBackground", omit_background.to_string());
        }

        if let Some(landscape) = self.landscape {
            form = form.text("landscape", landscape.to_string());
        }

        if let Some(scale) = self.scale {
            form = form.text("scale", scale.to_string());
        }

        if let Some(native_page_ranges) = self.native_page_ranges {
            form = form.text("nativePageRanges", native_page_ranges.to_string());
        }

        if let Some(header_html) = self.header_html {
            let file_bytes = header_html.into_bytes();
            let part = reqwest::blocking::multipart::Part::bytes(file_bytes)
                .file_name("header.html")
                .mime_str("text/html")
                .unwrap();
            form = form.part("header.html", part);
        }

        if let Some(footer_html) = self.footer_html {
            let file_bytes = footer_html.into_bytes();
            let part = reqwest::blocking::multipart::Part::bytes(file_bytes)
                .file_name("footer.html")
                .mime_str("text/html")
                .unwrap();
            form = form.part("footer.html", part);
        }

        if let Some(wait_delay) = self.wait_delay {
            form = form.text("waitDelay", format!("{}ms", wait_delay.as_millis()));
        }

        if let Some(wait_for_expression) = self.wait_for_expression {
            form = form.text("waitForExpression", wait_for_expression);
        }

        if let Some(emulated_media_type) = self.emulated_media_type {
            form = form.text("emulatedMediaType", emulated_media_type.to_string());
        }

        if let Some(cookies) = self.cookies {
            form = form.text("cookies", serde_json::to_string(&cookies).unwrap());
        }

        if let Some(skip_network_idle_events) = self.skip_network_idle_events {
            form = form.text(
                "skipNetworkIdleEvents",
                skip_network_idle_events.to_string(),
            );
        }

        if let Some(user_agent) = self.user_agent {
            form = form.text("userAgent", user_agent);
        }

        if let Some(extra_http_headers) = self.extra_http_headers {
            form = form.text(
                "extraHttpHeaders",
                serde_json::to_string(&extra_http_headers).unwrap(),
            );
        }

        if let Some(pdfa) = self.pdfa {
            form = form.text("pdfa", pdfa.to_string());
        }

        if let Some(pdfua) = self.pdfua {
            form = form.text("pdfua", pdfua.to_string());
        }

        if let Some(metadata) = self.metadata {
            form = form.text("metadata", serde_json::to_string(&metadata).unwrap());
        }

        if let Some(fail_on_http_status_codes) = self.fail_on_http_status_codes {
            form = form.text(
                "failOnHttpStatusCodes",
                serde_json::to_string(&fail_on_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_http_status_codes) = self.fail_on_resource_http_status_codes {
            form = form.text(
                "failOnResourceHttpStatusCodes",
                serde_json::to_string(&fail_on_resource_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_loading_failed) = self.fail_on_resource_loading_failed {
            form = form.text(
                "failOnResourceLoadingFailed",
                fail_on_resource_loading_failed.to_string(),
            );
        }

        if let Some(fail_on_console_exceptions) = self.fail_on_console_exceptions {
            form = form.text(
                "failOnConsoleExceptions",
                fail_on_console_exceptions.to_string(),
            );
        }

        form
    }
}

/// Options for taking a screenshot of a webpage.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScreenshotOptions {
    /// By default, the API assigns a unique UUID trace to every request. However, you also have the option to specify the trace for each request.
    /// This trace will show up on the end server as a `Gotenberg-Trace` header.
    pub trace_id: Option<String>,

    /// The device screen width in pixels. Default: 800.
    pub width: Option<u32>,

    /// The device screen height in pixels. Default: 600.
    pub height: Option<u32>,

    /// Define whether to clip the screenshot according to the device dimensions. Default: false.
    pub clip: Option<bool>,

    /// The image format, either "png", "jpeg" or "webp". Default: png.
    pub format: Option<ImageFormat>,

    /// The compression quality from range 0 to 100 (jpeg only). Default: 100.
    pub quality: Option<u8>,

    /// Hide the default white background and allow generating screenshots with transparency. Default: false.
    pub omit_background: Option<bool>,

    /// Define whether to optimize image encoding for speed, not for resulting size. Default: false.
    pub optimize_for_speed: Option<bool>,

    /// Duration to wait when loading an HTML document before converting it into PDF.
    pub wait_delay: Option<std::time::Duration>,

    /// The JavaScript expression to wait before converting an HTML document into PDF until it returns true.
    ///
    /// For example:
    ///    ```text
    ///    # Somewhere in the HTML document.
    ///    var globalVar = 'notReady'
    ///    await promises()
    ///    window.globalVar = 'ready'
    ///    ```
    ///
    ///    ```text
    ///    request_options.wait_until = Some("window.globalVar === 'ready'".to_string());
    ///    ```
    pub wait_for_expression: Option<String>,

    /// The media type to emulate, either "screen" or "print". Default: "print".
    pub emulated_media_type: Option<MediaType>,

    /// Cookies to store in the Chromium cookie jar
    pub cookies: Option<Vec<Cookie>>,

    /// Do not wait for Chromium network to be idle. Default: true.
    ///
    /// If you are having problems where the page is not fully rendered, try setting this to false.
    pub skip_network_idle_events: Option<bool>,

    /// Override the default User-Agent HTTP header.
    pub user_agent: Option<String>,

    /// Extra HTTP headers to send by Chromium.
    pub extra_http_headers: Option<HashMap<String, String>>,

    /// Fail on these HTTP status codes.
    /// Fail a response if the HTTP status code from the main page is not acceptable.
    /// An X99 entry means every HTTP status codes between X00 and X99 (e.g., 499 means every HTTP status codes between 400 and 499).
    /// Default: `[499,599]` (all 4XX and 5XX status codes)
    pub fail_on_http_status_codes: Option<Vec<u32>>,

    /// Fail on these HTTP status codes on resources.
    /// Fail a response if any of the resources loaded in the page have a status code that is not acceptable.
    /// An X99 entry means every HTTP status codes between X00 and X99 (e.g., 499 means every HTTP status codes between 400 and 499).
    /// Default: None
    pub fail_on_resource_http_status_codes: Option<Vec<u32>>,

    /// Fail a response if Chromium fails to load at least one resource. Default: `false`.
    pub fail_on_resource_loading_failed: Option<bool>,

    /// Fail a response if there are exceptions in the Chromium console.
    pub fail_on_console_exceptions: Option<bool>,
}

impl ScreenshotOptions {
    fn fill_form(self, form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        let mut form = form;

        if let Some(width) = self.width {
            form = form.text("width", width.to_string());
        }

        if let Some(height) = self.height {
            form = form.text("height", height.to_string());
        }

        if let Some(clip) = self.clip {
            form = form.text("clip", clip.to_string());
        }

        if let Some(format) = self.format {
            form = form.text("format", format.to_string());
        }

        if let Some(quality) = self.quality {
            form = form.text("quality", quality.to_string());
        }

        if let Some(omit_background) = self.omit_background {
            form = form.text("omitBackground", omit_background.to_string());
        }

        if let Some(optimize_for_speed) = self.optimize_for_speed {
            form = form.text("optimizeForSpeed", optimize_for_speed.to_string());
        }

        if let Some(wait_delay) = self.wait_delay {
            form = form.text("waitDelay", format!("{}ms", wait_delay.as_millis()));
        }

        if let Some(wait_for_expression) = self.wait_for_expression {
            form = form.text("waitForExpression", wait_for_expression);
        }

        if let Some(emulated_media_type) = self.emulated_media_type {
            form = form.text("emulatedMediaType", emulated_media_type.to_string());
        }

        if let Some(cookies) = self.cookies {
            form = form.text("cookies", serde_json::to_string(&cookies).unwrap());
        }

        if let Some(skip_network_idle_events) = self.skip_network_idle_events {
            form = form.text(
                "skipNetworkIdleEvents",
                skip_network_idle_events.to_string(),
            );
        }

        if let Some(user_agent) = self.user_agent {
            form = form.text("userAgent", user_agent);
        }

        if let Some(extra_http_headers) = self.extra_http_headers {
            form = form.text(
                "extraHttpHeaders",
                serde_json::to_string(&extra_http_headers).unwrap(),
            );
        }

        if let Some(fail_on_http_status_codes) = self.fail_on_http_status_codes {
            form = form.text(
                "failOnHttpStatusCodes",
                serde_json::to_string(&fail_on_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_http_status_codes) = self.fail_on_resource_http_status_codes {
            form = form.text(
                "failOnResourceHttpStatusCodes",
                serde_json::to_string(&fail_on_resource_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_loading_failed) = self.fail_on_resource_loading_failed {
            form = form.text(
                "failOnResourceLoadingFailed",
                fail_on_resource_loading_failed.to_string(),
            );
        }

        if let Some(fail_on_console_exceptions) = self.fail_on_console_exceptions {
            form = form.text(
                "failOnConsoleExceptions",
                fail_on_console_exceptions.to_string(),
            );
        }

        form
    }

    #[cfg(feature = "blocking")]
    fn fill_form_blocking(
        self,
        form: reqwest::blocking::multipart::Form,
    ) -> reqwest::blocking::multipart::Form {
        let mut form = form;

        if let Some(width) = self.width {
            form = form.text("width", width.to_string());
        }

        if let Some(height) = self.height {
            form = form.text("height", height.to_string());
        }

        if let Some(clip) = self.clip {
            form = form.text("clip", clip.to_string());
        }

        if let Some(format) = self.format {
            form = form.text("format", format.to_string());
        }

        if let Some(quality) = self.quality {
            form = form.text("quality", quality.to_string());
        }

        if let Some(omit_background) = self.omit_background {
            form = form.text("omitBackground", omit_background.to_string());
        }

        if let Some(optimize_for_speed) = self.optimize_for_speed {
            form = form.text("optimizeForSpeed", optimize_for_speed.to_string());
        }

        if let Some(wait_delay) = self.wait_delay {
            form = form.text("waitDelay", format!("{}ms", wait_delay.as_millis()));
        }

        if let Some(wait_for_expression) = self.wait_for_expression {
            form = form.text("waitForExpression", wait_for_expression);
        }

        if let Some(emulated_media_type) = self.emulated_media_type {
            form = form.text("emulatedMediaType", emulated_media_type.to_string());
        }

        if let Some(cookies) = self.cookies {
            form = form.text("cookies", serde_json::to_string(&cookies).unwrap());
        }

        if let Some(skip_network_idle_events) = self.skip_network_idle_events {
            form = form.text(
                "skipNetworkIdleEvents",
                skip_network_idle_events.to_string(),
            );
        }

        if let Some(user_agent) = self.user_agent {
            form = form.text("userAgent", user_agent);
        }

        if let Some(extra_http_headers) = self.extra_http_headers {
            form = form.text(
                "extraHttpHeaders",
                serde_json::to_string(&extra_http_headers).unwrap(),
            );
        }

        if let Some(fail_on_http_status_codes) = self.fail_on_http_status_codes {
            form = form.text(
                "failOnHttpStatusCodes",
                serde_json::to_string(&fail_on_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_http_status_codes) = self.fail_on_resource_http_status_codes {
            form = form.text(
                "failOnResourceHttpStatusCodes",
                serde_json::to_string(&fail_on_resource_http_status_codes).unwrap(),
            );
        }

        if let Some(fail_on_resource_loading_failed) = self.fail_on_resource_loading_failed {
            form = form.text(
                "failOnResourceLoadingFailed",
                fail_on_resource_loading_failed.to_string(),
            );
        }

        if let Some(fail_on_console_exceptions) = self.fail_on_console_exceptions {
            form = form.text(
                "failOnConsoleExceptions",
                fail_on_console_exceptions.to_string(),
            );
        }

        form
    }
}

/// Options for converting a document to a PDF using the LibreOffice engine.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentOptions {
    /// By default, the API assigns a unique UUID trace to every request. However, you also have the option to specify the trace for each request.
    /// This trace will show up on the end server as a `Gotenberg-Trace` header.
    pub trace_id: Option<String>,

    /// Set the password for opening the source file.
    pub password: Option<String>,

    /// Set the paper orientation to landscape. default: false
    pub landscape: Option<bool>,

    /// Page ranges to print, e.g., '1-4' - empty means all pages. default: All pages
    pub native_page_ranges: Option<PageRange>,

    /// Specify whether form fields are exported as widgets or only their fixed print representation is exported. default: true
    pub export_form_fields: Option<bool>,

    /// Specify whether multiple form fields exported are allowed to have the same field name. default: false
    pub allow_duplicate_field_names: Option<bool>,

    /// Specify if bookmarks are exported to PDF. default: true
    pub export_bookmarks: Option<bool>,

    /// Specify that the bookmarks contained in the source LibreOffice file should be exported to the PDF file as Named Destination. default: false
    pub export_bookmarks_to_pdf_destination: Option<bool>,

    /// Export the placeholders fields visual markings only. The exported placeholder is ineffective. default: false
    pub export_placeholders: Option<bool>,

    /// Specify if notes are exported to PDF. default: false
    pub export_notes: Option<bool>,

    /// Specify if notes pages are exported to PDF. Notes pages are available in Impress documents only. default: false
    pub export_notes_pages: Option<bool>,

    /// Specify, if the form field exportNotesPages is set to true, if only notes pages are exported to PDF. default: false
    pub export_only_notes_pages: Option<bool>,

    /// Specify if notes in margin are exported to PDF. default: false
    pub export_notes_in_margin: Option<bool>,

    /// Specify that the target documents with `.od[tpgs]` extension, will have that extension changed to .pdf when the link is exported to PDF. The source document remains untouched. default: false
    pub convert_ooo_target_to_pdf_target: Option<bool>,

    /// Specify that the file system related hyperlinks (file:// protocol) present in the document will be exported as relative to the source document location. default: false
    pub export_links_relative_fsys: Option<bool>,

    /// Export, for LibreOffice Impress, slides that are not included in slide shows. default: false
    pub export_hidden_slides: Option<bool>,

    /// Specify that automatically inserted empty pages are suppressed. This option is active only if storing Writer documents. default: false
    pub skip_empty_pages: Option<bool>,

    /// Specify that a stream is inserted to the PDF file which contains the original document for archiving purposes. default: false
    pub add_original_document_as_stream: Option<bool>,

    /// Ignore each sheet’s paper size, print ranges and shown/hidden status and puts every sheet (even hidden sheets) on exactly one page. default: false
    pub single_page_sheets: Option<bool>,

    /// Specify if images are exported to PDF using a lossless compression format like PNG or compressed using the JPEG format. default: false
    pub lossless_image_compression: Option<bool>,

    /// Specify the quality of the JPG export. A higher value produces a higher-quality image and a larger file. Between 1 and 100. default: 90
    pub quality: Option<u8>,

    /// Specify if the resolution of each image is reduced to the resolution specified by the form field maxImageResolution. default: false
    pub reduce_image_resolution: Option<bool>,

    /// If the form field reduceImageResolution is set to true, tell if all images will be reduced to the given value in DPI. Possible values are: 75, 150, 300, 600 and 1200. default: 300
    pub max_image_resolution: Option<u32>,

    /// Convert the resulting PDF into the given PDF/A format
    pub pdfa: Option<PDFFormat>,

    /// Enable PDF for Universal Access for optimal accessibility.
    pub pdfua: Option<bool>,
}

/// Options for converting a document to a PDF using the LibreOffice engine.
impl DocumentOptions {
    fn fill_form(self, form: reqwest::multipart::Form) -> reqwest::multipart::Form {
        let mut form = form;

        if let Some(password) = self.password {
            form = form.text("password", password);
        }

        if let Some(landscape) = self.landscape {
            form = form.text("landscape", landscape.to_string());
        }

        if let Some(native_page_ranges) = self.native_page_ranges {
            form = form.text("nativePageRanges", native_page_ranges.to_string());
        }

        if let Some(export_form_fields) = self.export_form_fields {
            form = form.text("exportFormFields", export_form_fields.to_string());
        }

        if let Some(allow_duplicate_field_names) = self.allow_duplicate_field_names {
            form = form.text(
                "allowDuplicateFieldNames",
                allow_duplicate_field_names.to_string(),
            );
        }

        if let Some(export_bookmarks) = self.export_bookmarks {
            form = form.text("exportBookmarks", export_bookmarks.to_string());
        }

        if let Some(export_bookmarks_to_pdf_destination) = self.export_bookmarks_to_pdf_destination
        {
            form = form.text(
                "exportBookmarksToPdfDestination",
                export_bookmarks_to_pdf_destination.to_string(),
            );
        }

        if let Some(export_placeholders) = self.export_placeholders {
            form = form.text("exportPlaceholders", export_placeholders.to_string());
        }

        if let Some(export_notes) = self.export_notes {
            form = form.text("exportNotes", export_notes.to_string());
        }

        if let Some(export_notes_pages) = self.export_notes_pages {
            form = form.text("exportNotesPages", export_notes_pages.to_string());
        }

        if let Some(export_only_notes_pages) = self.export_only_notes_pages {
            form = form.text("exportOnlyNotesPages", export_only_notes_pages.to_string());
        }

        if let Some(export_notes_in_margin) = self.export_notes_in_margin {
            form = form.text("exportNotesInMargin", export_notes_in_margin.to_string());
        }

        if let Some(convert_ooo_target_to_pdf_target) = self.convert_ooo_target_to_pdf_target {
            form = form.text(
                "convertOooTargetToPdfTarget",
                convert_ooo_target_to_pdf_target.to_string(),
            );
        }

        if let Some(export_links_relative_fsys) = self.export_links_relative_fsys {
            form = form.text(
                "exportLinksRelativeFsys",
                export_links_relative_fsys.to_string(),
            );
        }

        if let Some(export_hidden_slides) = self.export_hidden_slides {
            form = form.text("exportHiddenSlides", export_hidden_slides.to_string());
        }

        if let Some(skip_empty_pages) = self.skip_empty_pages {
            form = form.text("skipEmptyPages", skip_empty_pages.to_string());
        }

        if let Some(add_original_document_as_stream) = self.add_original_document_as_stream {
            form = form.text(
                "addOriginalDocumentAsStream",
                add_original_document_as_stream.to_string(),
            );
        }

        if let Some(single_page_sheets) = self.single_page_sheets {
            form = form.text("singlePageSheets", single_page_sheets.to_string());
        }

        if let Some(lossless_image_compression) = self.lossless_image_compression {
            form = form.text(
                "losslessImageCompression",
                lossless_image_compression.to_string(),
            );
        }

        if let Some(quality) = self.quality {
            form = form.text("quality", quality.to_string());
        }

        if let Some(reduce_image_resolution) = self.reduce_image_resolution {
            form = form.text("reduceImageResolution", reduce_image_resolution.to_string());
        }

        if let Some(max_image_resolution) = self.max_image_resolution {
            form = form.text("maxImageResolution", max_image_resolution.to_string());
        }

        if let Some(pdfa) = self.pdfa {
            form = form.text("pdfa", pdfa.to_string());
        }

        if let Some(pdfua) = self.pdfua {
            form = form.text("pdfua", pdfua.to_string());
        }

        form
    }

    #[cfg(feature = "blocking")]
    fn fill_form_blocking(
        self,
        form: reqwest::blocking::multipart::Form,
    ) -> reqwest::blocking::multipart::Form {
        let mut form = form;

        if let Some(password) = self.password {
            form = form.text("password", password);
        }

        if let Some(landscape) = self.landscape {
            form = form.text("landscape", landscape.to_string());
        }

        if let Some(native_page_ranges) = self.native_page_ranges {
            form = form.text("nativePageRanges", native_page_ranges.to_string());
        }

        if let Some(export_form_fields) = self.export_form_fields {
            form = form.text("exportFormFields", export_form_fields.to_string());
        }

        if let Some(allow_duplicate_field_names) = self.allow_duplicate_field_names {
            form = form.text(
                "allowDuplicateFieldNames",
                allow_duplicate_field_names.to_string(),
            );
        }

        if let Some(export_bookmarks) = self.export_bookmarks {
            form = form.text("exportBookmarks", export_bookmarks.to_string());
        }

        if let Some(export_bookmarks_to_pdf_destination) = self.export_bookmarks_to_pdf_destination
        {
            form = form.text(
                "exportBookmarksToPdfDestination",
                export_bookmarks_to_pdf_destination.to_string(),
            );
        }

        if let Some(export_placeholders) = self.export_placeholders {
            form = form.text("exportPlaceholders", export_placeholders.to_string());
        }

        if let Some(export_notes) = self.export_notes {
            form = form.text("exportNotes", export_notes.to_string());
        }

        if let Some(export_notes_pages) = self.export_notes_pages {
            form = form.text("exportNotesPages", export_notes_pages.to_string());
        }

        if let Some(export_only_notes_pages) = self.export_only_notes_pages {
            form = form.text("exportOnlyNotesPages", export_only_notes_pages.to_string());
        }

        if let Some(export_notes_in_margin) = self.export_notes_in_margin {
            form = form.text("exportNotesInMargin", export_notes_in_margin.to_string());
        }

        if let Some(convert_ooo_target_to_pdf_target) = self.convert_ooo_target_to_pdf_target {
            form = form.text(
                "convertOooTargetToPdfTarget",
                convert_ooo_target_to_pdf_target.to_string(),
            );
        }

        if let Some(export_links_relative_fsys) = self.export_links_relative_fsys {
            form = form.text(
                "exportLinksRelativeFsys",
                export_links_relative_fsys.to_string(),
            );
        }

        if let Some(export_hidden_slides) = self.export_hidden_slides {
            form = form.text("exportHiddenSlides", export_hidden_slides.to_string());
        }

        if let Some(skip_empty_pages) = self.skip_empty_pages {
            form = form.text("skipEmptyPages", skip_empty_pages.to_string());
        }

        if let Some(add_original_document_as_stream) = self.add_original_document_as_stream {
            form = form.text(
                "addOriginalDocumentAsStream",
                add_original_document_as_stream.to_string(),
            );
        }

        if let Some(single_page_sheets) = self.single_page_sheets {
            form = form.text("singlePageSheets", single_page_sheets.to_string());
        }

        if let Some(lossless_image_compression) = self.lossless_image_compression {
            form = form.text(
                "losslessImageCompression",
                lossless_image_compression.to_string(),
            );
        }

        if let Some(quality) = self.quality {
            form = form.text("quality", quality.to_string());
        }

        if let Some(reduce_image_resolution) = self.reduce_image_resolution {
            form = form.text("reduceImageResolution", reduce_image_resolution.to_string());
        }

        if let Some(max_image_resolution) = self.max_image_resolution {
            form = form.text("maxImageResolution", max_image_resolution.to_string());
        }

        if let Some(pdfa) = self.pdfa {
            form = form.text("pdfa", pdfa.to_string());
        }

        if let Some(pdfua) = self.pdfua {
            form = form.text("pdfua", pdfua.to_string());
        }

        form
    }
}

/// Cookie to send to the end server.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    /// Cookie name.
    pub name: String,

    /// Cookie value.
    pub value: String,

    /// Cookie domain.
    pub domain: String,

    /// Cookie path.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Set the cookie to secure if true.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,

    /// Set the cookie as HTTP-only if true.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_only: Option<bool>,

    /// The [`SameSite`] cookie attribute.
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub same_site: Option<SameSite>,
}

impl Cookie {
    pub fn new(name: &str, value: &str, domain: &str) -> Self {
        Cookie {
            name: name.to_string(),
            value: value.to_string(),
            domain: domain.to_string(),
            ..Default::default()
        }
    }
}

/// The `SameSite` cookie attribute.
///
/// A cookie with a SameSite attribute is imposed restrictions on when it is sent to the origin server in a cross-site request.
/// If the SameSite attribute is “Strict”, then the cookie is never sent in cross-site requests.
/// If the SameSite attribute is “Lax”, the cookie is only sent in cross-site requests with “safe” HTTP methods, i.e, GET, HEAD, OPTIONS, TRACE.
/// If the SameSite attribute is “None”, the cookie is sent in all cross-site requests if the “Secure” flag is also set, otherwise the cookie is ignored.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum SameSite {
    Strict,
    Lax,
    None,
}

/// Supported PDF binary formats.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PDFFormat {
    /// PDF/A-1: (ISO 19005-1:2005)
    #[serde(rename = "PDF/A-1b")]
    A1b,

    /// PDF/A-2: (ISO 19005-2:2011)
    #[serde(rename = "PDF/A-2b")]
    A2b,

    /// PDF/A-3 (ISO 19005-3:2012)
    #[serde(rename = "PDF/A-3b")]
    A3b,
}

impl fmt::Display for PDFFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PDFFormat::A1b => write!(f, "PDF/A-1b"),
            PDFFormat::A2b => write!(f, "PDF/A-2b"),
            PDFFormat::A3b => write!(f, "PDF/A-3b"),
        }
    }
}

impl FromStr for PDFFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PDF/A-1b" => Ok(PDFFormat::A1b),
            "PDF/A-2b" => Ok(PDFFormat::A2b),
            "PDF/A-3b" => Ok(PDFFormat::A3b),
            _ => Err(Error::ParseError(
                "PDFFormat".to_string(),
                s.to_string(),
                "Invalid PDF format".to_string(),
            )),
        }
    }
}

/// Image format to use when taking a screenshot.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ImageFormat {
    /// Portable Network Graphics (PNG)
    #[serde(rename = "png")]
    Png,

    /// JPEG Image, best for photographs.
    #[serde(rename = "jpeg")]
    Jpeg,

    /// WebP Image, best quality and compression, but not as widely supported.
    #[serde(rename = "webp")]
    Webp,
}

impl fmt::Display for ImageFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageFormat::Png => write!(f, "png"),
            ImageFormat::Jpeg => write!(f, "jpeg"),
            ImageFormat::Webp => write!(f, "webp"),
        }
    }
}

impl FromStr for ImageFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "png" => Ok(ImageFormat::Png),
            "jpeg" => Ok(ImageFormat::Jpeg),
            "webp" => Ok(ImageFormat::Webp),
            _ => Err(Error::ParseError(
                "ImageFormat".to_string(),
                s.to_string(),
                "Invalid image format".to_string(),
            )),
        }
    }
}

/// Media type, either "print" or "screen".
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MediaType {
    #[serde(rename = "screen")]
    /// Screen media type.
    Screen,

    #[serde(rename = "print")]
    /// Print media type.
    Print,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::Screen => write!(f, "screen"),
            MediaType::Print => write!(f, "print"),
        }
    }
}

impl FromStr for MediaType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "screen" => Ok(MediaType::Screen),
            "print" => Ok(MediaType::Print),
            _ => Err(Error::ParseError(
                "MediaType".to_string(),
                s.to_string(),
                "Invalid media type".to_string(),
            )),
        }
    }
}
