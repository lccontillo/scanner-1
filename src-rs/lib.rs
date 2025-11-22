#![no_std]
#[macro_use]
extern crate alloc;

use core::cmp::Ordering;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

mod image;
use image::{Quad, RGBAImage};

#[cfg(not(target_arch = "wasm32"))]
compile_error!("Only compilable to WASM");

// Helper to sort floats safely
fn compare_floats(a: f32, b: f32) -> Ordering {
    a.partial_cmp(&b).unwrap_or(Ordering::Equal)
}

// This function now replicates your JS `fixquad` logic
fn sort_quad(quad: Quad) -> Quad {
    let Quad { a, b, c, d } = quad;
    
    // 1. Collect all points into an array
    let mut points = [a, b, c, d];

    // 2. Sort by X coordinate (simulating `xArr.sort`)
    // This ensures points[0] and points[1] are the "Left" side
    // and points[2] and points[3] are the "Right" side.
    points.sort_by(|p1, p2| compare_floats(p1.x, p2.x));

    // 3. Split into Left and Right pairs
    let mut left = [points[0], points[1]];
    let mut right = [points[2], points[3]];

    // 4. Sort each pair by Y coordinate
    // index 0 becomes Smallest Y (Top), index 1 becomes Largest Y (Bottom)
    left.sort_by(|p1, p2| compare_floats(p1.y, p2.y));
    right.sort_by(|p1, p2| compare_floats(p1.y, p2.y));

    // 5. Assign exactly as per your JS:
    // JS: a = ab[1] (Left, Bottom)
    // JS: b = ab[0] (Left, Top)
    // JS: c = cd[0] (Right, Top)
    // JS: d = cd[1] (Right, Bottom)
    Quad {
        a: left[1],  // Bottom-Left
        b: left[0],  // Top-Left
        c: right[0], // Top-Right
        d: right[1], // Bottom-Right
    }
}

// Kept for reference if you need to calculate aspect ratio, 
// but sort_quad no longer depends on it.
fn sum_sides(quad: Quad) -> (f32, f32) {
    let Quad { a, b, c, d } = quad;
    let side = (a.x - b.x).hypot(a.y - b.y) + (c.x - d.x).hypot(c.y - d.y);
    let top = (b.x - c.x).hypot(b.y - c.y) + (d.x - a.x).hypot(d.y - a.y);
    (side, top)
}

impl From<ImageData> for RGBAImage {
    fn from(data: ImageData) -> Self {
        let width = data.width() as usize;
        let height = data.height() as usize;
        let data = data.data().0;
        RGBAImage {
            data,
            width,
            height,
        }
    }
}

#[macro_export]
macro_rules! perf {
    ($b:expr) => {{
        use js_sys::{global, Reflect};
        use wasm_bindgen::{prelude::*, JsCast};
        use web_sys::Performance;

        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_namespace = console)]
            fn log(a: &str, b: &str, c: &str, d: f64);
        }
        let performance = Reflect::get(&global(), &JsValue::from_str("performance"))
            .unwrap()
            .unchecked_into::<Performance>();
        let ts = performance.now();
        let ret = $b;
        log("time", stringify!($b), "=", performance.now() - ts);
        ret
    }};
}

#[wasm_bindgen]
pub fn find_document(data: ImageData) -> Option<Quad> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let rgba: RGBAImage = data.into();
    let mut by = (rgba.width.min(rgba.height) as f32) / 360.0;
    if by < 2.0 {
        by = 1.0
    }
    let mut src = rgba.to_grayscale();
    if by != 1.0 {
        src = src.downscale(by);
    }
    src.gaussian().document().map(|doc| {
        // The sort is now applied here with the fixed logic
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
pub fn extract_document(
    data: ImageData,
    region: Quad,
    target_width: usize,
    target_height: Option<usize>,
) -> ImageData {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    let rgba: RGBAImage = data.into();
    let target_height = if let Some(height) = target_height {
        height
    } else {
        let (side, top) = sum_sides(region);
        (side / top * (target_width as f32)) as usize
    };
    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&rgba.perspective(region, target_width, target_height).data),
        target_width as u32,
        target_height as u32,
    )
    .unwrap()
}