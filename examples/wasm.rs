#[cfg(target_arch = "wasm32")]
use pdfium_render::prelude::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::Blob;

// See https://github.com/ajrcarey/pdfium-render/tree/master/examples for information
// on how to build and package this example alongside a WASM build of Pdfium, suitable
// for running in a browser.

/// Downloads the given url, opens it as a PDF document, then returns the ImageData for
/// the given page index using the given bitmap dimensions.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub async fn convert_pdf_to_png_with_metadata(blob: Blob) -> Blob {
    use std::io::Cursor;

    use image::ImageFormat;
    use js_sys::{Array, Uint8Array};
    use pdfium_render::pdftopng::{self, RenderedImage};
    use prost::Message;

    let images = Pdfium::default()
        .load_pdf_from_blob(blob, None)
        .await
        .unwrap()
        .pages()
        .iter()
        .map(|mut page| -> pdftopng::RenderedImage {
            page.flatten().unwrap();
            let image = page
                .render_with_config(
                    &PdfRenderConfig::new()
                        .render_form_data(true)
                        .scale_page_by_factor(2.0),
                )
                .unwrap()
                .as_image();
            let mut png_bytes_cur = Cursor::new(Vec::new());
            image
                .write_to(&mut png_bytes_cur, ImageFormat::Png)
                .unwrap();
            RenderedImage {
                data: png_bytes_cur.into_inner(),
                format: "png".to_string(),
                height: image.height(),
                width: image.width(),
            }
        })
        .collect();
    let result = pdftopng::PdfToPngResult { images };
    let mut buf = Vec::new();
    buf.reserve(result.encoded_len());
    result.encode(&mut buf).unwrap();
    //from https://github.com/rustwasm/wasm-bindgen/issues/1693#issuecomment-926879272
    let arr = Array::new();
    arr.push(&Uint8Array::from(buf.as_slice()));
    Blob::new_with_u8_array_sequence(&arr).unwrap()
}

// Source files in examples/ directory are expected to always have a main() entry-point.
// Since we're compiling to WASM, we'll never actually use this.
#[allow(dead_code)]
fn main() {}
