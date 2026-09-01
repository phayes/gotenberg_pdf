#![allow(clippy::field_reassign_with_default)]

use super::*;
use std::time::Duration;

const HTML_CONTENT: &str = r#"
<!doctype html>
<html>
    <head><title>My PDF</title></head>
    <body><h1>Hello, PDF!</h1></body>
</html>
"#;

const DOCX_CONTENT: &[u8] = include_bytes!("../test_files/example.docx");

const PASSWORD_PROTECTED_ODT_CONTENT: &[u8] =
    include_bytes!("../test_files/example_password_protected.odt");

#[tokio::test]
async fn test_url_to_pdf() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.skip_network_idle_events = Some(false);

    // Call the API and handle the result
    match client.pdf_from_url("https://example.com", options).await {
        Ok(bytes) => {
            // Verify the response content
            assert!(!bytes.is_empty(), "PDF content should not be empty");
            println!("Received PDF content: {} bytes", bytes.len());

            // Save to local temp directory
            let temp_dir = std::env::temp_dir();
            let pdf_path = temp_dir.join("ocudigital.pdf");
            std::fs::write(&pdf_path, bytes).unwrap();
            println!("PDF saved to: {:?}", pdf_path);
        }
        Err(err) => {
            panic!("API call failed: {:?}", err);
        }
    }
}

#[tokio::test]
async fn test_web_options_trace_id() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.trace_id = Some("test-trace-id".to_string());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_single_page() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.single_page = Some(true);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_paper_size() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.paper_width = Some("210mm".parse().unwrap());
    options.paper_height = Some("297mm".parse().unwrap());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_margins() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.margin_top = Some("1in".parse().unwrap());
    options.margin_bottom = Some("1in".parse().unwrap());
    options.margin_left = Some("0.5in".parse().unwrap());
    options.margin_right = Some("0.5in".parse().unwrap());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_prefer_css_page_size() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.prefer_css_page_size = Some(true);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_print_background() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.print_background = Some(true);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_landscape() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.landscape = Some(true);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_scale() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.scale = Some(1.5);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_page_ranges() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.native_page_ranges = Some("1-3,5".parse().unwrap());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_header_footer() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.header_html = Some("<h1>Header Test: <div class='title'></div></h1>".into());
    options.footer_html = Some("Page Number: <div class='pageNumber'></div>".into());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_wait_delay() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.wait_delay = Some(Duration::from_secs(1));

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_emulated_media_type() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.emulated_media_type = Some("screen".parse().unwrap());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_fail_on_http_status_codes() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.fail_on_http_status_codes = Some(vec![404, 500]);

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_metadata() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.metadata = Some(HashMap::from([
        ("Title".to_string(), "Test Document".into()),
        ("Author".to_string(), "Test Author".into()),
    ]));

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_user_agent() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.user_agent = Some("TestUserAgent/1.0".into());

    let _pdf_bytes = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_web_options_negative_scale() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    // Negative scale should fail
    options.scale = Some(-1.0);

    let result = client.pdf_from_html(HTML_CONTENT, options).await;
    assert!(result.is_err(), "Expected negative scale to fail");
}

#[tokio::test]
async fn test_web_options_unsupported_user_agent() {
    let client = Client::new(&BASE_URL);

    let mut options = WebOptions::default();
    // Unsupported user agent format
    options.user_agent = Some("\0invalid_user_agent".into());

    let result = client.pdf_from_html(HTML_CONTENT, options).await;
    assert!(result.is_err(), "Expected unsupported user agent to fail");
}

#[tokio::test]
async fn test_screenshot_options_trace_id() {
    let client = Client::new(&BASE_URL);

    let mut options = ScreenshotOptions::default();
    options.trace_id = Some("test-trace-id".to_string());

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_width() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.width = Some(1024);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_height() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.height = Some(768);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_clip() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.clip = Some(true);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_format() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.format = Some(ImageFormat::Jpeg);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_quality() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.quality = Some(85);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_omit_background() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.omit_background = Some(true);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_optimize_for_speed() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.optimize_for_speed = Some(true);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_wait_delay() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.wait_delay = Some(Duration::from_secs(1));

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_wait_for_expression() {
    let html_content: &str = r#"
    <!doctype html>
    <html>
        <head><title>My PDF</title></head>
        <body>
            <h1>Hello, PDF!</h1>
            <script>
                setTimeout(() => {
                    window.isReady = true;
                }, 200);
            </script>
        </body>
    </html>
    "#;

    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.wait_for_expression = Some("window.isReady === true".to_string());

    let _image_bytes = client.screenshot_html(html_content, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_emulated_media_type() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.emulated_media_type = Some(MediaType::Screen);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_cookies() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.cookies = Some(vec![Cookie {
        name: "session".to_string(),
        value: "abc123".to_string(),
        domain: "example.com".to_string(),
        ..Default::default()
    }]);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_skip_network_idle_events() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.skip_network_idle_events = Some(false);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_user_agent() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.user_agent = Some("Test-Agent".to_string());

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_extra_http_headers() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.extra_http_headers = Some(
        vec![
            ("Authorization".to_string(), "Bearer token".to_string()),
            ("X-Custom-Header".to_string(), "custom-value".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_fail_on_http_status_codes() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_http_status_codes = Some(vec![404, 500]);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_fail_on_resource_http_status_codes() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_resource_http_status_codes = Some(vec![403, 502]);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_fail_on_resource_loading_failed() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_resource_loading_failed = Some(true);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_screenshot_options_fail_on_console_exceptions() {
    let client = Client::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_console_exceptions = Some(true);

    let _image_bytes = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
}

#[tokio::test]
async fn test_doc_options_trace_id() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.trace_id = Some("some-trace-id".to_string());

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}
#[tokio::test]
async fn test_doc_options_password() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.password = Some("secure-password".to_string());

    let _pdf_content = client
        .pdf_from_doc(
            "example.odt",
            PASSWORD_PROTECTED_ODT_CONTENT.to_vec(),
            options,
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_landscape() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.landscape = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_form_fields() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_form_fields = Some(false);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_allow_duplicate_field_names() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.allow_duplicate_field_names = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_bookmarks() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_bookmarks = Some(false);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_notes() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_quality() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.quality = Some(75);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_max_image_resolution() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.max_image_resolution = Some(600);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_pdfua() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.pdfua = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_native_page_ranges() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.native_page_ranges = Some("1-3,5".parse().unwrap());

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_bookmarks_to_pdf_destination() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_bookmarks_to_pdf_destination = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_placeholders() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_placeholders = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_notes_pages() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes_pages = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_only_notes_pages() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_only_notes_pages = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_notes_in_margin() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes_in_margin = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_convert_ooo_target_to_pdf_target() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.convert_ooo_target_to_pdf_target = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_links_relative_fsys() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_links_relative_fsys = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_export_hidden_slides() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_hidden_slides = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_skip_empty_pages() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.skip_empty_pages = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_add_original_document_as_stream() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.add_original_document_as_stream = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_single_page_sheets() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.single_page_sheets = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_lossless_image_compression() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.lossless_image_compression = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_reduce_image_resolution() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.reduce_image_resolution = Some(true);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_doc_options_pdfa() {
    let client = Client::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.pdfa = Some(PDFFormat::A1b);

    let _pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_pdf_metadata() {
    let client = Client::new(&BASE_URL);
    let options = DocumentOptions::default();

    // Create the PDF
    let pdf_content = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options.clone())
        .await
        .unwrap();

    // Update the metadata
    let metadata = HashMap::from([
        ("Title".to_string(), "Test Document 123".into()),
        ("Author".to_string(), "Test Author 123".into()),
    ]);

    let pdf_content = client
        .write_metadata(pdf_content.to_vec(), metadata)
        .await
        .unwrap();

    // Read the metadata
    let metadata = client.read_metadata(pdf_content.to_vec()).await.unwrap();

    assert_eq!(
        metadata.get("Title"),
        Some(&serde_json::Value::String("Test Document 123".to_string()))
    );
    assert_eq!(
        metadata.get("Author"),
        Some(&serde_json::Value::String("Test Author 123".to_string()))
    );
}

#[tokio::test]
pub async fn test_health_check() {
    let client = Client::new(&BASE_URL);
    let _health = client.health_check().await.unwrap();
}

#[tokio::test]
pub async fn test_version_string() {
    let client = Client::new(&BASE_URL);
    let version = client.version().await.unwrap();

    // It should start with 8.
    assert!(version.starts_with("8."));
}

#[tokio::test]
pub async fn test_metrics() {
    let client = Client::new(&BASE_URL);
    let _metrics = client.metrics().await.unwrap();
}
