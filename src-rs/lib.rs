#![no_std]
#[macro_use]
extern crate alloc;

use core::cmp::Ordering;
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::ImageData;

mod image;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::slice;
use image::{Image, Quad, RGBAImage};

static mut SHARED_BUFFER: Vec<u8> = Vec::new();

#[wasm_bindgen]
pub fn alloc(len: usize) -> *const u8 {
    unsafe {
        if SHARED_BUFFER.len() < len {
            SHARED_BUFFER.resize(len, 0);
        }
        SHARED_BUFFER.as_ptr()
    }
}

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

impl From<ImageData> for RGBAImage<'static> {
    fn from(data: ImageData) -> Self {
        let width = data.width() as usize;
        let height = data.height() as usize;
        let data = data.data().0;
        RGBAImage {
            data: Cow::Owned(data),
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
    let mut by = (rgba.width.min(rgba.height) as f32) / 480.0;
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
pub fn find_document_shared(width: usize, height: usize) -> Option<Quad> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let data = unsafe { slice::from_raw_parts(SHARED_BUFFER.as_ptr(), width * height * 4) };
    let rgba = RGBAImage {
        data: Cow::Borrowed(data),
        width,
        height,
    };

    let mut by = (rgba.width.min(rgba.height) as f32) / 480.0;
    if by < 2.0 {
        by = 1.0
    }
    let mut src = rgba.to_grayscale();
    if by != 1.0 {
        src = src.downscale(by);
    }
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
pub fn find_document_yuv_shared(width: usize, height: usize) -> Option<Quad> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let data = unsafe { slice::from_raw_parts(SHARED_BUFFER.as_ptr(), width * height) };
    let float_data: Vec<f32> = data.iter().map(|&p| p as f32 / 255.0).collect();

    let mut src = Image {
        data: float_data,
        width,
        height,
    };

    let mut by = (src.width.min(src.height) as f32) / 480.0;
    if by < 2.0 {
        by = 1.0
    }

    if by != 1.0 {
        src = src.downscale(by);
    }
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

#[wasm_bindgen]
pub fn extract_document_shared(
    width: usize,
    height: usize,
    region: Quad,
    target_width: usize,
    target_height: Option<usize>,
) -> ImageData {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let data = unsafe { slice::from_raw_parts(SHARED_BUFFER.as_ptr(), width * height * 4) };
    let rgba = RGBAImage {
        data: Cow::Borrowed(data),
        width,
        height,
    };

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

#[derive(Copy, Clone)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Copy, Clone)]
struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

#[wasm_bindgen]
pub fn apply_filter(
    data: ImageData,
    filter_type: &str,
    sign_color_name: Option<alloc::string::String>,
) -> ImageData {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let width = data.width() as usize;
    let height = data.height() as usize;
    let mut pixels = data.data().0;

    match filter_type {
        "original" => {
            let mut temp = pixels.clone();
            sharpen_pixels(&pixels, &mut temp, width, height, 0.6);
            contrast_image(&mut temp, 0.25);
            pixels = temp;
        }
        "magic" => {
            let mut temp = pixels.clone();
            apply_magic_color(&pixels, &mut temp, width, height, 45);
            pixels = temp;
        }
        "bw" => {
            let mut temp = pixels.clone();
            apply_bradley_threshold(&pixels, &mut temp, width, height, 51, 0.10);
            pixels = temp;
        }
        "gray" => {
            grayscale_image(&mut pixels);
        }
        "photo" => {
            contrast_image(&mut pixels, 0.18);
        }
        "sign" => {
            let sign_color = match sign_color_name.as_deref() {
                Some("red") => RGBA { r: 255, g: 0, b: 0, a: 255 },
                Some("green") => RGBA { r: 0, g: 255, b: 0, a: 255 },
                Some("blue") => RGBA { r: 0, g: 0, b: 255, a: 255 },
                Some("purple") => RGBA { r: 255, g: 0, b: 255, a: 255 },
                _ => RGBA { r: 0, g: 0, b: 0, a: 255 },
            };

            let mut temp = pixels.clone();
            sharpen_pixels(&pixels, &mut temp, width, height, 0.6);
            contrast_image(&mut temp, 0.25);

            let mut canvas1 = temp;
            set_image_data(
                &mut canvas1,
                RGB { r: 255, g: 255, b: 255 },
                RGBA { r: 255, g: 255, b: 255, a: 255 },
                sign_color,
                72.0,
            );

            let mut canvas2 = canvas1.clone();
            blur_rgba(&canvas1, &mut canvas2, width, height);
            apply_brightness_contrast(&mut canvas2, 2.0, 6.0);

            let mut canvas3 = canvas2.clone();
            set_image_data(
                &mut canvas3,
                RGB { r: 255, g: 255, b: 255 },
                RGBA { r: 255, g: 255, b: 255, a: 0 },
                sign_color,
                10.0,
            );
            pixels = canvas3;
        }
        _ => {}
    }

    ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&pixels),
        width as u32,
        height as u32,
    )
    .unwrap()
}

fn sharpen_pixels(src: &[u8], dst: &mut [u8], w: usize, h: usize, mix: f32) {
    let weights = [0, -1, 0, -1, 5, -1, 0, -1, 0];
    let katet = 3;
    let half = 1;

    for y in 0..h {
        for x in 0..w {
            let dst_off = (y * w + x) * 4;
            if x > 0 && y > 0 && x < w - 1 && y < h - 1 {
                let mut r = 0;
                let mut g = 0;
                let mut b = 0;
                for cy in 0..katet {
                    for cx in 0..katet {
                        let scy = (y as isize) + (cy as isize) - half;
                        let scx = (x as isize) + (cx as isize) - half;
                        if scy >= 0 && scy < h as isize && scx >= 0 && scx < w as isize {
                            let src_off = ((scy as usize) * w + (scx as usize)) * 4;
                            let wt = weights[cy * katet + cx];
                            r += (src[src_off] as i32) * wt;
                            g += (src[src_off + 1] as i32) * wt;
                            b += (src[src_off + 2] as i32) * wt;
                        }
                    }
                }
                
                let out_r = (r as f32) * mix + (src[dst_off] as f32) * (1.0 - mix);
                let out_g = (g as f32) * mix + (src[dst_off + 1] as f32) * (1.0 - mix);
                let out_b = (b as f32) * mix + (src[dst_off + 2] as f32) * (1.0 - mix);

                dst[dst_off] = out_r.clamp(0.0, 255.0) as u8;
                dst[dst_off + 1] = out_g.clamp(0.0, 255.0) as u8;
                dst[dst_off + 2] = out_b.clamp(0.0, 255.0) as u8;
                dst[dst_off + 3] = src[dst_off + 3];
            } else {
                dst[dst_off] = src[dst_off];
                dst[dst_off + 1] = src[dst_off + 1];
                dst[dst_off + 2] = src[dst_off + 2];
                dst[dst_off + 3] = src[dst_off + 3];
            }
        }
    }
}

fn contrast_image(pixels: &mut [u8], contrast: f32) {
    let contrast_val = contrast * 255.0;
    let factor = (contrast_val + 255.0) / (255.01 - contrast_val);
    for i in (0..pixels.len()).step_by(4) {
        let r = factor * (pixels[i] as f32 - 128.0) + 128.0;
        let g = factor * (pixels[i + 1] as f32 - 128.0) + 128.0;
        let b = factor * (pixels[i + 2] as f32 - 128.0) + 128.0;

        pixels[i] = r.clamp(0.0, 255.0) as u8;
        pixels[i + 1] = g.clamp(0.0, 255.0) as u8;
        pixels[i + 2] = b.clamp(0.0, 255.0) as u8;
    }
}

fn grayscale_image(pixels: &mut [u8]) {
    for i in (0..pixels.len()).step_by(4) {
        let lightness = ((pixels[i] as u32 + pixels[i + 1] as u32 + pixels[i + 2] as u32) / 3) as u8;
        pixels[i] = lightness;
        pixels[i + 1] = lightness;
        pixels[i + 2] = lightness;
    }
}

fn set_image_data(
    pixels: &mut [u8],
    target_color: RGB,
    new_color_selected: RGBA,
    new_color_unselected: RGBA,
    tolerance: f32,
) {
    let is_mask = (((new_color_selected.r as i32 - new_color_unselected.r as i32).abs()
        + (new_color_selected.g as i32 - new_color_unselected.g as i32).abs()
        + (new_color_selected.b as i32 - new_color_unselected.b as i32).abs())
        / 3)
        == 255;

    for i in (0..pixels.len()).step_by(4) {
        let cr = pixels[i];
        let cg = pixels[i + 1];
        let cb = pixels[i + 2];

        let diff = ((cr as i32 - target_color.r as i32).abs()
            + (cg as i32 - target_color.g as i32).abs()
            + (cb as i32 - target_color.b as i32).abs()) as f32
            / 3.0;

        if diff > tolerance {
            pixels[i] = new_color_unselected.r;
            pixels[i + 1] = new_color_unselected.g;
            pixels[i + 2] = new_color_unselected.b;
            pixels[i + 3] = if new_color_selected.a == 0 && is_mask {
                diff.clamp(0.0, 255.0) as u8
            } else {
                new_color_unselected.a
            };
        } else {
            pixels[i] = new_color_selected.r;
            pixels[i + 1] = new_color_selected.g;
            pixels[i + 2] = new_color_selected.b;
            pixels[i + 3] = new_color_selected.a;
        }
    }
}

fn blur_rgba(src: &[u8], dst: &mut [u8], w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            let idx = (y * w + x) * 4;
            let mut sum_r = 0;
            let mut sum_g = 0;
            let mut sum_b = 0;
            let mut count = 0;
            
            for ky in -1..=1 {
                let py = y as isize + ky;
                if py >= 0 && py < h as isize {
                    for kx in -1..=1 {
                        let px = x as isize + kx;
                        if px >= 0 && px < w as isize {
                            let n_idx = ((py as usize) * w + (px as usize)) * 4;
                            sum_r += src[n_idx] as u32;
                            sum_g += src[n_idx + 1] as u32;
                            sum_b += src[n_idx + 2] as u32;
                            count += 1;
                        }
                    }
                }
            }
            
            dst[idx] = (sum_r / count) as u8;
            dst[idx + 1] = (sum_g / count) as u8;
            dst[idx + 2] = (sum_b / count) as u8;
            dst[idx + 3] = src[idx + 3];
        }
    }
}

fn apply_brightness_contrast(pixels: &mut [u8], brightness: f32, contrast: f32) {
    for i in (0..pixels.len()).step_by(4) {
        let r = pixels[i] as f32 * brightness;
        let g = pixels[i + 1] as f32 * brightness;
        let b = pixels[i + 2] as f32 * brightness;

        let r = (r - 128.0) * contrast + 128.0;
        let g = (g - 128.0) * contrast + 128.0;
        let b = (b - 128.0) * contrast + 128.0;

        pixels[i] = r.clamp(0.0, 255.0) as u8;
        pixels[i + 1] = g.clamp(0.0, 255.0) as u8;
        pixels[i + 2] = b.clamp(0.0, 255.0) as u8;
    }
}

// Fast horizontal & vertical sliding-window box blur. O(N) complexity.
// Correctly clamps at boundary coordinates and uses a constant divisor to avoid border artifact scaling.
fn fast_box_blur_rgba(src: &[u8], dst: &mut [u8], w: usize, h: usize, radius: usize) {
    let mut temp = vec![0u8; src.len()];
    let r = radius;
    let div = (2 * r + 1) as u32;

    // Horizontal pass
    for y in 0..h {
        let row_offset = y * w * 4;

        let mut sum_r = r as u32 * src[row_offset] as u32;
        let mut sum_g = r as u32 * src[row_offset + 1] as u32;
        let mut sum_b = r as u32 * src[row_offset + 2] as u32;

        for x in 0..=r {
            let idx = row_offset + x.min(w - 1) * 4;
            sum_r += src[idx] as u32;
            sum_g += src[idx + 1] as u32;
            sum_b += src[idx + 2] as u32;
        }

        for x in 0..w {
            let out_idx = row_offset + x * 4;
            temp[out_idx] = (sum_r / div) as u8;
            temp[out_idx + 1] = (sum_g / div) as u8;
            temp[out_idx + 2] = (sum_b / div) as u8;
            temp[out_idx + 3] = src[out_idx + 3];

            // Slide window for next step
            let next_r_x = (x + 1 + r).min(w - 1);
            let prev_l_x = if x >= r { x - r } else { 0 };

            let next_idx = row_offset + next_r_x * 4;
            let prev_idx = row_offset + prev_l_x * 4;

            sum_r = sum_r + src[next_idx] as u32 - src[prev_idx] as u32;
            sum_g = sum_g + src[next_idx + 1] as u32 - src[prev_idx + 1] as u32;
            sum_b = sum_b + src[next_idx + 2] as u32 - src[prev_idx + 2] as u32;
        }
    }

    // Vertical pass
    for x in 0..w {
        let mut sum_r = r as u32 * temp[x * 4] as u32;
        let mut sum_g = r as u32 * temp[x * 4 + 1] as u32;
        let mut sum_b = r as u32 * temp[x * 4 + 2] as u32;

        for y in 0..=r {
            let idx = (y.min(h - 1) * w + x) * 4;
            sum_r += temp[idx] as u32;
            sum_g += temp[idx + 1] as u32;
            sum_b += temp[idx + 2] as u32;
        }

        for y in 0..h {
            let out_idx = (y * w + x) * 4;
            dst[out_idx] = (sum_r / div) as u8;
            dst[out_idx + 1] = (sum_g / div) as u8;
            dst[out_idx + 2] = (sum_b / div) as u8;
            dst[out_idx + 3] = temp[out_idx + 3];

            // Slide window for next step
            let next_r_y = (y + 1 + r).min(h - 1);
            let prev_l_y = if y >= r { y - r } else { 0 };

            let next_idx = (next_r_y * w + x) * 4;
            let prev_idx = (prev_l_y * w + x) * 4;

            sum_r = sum_r + temp[next_idx] as u32 - temp[prev_idx] as u32;
            sum_g = sum_g + temp[next_idx + 1] as u32 - temp[prev_idx + 1] as u32;
            sum_b = sum_b + temp[next_idx + 2] as u32 - temp[prev_idx + 2] as u32;
        }
    }
}

// Separable 1D horizontal & vertical RGBA dilation (local maximum). O(N) complexity.
// Dilates each color channel independently to estimate per-channel background paper illumination.
fn fast_dilate_rgba(src: &[u8], dst: &mut [u8], w: usize, h: usize, radius: usize) {
    let mut temp = vec![0u8; src.len()];

    // Horizontal pass
    for y in 0..h {
        let row_offset = y * w * 4;
        for x in 0..w {
            let mut max_r = 0u8;
            let mut max_g = 0u8;
            let mut max_b = 0u8;
            
            let start = if x >= radius { x - radius } else { 0 };
            let end = (x + radius).min(w - 1);
            
            for kx in start..=end {
                let idx = row_offset + kx * 4;
                if src[idx] > max_r { max_r = src[idx]; }
                if src[idx + 1] > max_g { max_g = src[idx + 1]; }
                if src[idx + 2] > max_b { max_b = src[idx + 2]; }
            }
            
            let out_idx = row_offset + x * 4;
            temp[out_idx] = max_r;
            temp[out_idx + 1] = max_g;
            temp[out_idx + 2] = max_b;
            temp[out_idx + 3] = src[out_idx + 3];
        }
    }

    // Vertical pass
    for x in 0..w {
        for y in 0..h {
            let mut max_r = 0u8;
            let mut max_g = 0u8;
            let mut max_b = 0u8;
            
            let start = if y >= radius { y - radius } else { 0 };
            let end = (y + radius).min(h - 1);
            
            for ky in start..=end {
                let idx = (ky * w + x) * 4;
                if temp[idx] > max_r { max_r = temp[idx]; }
                if temp[idx + 1] > max_g { max_g = temp[idx + 1]; }
                if temp[idx + 2] > max_b { max_b = temp[idx + 2]; }
            }
            
            let out_idx = (y * w + x) * 4;
            dst[out_idx] = max_r;
            dst[out_idx + 1] = max_g;
            dst[out_idx + 2] = max_b;
            dst[out_idx + 3] = temp[out_idx + 3];
        }
    }
}

// Flat-Field Division Normalization using Dilation ("Magic Color")
fn apply_magic_color(src: &[u8], dst: &mut [u8], w: usize, h: usize, _radius: usize) {
    let dilate_radius = 12;
    let blur_radius = 12;

    // 1. Dilation: Erase all dark text from background estimation per-channel
    let mut dilated = vec![0u8; src.len()];
    fast_dilate_rgba(src, &mut dilated, w, h, dilate_radius);

    // 2. Blur: Smooth the dilated paper background map
    let mut bg = vec![0u8; src.len()];
    fast_box_blur_rgba(&dilated, &mut bg, w, h, blur_radius);

    // 3. Normalize: divide original by the clean background map and apply gamma
    for i in (0..src.len()).step_by(4) {
        for c in 0..3 {
            let s = src[i + c] as f32;
            let b = bg[i + c] as f32;
            
            // Per-channel division removes camera color shifts & chromatic text boundaries
            let norm = if b > 1.0 { (s / b).min(1.0) } else { s / 255.0 };
            
            // Gamma curve (1.8) to darken/bolden text details
            let corrected = norm.powf(1.8);
            
            // Whiten paper: values above 0.85 (corrected scale) are forced to pure white
            let final_val = if corrected > 0.85 {
                255.0
            } else {
                (corrected / 0.85) * 255.0
            };
            
            dst[i + c] = final_val.clamp(0.0, 255.0) as u8;
        }
        dst[i + 3] = src[i + 3];
    }
}

// B&W: Normalize lighting using Magic Color, then apply a clean global threshold
fn apply_bradley_threshold(src: &[u8], dst: &mut [u8], w: usize, h: usize, _win_size: usize, _t: f32) {
    // 1. Normalize lighting first (flat-field correction using dilation magic color)
    let mut normalized = vec![0u8; src.len()];
    apply_magic_color(src, &mut normalized, w, h, 0);

    // 2. Threshold the normalized image to binary
    for i in (0..src.len()).step_by(4) {
        // Grayscale conversion using accurate luminance weights
        let gray = ((normalized[i] as u32 * 77 + normalized[i + 1] as u32 * 150 + normalized[i + 2] as u32 * 29) >> 8) as u8;
        
        // Simple, sharp global threshold at 190 (since paper is pushed to 255)
        let val = if gray < 190 { 0u8 } else { 255u8 };
        
        dst[i] = val;
        dst[i + 1] = val;
        dst[i + 2] = val;
        dst[i + 3] = src[i + 3];
    }
}
