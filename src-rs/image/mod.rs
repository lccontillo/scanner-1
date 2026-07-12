use alloc::vec::Vec;

mod document;
mod downscale;
mod gaussian;
mod grayscale;
pub use document::{GradientVotesResult, Line, Point, Quad, ScoredQuad};

pub struct Image {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    pub fn downscale(&self, by: f32) -> Image {
        downscale::downscale(self, by)
    }
    pub fn gaussian(&self) -> Image {
        gaussian::gaussian(self)
    }
    // pub fn edges(&self, threshold: f32) -> Vec<Line> {
    //     let result = document::gradient_votes(self);
    //     let mut edges = document::edges(&result, threshold);
    //     edges.sort_unstable_by(|a, b| b.cmp(a));
    //     edges
    // }
    pub fn document(&self) -> Option<ScoredQuad> {
        // Second Gaussian pass for stronger noise suppression
        // (first pass was applied by the caller before calling document())
        let blurred = self.gaussian();
        let result = document::gradient_votes(&blurred);

        // Adaptive threshold: raise on noisy images where avg gradient is high
        let threshold = (0.05 + result.avg_grad * 0.03).min(0.20);

        let mut edges = document::edges(&result, threshold);
        edges.truncate(30);
        edges.sort_unstable_by(|a, b| b.cmp(a));
        document::documents(&result, &edges).get(0).copied()
    }
}

use alloc::borrow::Cow;

pub struct RGBAImage<'a> {
    pub data: Cow<'a, [u8]>,
    pub width: usize,
    pub height: usize,
}

impl<'a> RGBAImage<'a> {
    pub fn to_grayscale(&self) -> Image {
        grayscale::grayscale(self)
    }
    pub fn perspective(&self, quad: Quad, width: usize, height: usize) -> RGBAImage<'static> {
        document::perspective(self, quad, width, height)
    }
}
