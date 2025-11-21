#![no_std]
#[macro_use]
extern crate alloc;

use wasm_bindgen::prelude::*;

// This is your local module (src/image/mod.rs)
mod image;
use image::{Quad, RGBAImage};

#[cfg(not(target_arch = "wasm32"))]
compile_error!("Only compilable to WASM");

// Helper to calculate side lengths
fn sum_sides(quad: Quad) -> (f32, f32) {
    let Quad { a, b, c, d } = quad;
    let side = (a.x - b.x).hypot(a.y - b.y) + (c.x - d.x).hypot(c.y - d.y);
    let top = (b.x - c.x).hypot(b.y - c.y) + (d.x - a.x).hypot(d.y - a.y);
    (side, top)
}

// Helper to organize the corners of the Quad
// Helper to organize the corners of the Quad
fn sort_quad(quad: Quad) -> Quad {
    let Quad { a, b, c, d } = quad;

    // 1. Collect the four points into a mutable vector
    let mut points = vec![a, b, c, d];

    // 2. Sort all points primarily by X-coordinate
    // We use a stable sort to maintain relative Y-order for points with the same X
    // but a plain sort works fine here.
    points.sort_by(|p1, p2| p1.x.partial_cmp(&p2.x).unwrap_or(std::cmp::Ordering::Equal));

    // 3. Separate the left two (P0, P1) from the right two (P2, P3)
    let mut left_side = vec![points[0], points[1]];
    let mut right_side = vec![points[2], points[3]];

    // 4. Sort left side by Y-coordinate to get Top-Left (a) and Bottom-Left (b)
    // The top-down image coordinate system means a smaller Y is "top".
    left_side.sort_by(|p1, p2| p1.y.partial_cmp(&p2.y).unwrap_or(std::cmp::Ordering::Equal));
    
    // a is Top-Left (smaller Y)
    let a = left_side[0];
    // b is Bottom-Left (larger Y)
    let b = left_side[1];

    // 5. Sort right side by Y-coordinate to get Top-Right (d) and Bottom-Right (c)
    right_side.sort_by(|p1, p2| p1.y.partial_cmp(&p2.y).unwrap_or(std::cmp::Ordering::Equal));

    // d is Top-Right (smaller Y)
    let d = right_side[0];
    // c is Bottom-Right (larger Y)
    let c = right_side[1];

    // Return the correctly ordered Quad: Top-Left (a), Bottom-Left (b), Bottom-Right (c), Top-Right (d)
    Quad { a, b, c, d }
}
// Import types from the external 'image' crate.
// We use ::image to ensure we aren't looking inside our local 'mod image'.
use ::image::{load_from_memory, RgbaImage};

// Convert external image crate RgbaImage to your internal RGBAImage
impl From<RgbaImage> for RGBAImage {
    fn from(rgba_image: RgbaImage) -> Self {
        let width = rgba_image.width() as usize;
        let height = rgba_image.height() as usize;
        let data = rgba_image.into_raw();
        RGBAImage { data, width, height }
    }
}

#[wasm_bindgen]
pub fn find_document(buf: &js_sys::Uint8Array) -> Option<Quad> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let bytes = buf.to_vec();
    
    // Use the imported 'load_from_memory' directly
    let img = load_from_memory(&bytes).ok()?;
    let rgba_image = RGBAImage::from(img.to_rgba8());

    let mut by = (rgba_image.width.min(rgba_image.height) as f32) / 360.0;
    if by < 2.0 {
        by = 1.0;
    }

    let mut src = rgba_image.to_grayscale();
    if by != 1.0 {
        src = src.downscale(by);
    }

    // Assuming gaussian() and document() are defined in your local module
    src.gaussian().document().map(|doc| {
        let mut doc = sort_quad(doc.quad);
        doc.a.x *= by;
        doc.a.y *= by;
        doc.b.x *= by;
        doc.b.y *= by;
        doc.c.x *= by;
        doc.c.y *= by;
        doc.d.x *= by;
        doc.d.y *= by;
        doc
    })
}

#[wasm_bindgen]
pub struct ExtractedImage {
    // FIX 1: Added getter_with_clone because Uint8Array is not Copy
    #[wasm_bindgen(getter_with_clone)]
    pub data: js_sys::Uint8Array,
    pub width: usize,
    pub height: usize,
}

#[wasm_bindgen]
pub fn extract_document(buf: &js_sys::Uint8Array, region: Quad, target_width: usize) -> ExtractedImage {
    let bytes = buf.to_vec();
    
    // FIX 2: Removed 'image::' prefix. 
    // Calling 'image::load_from_memory' tries to look in your LOCAL module.
    // We call 'load_from_memory' which is imported from the EXTERNAL crate.
    let img = load_from_memory(&bytes).unwrap();
    
    let rgba_image = RGBAImage::from(img.to_rgba8());

    let (side, top) = sum_sides(region);
    let target_height = (side / top * (target_width as f32)) as usize;

    let extracted = rgba_image.perspective(region, target_width, target_height);

    ExtractedImage {
        data: js_sys::Uint8Array::from(&extracted.data[..]),
        width: extracted.width,
        height: extracted.height,
    }
}