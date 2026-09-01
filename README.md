# Gotenberg PDF Client

[![crates.io](https://img.shields.io/crates/v/gotenberg_pdf.svg)](https://crates.io/crates/gotenberg_pdf)
[![docs.rs](https://img.shields.io/badge/docs.rs-gotenberg_pdf-green)](https://docs.rs/gotenberg_pdf)
[![GitHub Actions](https://github.com/phayes/gotenberg_pdf/actions/workflows/rust.yml/badge.svg)](https://github.com/phayes/gotenberg_pdf/actions)

**`gotenberg_pdf`** is a Rust library that provides an easy-to-use interface for interacting with the [Gotenberg API](https://gotenberg.dev/).

Gotenberg is a docker-based service for converting HTML, Markdown, URLs, and various documents to PDFs. It uses the Chrome engine to render web content to PDF and the LibreOffice engine to convert documents.

## Features

- **URL to PDF**: Generate PDFs directly from a webpage URL.
- **HTML to PDF**: Convert raw HTML into a PDF.
- **Markdown to PDF**: Render Markdown files into a PDF.
- **Document to PDF**: Convert various document formats (e.g., DOCX, PPTX) to PDF using the LibreOffice engine.
- **Screenshot**: Capture screenshots of webpages or HTML content.

## Installation

Add `gotenberg_pdf` to your `Cargo.toml`:

```toml
[dependencies]
gotenberg_pdf = "0.5"
```

Ensure you have a running instance of Gotenberg, typically via Docker:

```sh
docker run --rm -p 3000:3000 gotenberg/gotenberg:8
```

**N.B.**: This crate is compatible with Gotenberg version 8.

## Usage Examples

### Convert URL to PDF

```rust
use gotenberg_pdf::{Client, WebOptions, PaperFormat};
use tokio;

#[tokio::main]
async fn main() {
    // Initialize the client with the Gotenberg server URL
    let client = Client::new("http://localhost:3000");

    // Define optional rendering configurations
    let mut options = WebOptions::default();
    options.set_paper_format(PaperFormat::A4);

    // Convert a URL to PDF
    let pdf_bytes = client.pdf_from_url("https://example.com", options).await.unwrap();
}
```

### Convert HTML to PDF

```rust
use gotenberg_pdf::{Client, WebOptions};
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:3000");

    let html_content = r#"
    <!doctype html>
    <html>
        <head><title>My PDF</title></head>
        <body><h1>Hello, PDF!</h1></body>
    </html>
    "#;

    let options = WebOptions::default();

    let pdf_bytes = client.pdf_from_html(html_content, options).await.unwrap();
}
```

### Convert Markdown to PDF

```rust
use gotenberg_pdf::{Client, WebOptions};
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:3000");

    // Markdown content
    let mut markdown_files = HashMap::new();
    markdown_files.insert("example.md", "# My Markdown PDF\nThis is a test document.");

    // HTML template to wrap the markdown
    let html_template = r#"
    <!doctype html>
    <html>
        <head><title>Markdown PDF</title></head>
        <body>{{ toHTML "example.md" }}</body>
    </html>
    "#;

    let options = WebOptions::default();

    let pdf_bytes = client.pdf_from_markdown(html_template, markdown_files, options).await.unwrap();
}
```

### Take a Screenshot of a URL

```rust
use gotenberg_pdf::{Client, ScreenshotOptions, ImageFormat};
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:3000");

    let mut options = ScreenshotOptions::default();
    options.width = Some(1920);
    options.height = Some(1080);
    options.format = Some(ImageFormat::Png);

    let image_bytes = client.screenshot_url("https://example.com", options).await.unwrap();

    println!("Screenshot captured: {} bytes", image_bytes.len());
}
```

### Convert Document to PDF Using LibreOffice Engine

```rust
use gotenberg_pdf::{Client, DocumentOptions};
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:3000");

    let filename = "test_files/example.docx";
    let file_content = std::fs::read(filename).expect("Failed to read the file");

    let options = DocumentOptions {
        landscape: Some(false),
        ..Default::default()
    };

    let pdf_bytes = client.pdf_from_doc(filename, file_content, options).await.unwrap();
}
```

### Convert HTML to Screenshot Image

```rust
use gotenberg_pdf::{Client, ScreenshotOptions, ImageFormat};
use tokio;

#[tokio::main]
async fn main() {
    let client = Client::new("http://localhost:3000");

    let html_content = r#"
    <!doctype html>
    <html>
        <head><title>Screenshot</title></head>
        <body><h1>Hello, Screenshot!</h1></body>
    </html>
    "#;

    let mut options = ScreenshotOptions::default();
    options.width = Some(800);
    options.height = Some(600);
    options.format = Some(ImageFormat::Png);

    let image_bytes = client.screenshot_html(html_content, options).await.unwrap();
}
```


### Use the streaming client

Requires the `stream` feature to be enabled in your `Cargo.toml`.

```rust
use gotenberg_pdf::{StreamingClient, WebOptions};
use futures::StreamExt; // for `next()`
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = StreamingClient::new("http://localhost:3000");

    let options = WebOptions::default();
    let mut stream = client.pdf_from_url("https://example.com", options).await?;

    // Create or overwrite the PDF file asynchronously
    let temp_dir = std::env::temp_dir();
    let pdf_path = temp_dir.join("example_com.pdf");
    let mut file = File::create(pdf_path).await?;

    // As we receive chunks, write them directly to disk
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }

    println!("PDF rendered and saved as example_com.pdf");
    Ok(())
}
```


### Use the blocking client for use without tokio or another async runtime.

Requires the `blocking` feature to be enabled in your `Cargo.toml`.

```rust
use gotenberg_pdf::{BlockingClient, WebOptions, PaperFormat};

fn main() {
    // Initialize the client with the Gotenberg server URL
    let client = BlockingClient::new("http://localhost:3000");

    // Define optional rendering configurations
    let mut options = WebOptions::default();
    options.set_paper_format(PaperFormat::A4);

    // Convert a URL to PDF
    let pdf_bytes = client.pdf_from_url("https://example.com", options).unwrap();
}
```

## Configuration Options

### [`WebOptions`]

Provides control over the PDF generation process from the Chrome engine. These options can be passed to the following methods:
   - [`Client::pdf_from_url`]
   - [`Client::pdf_from_html`]
   - [`Client::pdf_from_markdown`]

| Field Name                          | Description                                      | Default         |
|-------------------------------------|--------------------------------------------------|-----------------|
| trace_id                            | Unique trace ID for request                      | Random UUID     |
| single_page                         | Print content on one page                        | false           |
| paper_width                         | Paper width as a [LinearDimention]               | 8.5 inches      |
| paper_height                        | Paper height as a [LinearDimention]              | 11 inches       |
| margin_top                          | Top margin as a [LinearDimention]                | 0.39 inches     |
| margin_bottom                       | Bottom margin as a [LinearDimention]             | 0.39 inches     |
| margin_left                         | Left margin as a [LinearDimention]               | 0.39 inches     |
| margin_right                        | Right margin as a [LinearDimention]              | 0.39 inches     |
| prefer_css_page_size                | Use CSS-defined page size                        | false           |
| generate_document_outline           | Embed document outline                           | false           |
| print_background                    | Include background graphics                      | false           |
| omit_background                     | Allow transparency in PDF                        | false           |
| landscape                           | Set page orientation to landscape                | false           |
| scale                               | Scale of page rendering                          | 1.0             |
| native_page_ranges                  | [`PageRange`] to print, eg `"1,3,5"`, `"1-4'`    | All pages       |
| header_html                         | HTML for header content                          | None            |
| footer_html                         | HTML for footer content                          | None            |
| wait_delay                          | Delay before conversion                          | None            |
| wait_for_expression                 | Wait until this JS expression returns true       | None            |
| emulated_media_type                 | Emulated [`MediaType`] ("screen" or "print")     | print           |
| cookies                             | Cookies for Chromium                             | None            |
| skip_network_idle_events            | Ignore network idle events                       | true            |
| user_agent                          | Override default User-Agent header               | None            |
| extra_http_headers                  | Additional HTTP headers                          | None            |
| pdfa                                | Convert to specific PDF/A [PDFFormat]            | None            |
| pdfua                               | Enable Universal Access compliance               | false           |
| metadata                            | PDF metadata                                     | None            |
| fail_on_http_status_codes           | HTTP status codes to fail on, 99's are wild      | [499, 599]      |
| fail_on_resource_http_status_codes  | Resource HTTP status codes to fail on            | None            |
| fail_on_resource_loading_failed     | Fail if resource loading fails                   | false           |
| fail_on_console_exceptions          | Fail on Chromium console exceptions              | false           |

Includes the [`WebOptions::set_paper_format`] utlity method for common paper sizes.


### [`ScreenshotOptions`]

Provides control over the screenshot generation process from the Chrome engine. These options can be passed to the following method:
   - [`Client::screenshot_url`]
   - [`Client::screenshot_html`]
   - [`Client::screenshot_markdown`]

| Field Name                          | Description                                      | Default         |
|-------------------------------------|--------------------------------------------------|-----------------|
| trace_id                            | Unique trace ID for request                      | Random UUID     |
| width                               | Device screen width in pixels                    | 800             |
| height                              | Device screen height in pixels                   | 600             |
| clip                                | Clip screenshot to device dimensions             | false           |
| format                              | Image format as an [ImageFormat]                 | png             |
| quality                             | Compression quality (jpeg only, 0-100)           | 100             |
| omit_background                     | Generate screenshot with transparency            | false           |
| optimize_for_speed                  | Optimize image encoding for speed                | false           |
| wait_delay                          | Delay before taking screenshot                   | None            |
| wait_for_expression                 | Wait until this JS expression returns true       | None            |
| emulated_media_type                 | Emulated [`MediaType`] ("screen" or "print")     | print           |
| cookies                             | Cookies for Chromium                             | None            |
| skip_network_idle_events            | Ignore network idle events                       | true            |
| user_agent                          | Override default User-Agent header               | None            |
| extra_http_headers                  | Additional HTTP headers                          | None            |
| fail_on_http_status_codes           | HTTP status codes to fail on, 99's are wild      | [499, 599]      |
| fail_on_resource_http_status_codes  | Resource HTTP status codes to fail on            | None            |
| fail_on_resource_loading_failed     | Fail if resource loading fails                   | false           |
| fail_on_console_exceptions          | Fail on Chromium console exceptions              | None            |


### [`DocumentOptions`]

Provides control over the document generation process from the LibreOffice engine. These options can be passed to the following method:
   - [`Client::pdf_from_doc`]

| Field Name                          | Description                                      | Default         |
|-------------------------------------|--------------------------------------------------|-----------------|
| trace_id                            | Unique trace ID for request                      | Random UUID            |
| password                            | Password for opening the source file             | None            |
| landscape                           | Set paper orientation to landscape               | false           |
| native_page_ranges                  | [`PageRange`] to print, eg `"1,2,3"` or `"1-4"`  | All pages       |
| export_form_fields                  | Export form fields as widgets                    | true            |
| allow_duplicate_field_names         | Allow duplicate field names in form fields       | false           |
| export_bookmarks                    | Export bookmarks to PDF                          | true            |
| export_bookmarks_to_pdf_destination | Export bookmarks as named destinations           | false           |
| export_placeholders                 | Export placeholder fields visual markings only   | false           |
| export_notes                        | Export notes to PDF                              | false           |
| export_notes_pages                  | Export notes pages (Impress only)                | false           |
| export_only_notes_pages             | Export only notes pages                          | false           |
| export_notes_in_margin              | Export notes in margin                           | false           |
| convert_ooo_target_to_pdf_target    | Convert `.od[tpgs]` links to `.pdf`              | false           |
| export_links_relative_fsys          | Export file:// links as relative                 | false           |
| export_hidden_slides                | Export hidden slides (Impress only)              | false           |
| skip_empty_pages                    | Suppress automatically inserted empty pages      | false           |
| add_original_document_as_stream     | Add original document as a stream                | false           |
| single_page_sheets                  | Put each sheet on one page                       | false           |
| lossless_image_compression          | Use lossless image compression (e.g., PNG)       | false           |
| quality                             | JPG export quality (1-100)                       | 90              |
| reduce_image_resolution             | Reduce image resolution                          | false           |
| max_image_resolution                | Max resolution DPI. 75, 150, 300, 600 or 1200    | 300             |
| pdfa                                | Convert to specific PDF/A [PDFFormat]            | None            |
| pdfua                               | Enable Universal Access compliance               | false           |

## Features

### TLS / HTTPS

By default there is no support for HTTPS. If you need TLS, you can enable it by adding one of the following features to your `Cargo.toml`:

 - `rustls` - Enables TLS / HTTPS support using the `rustls` library.
 - `native-tls` - Enables TLS / HTTPS support using the native system TLS library.

### HTTP/2

By default there is no HTTP/2 support. HTTP/2 support can be enalbed with the `http2` feature. Even with the feature enabled, HTTP/2 will not be selected unless connecting over HTTPS. If you need HTTP/2 over plain HTTP, you need to make use of [`Client::new_with_client`] and [`reqwest::ClientBuilder::http2_prior_knowledge`].

### Additional features

  - `stream`   - Enables the streaming client to stream generated PDFs directly to disk or other destinations.
  - `blocking` - Enables the blocking client for use without tokio or another async runtime.
  - `zeroize`  - Enables zeroizing sensitive data in the client. Enabled by default.

## Web Assembly / Browser Support

This crate compiles to `wasm32-unknown-unknown` and is runnable in the browser. In the browser, it will use the built-in browser fetch API to make requests to the Gotenberg server. The `stream`, `blocking`, `rustls` and `native-tls` features are not available on wasm32 or in the browser.

Be aware that in the browser, the gotenberg server will need to be behind a proxy that sets the correct CORS headers ('Access-Control-Allow-Origin').
