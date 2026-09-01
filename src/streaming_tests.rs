#![allow(clippy::field_reassign_with_default)]

use super::*;
use futures::StreamExt; // For stream.next()
use std::collections::HashMap;
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

// Helper function to consume a stream into memory
async fn collect_stream(
    mut stream: impl futures::Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
) -> Vec<u8> {
    let mut data = Vec::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.unwrap();
        data.extend_from_slice(&chunk);
    }
    data
}

#[tokio::test]
async fn test_url_to_pdf_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.skip_network_idle_events = Some(false);

    let stream = client
        .pdf_from_url("https://example.com", options)
        .await
        .unwrap();
    let data = collect_stream(stream).await;
    assert!(!data.is_empty(), "PDF content should not be empty");
    println!("Received PDF content: {} bytes", data.len());

    // Save to local temp directory
    let temp_dir = std::env::temp_dir();
    let pdf_path = temp_dir.join("ocudigital_stream.pdf");
    std::fs::write(&pdf_path, data).unwrap();
    println!("PDF saved to: {:?}", pdf_path);
}

#[tokio::test]
async fn test_web_options_trace_id_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.trace_id = Some("test-trace-id".to_string());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_single_page_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.single_page = Some(true);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_paper_size_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.paper_width = Some("210mm".parse().unwrap());
    options.paper_height = Some("297mm".parse().unwrap());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_margins_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.margin_top = Some("1in".parse().unwrap());
    options.margin_bottom = Some("1in".parse().unwrap());
    options.margin_left = Some("0.5in".parse().unwrap());
    options.margin_right = Some("0.5in".parse().unwrap());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_prefer_css_page_size_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.prefer_css_page_size = Some(true);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_print_background_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.print_background = Some(true);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_landscape_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.landscape = Some(true);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_scale_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.scale = Some(1.5);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_page_ranges_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.native_page_ranges = Some("1-3,5".parse().unwrap());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_header_footer_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.header_html = Some("<h1>Header Test: <div class='title'></div></h1>".into());
    options.footer_html = Some("Page Number: <div class='pageNumber'></div>".into());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_wait_delay_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.wait_delay = Some(Duration::from_secs(1));

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_emulated_media_type_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.emulated_media_type = Some("screen".parse().unwrap());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_fail_on_http_status_codes_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.fail_on_http_status_codes = Some(vec![404, 500]);

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_metadata_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.metadata = Some(HashMap::from([
        ("Title".to_string(), "Test Document".into()),
        ("Author".to_string(), "Test Author".into()),
    ]));

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_user_agent_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    options.user_agent = Some("TestUserAgent/1.0".into());

    let stream = client.pdf_from_html(HTML_CONTENT, options).await.unwrap();
    let _pdf_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_web_options_negative_scale_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    // Negative scale should fail
    options.scale = Some(-1.0);

    let result = client.pdf_from_html(HTML_CONTENT, options).await;
    assert!(result.is_err(), "Expected negative scale to fail");
}

#[tokio::test]
async fn test_web_options_unsupported_user_agent_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = WebOptions::default();
    // Unsupported user agent format
    options.user_agent = Some("\0invalid_user_agent".into());

    let result = client.pdf_from_html(HTML_CONTENT, options).await;
    assert!(result.is_err(), "Expected unsupported user agent to fail");
}

#[tokio::test]
async fn test_screenshot_options_trace_id_streaming() {
    let client = StreamingClient::new(&BASE_URL);

    let mut options = ScreenshotOptions::default();
    options.trace_id = Some("test-trace-id".to_string());

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_width_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.width = Some(1024);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_height_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.height = Some(768);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_clip_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.clip = Some(true);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_format_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.format = Some(ImageFormat::Jpeg);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_quality_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.quality = Some(85);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_omit_background_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.omit_background = Some(true);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_optimize_for_speed_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.optimize_for_speed = Some(true);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_wait_delay_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.wait_delay = Some(Duration::from_secs(1));

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_wait_for_expression_streaming() {
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

    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.wait_for_expression = Some("window.isReady === true".to_string());

    let stream = client.screenshot_html(html_content, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_emulated_media_type_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.emulated_media_type = Some(MediaType::Screen);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_cookies_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.cookies = Some(vec![Cookie {
        name: "session".to_string(),
        value: "abc123".to_string(),
        domain: "example.com".to_string(),
        ..Default::default()
    }]);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_skip_network_idle_events_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.skip_network_idle_events = Some(false);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_user_agent_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.user_agent = Some("Test-Agent".to_string());

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_extra_http_headers_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.extra_http_headers = Some(
        vec![
            ("Authorization".to_string(), "Bearer token".to_string()),
            ("X-Custom-Header".to_string(), "custom-value".to_string()),
        ]
        .into_iter()
        .collect(),
    );

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_fail_on_http_status_codes_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_http_status_codes = Some(vec![404, 500]);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_fail_on_resource_http_status_codes_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_resource_http_status_codes = Some(vec![403, 502]);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_fail_on_resource_loading_failed_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_resource_loading_failed = Some(true);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_screenshot_options_fail_on_console_exceptions_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = ScreenshotOptions::default();
    options.fail_on_console_exceptions = Some(true);

    let stream = client.screenshot_html(HTML_CONTENT, options).await.unwrap();
    let _image_bytes = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_trace_id_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.trace_id = Some("some-trace-id".to_string());

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_password_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.password = Some("secure-password".to_string());

    let stream = client
        .pdf_from_doc(
            "example.odt",
            PASSWORD_PROTECTED_ODT_CONTENT.to_vec(),
            options,
        )
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_landscape_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.landscape = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_form_fields_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_form_fields = Some(false);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_allow_duplicate_field_names_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.allow_duplicate_field_names = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_bookmarks_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_bookmarks = Some(false);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_notes_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_quality_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.quality = Some(75);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_max_image_resolution_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.max_image_resolution = Some(600);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_pdfua_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.pdfua = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_native_page_ranges_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.native_page_ranges = Some("1-3,5".parse().unwrap());

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_bookmarks_to_pdf_destination_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_bookmarks_to_pdf_destination = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_placeholders_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_placeholders = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_notes_pages_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes_pages = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_only_notes_pages_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_only_notes_pages = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_notes_in_margin_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_notes_in_margin = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_convert_ooo_target_to_pdf_target_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.convert_ooo_target_to_pdf_target = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_links_relative_fsys_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_links_relative_fsys = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_export_hidden_slides_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.export_hidden_slides = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_skip_empty_pages_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.skip_empty_pages = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_add_original_document_as_stream_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.add_original_document_as_stream = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_single_page_sheets_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.single_page_sheets = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_lossless_image_compression_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.lossless_image_compression = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_reduce_image_resolution_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.reduce_image_resolution = Some(true);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}

#[tokio::test]
async fn test_doc_options_pdfa_streaming() {
    let client = StreamingClient::new(&BASE_URL);
    let mut options = DocumentOptions::default();
    options.pdfa = Some(PDFFormat::A1b);

    let stream = client
        .pdf_from_doc("example.docx", DOCX_CONTENT.to_vec(), options)
        .await
        .unwrap();
    let _pdf_content = collect_stream(stream).await;
}
