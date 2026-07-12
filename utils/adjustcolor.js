export function adjustContext() {
  let canvas = document.createElement("canvas");
  let ctx = canvas.getContext("2d");

  const sharpen = (data, mix) => {
    //mix 0.1 - 100
    let w = data.width,
      h = data.height;
    var x,
      sx,
      sy,
      r,
      g,
      b,
      a,
      dstOff,
      srcOff,
      wt,
      cx,
      cy,
      scy,
      scx,
      weights = [0, -1, 0, -1, 5, -1, 0, -1, 0],
      katet = Math.round(Math.sqrt(weights.length)),
      half = (katet * 0.5) | 0,
      dstData = ctx.createImageData(w, h),
      dstBuff = dstData.data,
      srcBuff = data.data,
      y = h;
    while (y--) {
      x = w;
      while (x--) {
        sy = y;
        sx = x;
        dstOff = (y * w + x) * 4;
        r = 0;
        g = 0;
        b = 0;
        a = 0;
        if (x > 0 && y > 0 && x < w - 1 && y < h - 1) {
          for (cy = 0; cy < katet; cy++) {
            for (cx = 0; cx < katet; cx++) {
              scy = sy + cy - half;
              scx = sx + cx - half;

              if (scy >= 0 && scy < h && scx >= 0 && scx < w) {
                srcOff = (scy * w + scx) * 4;
                wt = weights[cy * katet + cx];

                r += srcBuff[srcOff] * wt;
                g += srcBuff[srcOff + 1] * wt;
                b += srcBuff[srcOff + 2] * wt;
                a += srcBuff[srcOff + 3] * wt;
              }
            }
          }

          dstBuff[dstOff] = r * mix + srcBuff[dstOff] * (1 - mix);
          dstBuff[dstOff + 1] = g * mix + srcBuff[dstOff + 1] * (1 - mix);
          dstBuff[dstOff + 2] = b * mix + srcBuff[dstOff + 2] * (1 - mix);
          dstBuff[dstOff + 3] = srcBuff[dstOff + 3];
        } else {
          dstBuff[dstOff] = srcBuff[dstOff];
          dstBuff[dstOff + 1] = srcBuff[dstOff + 1];
          dstBuff[dstOff + 2] = srcBuff[dstOff + 2];
          dstBuff[dstOff + 3] = srcBuff[dstOff + 3];
        }
      }
    }

    /*     ctx.putImageData(dstData, 0, 0); */
    return dstData;
  };

  const saturate = (data, mix) => {
    let imageX = 0;
    let imageY = 0;
    let imageW = data.width;
    let imageH = data.height;
    //https://www.qoncious.com/questions/changing-saturation-image-html5-canvas-using-javascript
    /*     var imageData = context.getImageData(imageX, imageY, imageW, imageH); */
    var dA = data.data; // raw pixel data in array

    var sv = mix; // saturation value. 0 = grayscale, 1 = original

    var luR = 0.3086; // constant to determine luminance of red. Similarly, for green and blue
    var luG = 0.6094;
    var luB = 0.082;

    var az = (1 - sv) * luR + sv;
    var bz = (1 - sv) * luG;
    var cz = (1 - sv) * luB;
    var dz = (1 - sv) * luR;
    var ez = (1 - sv) * luG + sv;
    var fz = (1 - sv) * luB;
    var gz = (1 - sv) * luR;
    var hz = (1 - sv) * luG;
    var iz = (1 - sv) * luB + sv;

    for (var i = 0; i < dA.length; i += 4) {
      var red = dA[i]; // Extract original red color [0 to 255]. Similarly for green and blue below
      var green = dA[i + 1];
      var blue = dA[i + 2];

      var saturatedRed = az * red + bz * green + cz * blue;
      var saturatedGreen = dz * red + ez * green + fz * blue;
      var saturateddBlue = gz * red + hz * green + iz * blue;

      dA[i] = saturatedRed;
      dA[i + 1] = saturatedGreen;
      dA[i + 2] = saturateddBlue;
    }
    //
    // context.putImageData(imageData, imageX, imageY);
    return data;
  };

  const brighten = (imageData, mix) => {
    // var imageData = context.getImageData(
    //   0,
    //   0,
    //   context.canvas.width,
    //   context.canvas.height
    // );
    var brightness = mix;
    var data = imageData.data;

    for (var i = 0; i < data.length; i += 4) {
      data[i] += 255 * (brightness / 100);
      data[i + 1] += 255 * (brightness / 100);
      data[i + 2] += 255 * (brightness / 100);
    }
    //
    // context.putImageData(imageData, 0, 0);
    //
    return imageData;
  };

  function truncateColor(value) {
    if (value < 0) {
      value = 0;
    } else if (value > 255) {
      value = 255;
    }

    return value;
  }
  const contrast = (imageData, mix) => {
    // var imageData = context.getImageData(
    //   0,
    //   0,
    //   context.canvas.width,
    //   context.canvas.height
    // );
    var contrast = mix;
    var data = imageData.data;

    var factor = (259.0 * (contrast + 255.0)) / (255.0 * (259.0 - contrast));

    for (var i = 0; i < data.length; i += 4) {
      data[i] = truncateColor(factor * (data[i] - 128.0) + 128.0);
      data[i + 1] = truncateColor(factor * (data[i + 1] - 128.0) + 128.0);
      data[i + 2] = truncateColor(factor * (data[i + 2] - 128.0) + 128.0);
    }

    /*     context.putImageData(imageData, 0, 0); */
    return imageData;
  };
  const satval = (val) => {
    console.log(val / 100 + 1);
    return val / 100 + 1;
  };

  const adjustColors = (context, props) => {
    var imageData = context.getImageData(
      0,
      0,
      context.canvas.width,
      context.canvas.height
    );

    if (props.b != 0) imageData = brighten(imageData, props.b);
    if (props.c != 0) imageData = contrast(imageData, props.c);
    if (props.s != 0) imageData = saturate(imageData, satval(props.s));
    if (props.sh != 0) imageData = sharpen(imageData, props.sh);

    context.putImageData(imageData, 0, 0);
  };

  return { adjustColors };
}
