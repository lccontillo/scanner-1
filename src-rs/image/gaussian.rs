use super::Image;
use alloc::vec::Vec;

// NOTE: empirical tests showed repeated box blur is roughly the same performance
// I didn't do full tests so it's something to consider for the future if this becomes a bottleneck

pub fn gaussian(source: &Image) -> Image {
    let &Image {
        data: ref src,
        width,
        height,
    } = source;

    let len = src.len();
    let mut data = Vec::with_capacity(len);
    unsafe {
        data.set_len(len);
    }

    let wm = width - 2;
    let hm = height - 2;

    // Precompute offsets once
    let e = 1usize;
    let s = width;
    let sw = s - e;
    let se = s + e;

    let e2 = e * 2;
    let s2 = s * 2;

    let sw2 = sw * 2;
    let se2 = se * 2;

    let ssw = s + sw;
    let sww = sw - e;
    let sse = s + se;
    let see = e + se;

    let w2 = width * 2;

    // Weights
    const W1: f32 = 0.01258;
    const W2: f32 = 0.02516;
    const W3: f32 = 0.03145;
    const W4: f32 = 0.0566;
    const W5: f32 = 0.07547;
    const W6: f32 = 0.09434;

    for i in 2..hm {
        let ib = i * width;

        for j in 2..wm {
            let bp = ib + j;

            unsafe {
                let c0 = *src.get_unchecked(bp - se2)
                    + *src.get_unchecked(bp - sw2)
                    + *src.get_unchecked(bp + sw2)
                    + *src.get_unchecked(bp + se2);

                let c1 = *src.get_unchecked(bp - sse)
                    + *src.get_unchecked(bp - ssw)
                    + *src.get_unchecked(bp - see)
                    + *src.get_unchecked(bp - sww)
                    + *src.get_unchecked(bp + sww)
                    + *src.get_unchecked(bp + see)
                    + *src.get_unchecked(bp + ssw)
                    + *src.get_unchecked(bp + sse);

                let c2 = *src.get_unchecked(bp - s2)
                    + *src.get_unchecked(bp - e2)
                    + *src.get_unchecked(bp + e2)
                    + *src.get_unchecked(bp + s2);

                let c3 = *src.get_unchecked(bp - se)
                    + *src.get_unchecked(bp - sw)
                    + *src.get_unchecked(bp + sw)
                    + *src.get_unchecked(bp + se);

                let c4 = *src.get_unchecked(bp - s)
                    + *src.get_unchecked(bp - e)
                    + *src.get_unchecked(bp + e)
                    + *src.get_unchecked(bp + s);

                let c5 = *src.get_unchecked(bp);

                *data.get_unchecked_mut(bp) =
                    c0 * W1 + c1 * W2 + c2 * W3 + c3 * W4 + c4 * W5 + c5 * W6;
            }
        }
    }

    // Copy side borders (unchanged logic)
    for i in 2..hm {
        let ib = i * width;
        let ibe = ib + wm;
        unsafe {
            let v = *data.get_unchecked(ib + 2);
            *data.get_unchecked_mut(ib + 1) = v;
            *data.get_unchecked_mut(ib + 0) = v;

            let v = *data.get_unchecked(ibe - 1);
            *data.get_unchecked_mut(ibe) = v;
            *data.get_unchecked_mut(ibe + 1) = v;
        }
    }

    // Copy top & bottom borders (unchanged logic)
    let hmb = hm * width;
    let hmb2 = hmb - width;
    let hmbe = hmb + width;

    for j in 0..width {
        unsafe {
            let v = *data.get_unchecked(w2 + j);
            *data.get_unchecked_mut(width + j) = v;
            *data.get_unchecked_mut(j) = v;

            let v = *data.get_unchecked(hmb2 + j);
            *data.get_unchecked_mut(hmb + j) = v;
            *data.get_unchecked_mut(hmbe + j) = v;
        }
    }

    Image {
        data,
        width,
        height,
    }
}
