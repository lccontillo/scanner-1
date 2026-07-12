function contrastImage(imageData, contrast) {
  // contrast input as percent; range [-1..1]
  var data = imageData.data; // Note: original dataset modified directly!
  contrast *= 255;
  var factor = (contrast + 255) / (255.01 - contrast); //add .1 to avoid /0 error.

  for (var i = 0; i < data.length; i += 4) {
    data[i] = factor * (data[i] - 128) + 128;
    data[i + 1] = factor * (data[i + 1] - 128) + 128;
    data[i + 2] = factor * (data[i + 2] - 128) + 128;

    //
    // const red = data[i];
    // const green = data[i + 1];
    // const blue = data[i + 2];
    // const grayscale = 0.3 * red + 0.59 * green + 0.11 * blue;

    //Desaturate the blue channel
    //data[i + 2] = grayscale;
  }
  return imageData; //optional (e.g. for filter function chaining)
}

function setImageData(
  imageData,
  targetColor,
  newColorSelected,
  newColorUnselected,
  tolerance
) {
  const currentPixelColor = { r: null, g: null, b: null };
  const pixels = imageData.data;

  const isMask =
    (Math.abs(newColorSelected.r - newColorUnselected.r) +
      Math.abs(newColorSelected.g - newColorUnselected.g) +
      Math.abs(newColorSelected.b - newColorUnselected.b)) /
      3 ===
    255;

  // loop through imageData
  for (let i = 0, n = imageData.data.length; i < n; i += 4) {
    currentPixelColor.r = imageData.data[i];
    currentPixelColor.g = imageData.data[i + 1];
    currentPixelColor.b = imageData.data[i + 2];

    const diff =
      (Math.abs(currentPixelColor.r - targetColor.r) +
        Math.abs(currentPixelColor.g - targetColor.g) +
        Math.abs(currentPixelColor.b - targetColor.b)) /
      3;

    if (diff > tolerance) {
      pixels[i] = newColorUnselected.r;
      pixels[i + 1] = newColorUnselected.g;
      pixels[i + 2] = newColorUnselected.b;
      pixels[i + 3] =
        newColorSelected.a === 0 && isMask ? diff : newColorUnselected.a;
    } else {
      pixels[i] = newColorSelected.r;
      pixels[i + 1] = newColorSelected.g;
      pixels[i + 2] = newColorSelected.b;
      pixels[i + 3] = newColorSelected.a;
    }
  }
}

// the following is from https://stackoverflow.com/a/5624139/2630316
function hexToRgb(hex) {
  var shorthandRegex = /^#?([a-f\d])([a-f\d])([a-f\d])$/i;
  hex = hex.replace(shorthandRegex, function (m, r, g, b) {
    return r + r + g + g + b + b;
  });

  var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
  return result
    ? {
        r: parseInt(result[1], 16),
        g: parseInt(result[2], 16),
        b: parseInt(result[3], 16),
      }
    : null;
}

function loadImage(url) {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.src = url;
    img.addEventListener("load", () => resolve(img));
    img.addEventListener("error", () => {
      reject(new Error(`Failed to load image ${url}`));
    });
  });
}

function grayscale(imgData) {
  let pixels = imgData.data;
  for (var i = 0; i < pixels.length; i += 4) {
    let lightness = parseInt((pixels[i] + pixels[i + 1] + pixels[i + 2]) / 3);

    pixels[i] = lightness;
    pixels[i + 1] = lightness;
    pixels[i + 2] = lightness;
  }

  return imgData;
}

function grayCanvas(img) {
  const w = img.naturalWidth;
  const h = img.naturalHeight;
  var gray = document.createElement("canvas");
  let gctx = gray.getContext("2d");
  gray.width = w;
  gray.height = h;
  gctx.drawImage(img, 0, 0, w, h);
  var grayBits = gctx.getImageData(0, 0, w, h);
  grayscale(grayBits);
  gctx.putImageData(grayBits, 0, 0);

  return gray;
}

const sharpen = (ctx, mix) => {
  //mix 0.1 - 100
  var h = ctx.canvas.height;
  var w = ctx.canvas.width;

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
    srcBuff = ctx.getImageData(0, 0, w, h).data,
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

function contrastCanvas(img, type) {
  const w = img.naturalWidth;
  const h = img.naturalHeight;
  var orig = document.createElement("canvas");
  let ctxOrig = orig.getContext("2d");
  orig.width = w;
  orig.height = h;
  ctxOrig.drawImage(img, 0, 0, w, h);
  var origBits;
  // if (type == 'bw') {
  //   origBits = ctxOrig.getImageData(0, 0, w, h);
  origBits = sharpen(ctxOrig, 0.6);
  contrastImage(origBits, 0.25);
  ctxOrig.putImageData(origBits, 0, 0);

  return orig;
}

function photoCanvas(img) {
  const w = img.naturalWidth;
  const h = img.naturalHeight;
  var orig = document.createElement("canvas");
  let ctxOrig = orig.getContext("2d");
  orig.width = w;
  orig.height = h;
  ctxOrig.drawImage(img, 0, 0, w, h);
  var origBits;
  origBits = ctxOrig.getImageData(0, 0, w, h);
  contrastImage(origBits, 0.18);
  ctxOrig.putImageData(origBits, 0, 0);

  return orig;
}

function rgbCanvas(config, orig) {
  const canvas1 = document.createElement("canvas");
  canvas1.width = orig.width;
  canvas1.height = orig.height;

  let ctx = canvas1.getContext("2d");
  ctx.drawImage(orig, 0, 0);

  let imageData = ctx.getImageData(0, 0, canvas1.width, canvas1.height);

  setImageData(
    imageData,
    hexToRgb(config.targetColor),
    config.newColorSelected,
    config.newColorUnselected,
    config.tolerance
  );
  ctx.putImageData(imageData, 0, 0);

  return canvas1;
}

function blurCanvas(config, canvas1) {
  console.log("ccc", canvas1);
  const canvas2 = document.createElement("canvas");
  canvas2.width = canvas1.width;
  canvas2.height = canvas1.height;
  const ctx = canvas2.getContext("2d");
  ctx.filter = `blur( ${config.edgeBlur}px ) brightness( ${config.edgeBrightness} ) contrast( ${config.edgeContrast} )`;
  ctx.drawImage(canvas1, 0, 0);

  return canvas2;
}

function replacedCanvas(config, canvas1, canvas2) {
  const canvas3 = document.createElement("canvas");
  canvas3.width = canvas1.width;
  canvas3.height = canvas1.height;
  let ctx = canvas3.getContext("2d");
  ctx.drawImage(canvas2, 0, 0);
  let imageData = ctx.getImageData(0, 0, canvas3.width, canvas3.height);
  setImageData(
    imageData,
    { r: 255, g: 255, b: 255 },
    { ...config.newColorSelected, a: 0 },
    config.newColorUnselected,
    10
  );
  ctx.putImageData(imageData, 0, 0);

  return canvas3;
}

function magicColor(orig, canvas3) {
  const canvasDisplay = document.createElement("canvas");
  canvasDisplay.width = canvas3.width;
  canvasDisplay.height = canvas3.height;
  let ctx = canvasDisplay.getContext("2d");

  ctx.fillStyle = "red";
  ctx.fillRect(0, 0, canvas3.width, canvas3.height);
  ctx.drawImage(canvas3, 0, 0);
  ctx.globalCompositeOperation = "source-in";
  ctx.drawImage(orig, 0, 0); //magic color

  return canvasDisplay;
}

function bwCanvas(canvas3) {
  const canvasReplacementColor = document.createElement("canvas");
  canvasReplacementColor.width = canvas3.width;
  canvasReplacementColor.height = canvas3.height;
  let ctx = canvasReplacementColor.getContext("2d");
  ctx.fillStyle = "white";
  ctx.fillRect(0, 0, canvas3.width, canvas3.height);
  ctx.drawImage(canvas3, 0, 0);

  return canvasReplacementColor;
}
const CanvasMagicWand = {
  async knockoutColor(config = {}) {
    let { img = null } = config;

    const {
      src = null,
      targetColor = "#fff",
      replacementColor = "#ffffff00",
      tolerance = 10,
      edgeBlur = 4,
      edgeBrightness = 2,
      edgeContrast = 6,
      debugMode = false,
      signColor = { r: 0, g: 0, b: 0, a: 255 },
      type = "magic",
    } = config;

    if (src !== null || (img !== null && img.naturalWidth === 0)) {
      if (img === null && typeof src !== "string") {
        throw Error("src must be a string");
      }

      try {
        img = await loadImage(src || img.src);
      } catch (err) {
        console.error(err);
      }
    } else if (!(img instanceof Image)) {
      throw Error(
        "must provide a HTMLImageElement or a String representing the image's src attribute"
      );
    }

    const w = img.naturalWidth;
    const h = img.naturalHeight;

    const newColorSelected = { r: 255, g: 255, b: 255, a: 255 };
    const newColorUnselected = signColor || { r: 0, g: 0, b: 0, a: 255 };

    var config = {
      targetColor: targetColor,
      newColorSelected: newColorSelected,
      newColorUnselected: newColorUnselected,
      tolerance: tolerance,
      edgeBlur: edgeBlur,
      edgeBrightness: edgeBrightness,
      edgeContrast: edgeContrast,
    };

    let result;

    var orig = contrastCanvas(img, type);
    var canvas1, canvas2, canvas3;
    switch (type) {
      case "original":
        result = orig;
        break;
      case "magic":
        /*   canvas1 = await rgbCanvas(config, orig); */
        // canvas2 = await blurCanvas(config, canvas1);
        // canvas3 = await replacedCanvas(config, canvas1, canvas2);
        // // //
        // // result = await magicColor(orig, canvas3);
        // const canvasDisplay = document.createElement('canvas');
        // canvasDisplay.width = w;
        // canvasDisplay.height = h;
        // let ctx = canvasDisplay.getContext('2d');
        // ctx.drawImage(canvas3, 0, 0);
        // ctx.globalCompositeOperation = 'source-in';
        // ctx.drawImage(orig, 0, 0);
        //
        // const canvasReplacementColor = document.createElement('canvas');
        // canvasReplacementColor.width = w;
        // canvasReplacementColor.height = h;
        // ctx = canvasReplacementColor.getContext('2d');
        // ctx.fillStyle = debugMode ? '#fff' : replacementColor;
        // ctx.fillRect(0, 0, w, h);
        // ctx.drawImage(debugMode ? canvas3 : canvasDisplay, 0, 0);

        result = orig;
        break;
      case "gray":
        result = await grayCanvas(img);
        break;
      case "photo":
        result = await photoCanvas(img);
        break;
      case "bw":
        canvas1 = await rgbCanvas(config, orig);
        /*         canvas2 = await blurCanvas(config, canvas1); */
        // canvas3 = await replacedCanvas(config, canvas1, canvas2);
        //
        // result = await bwCanvas(canvas3);
        result = canvas1;
        break;
      case "sign":
        canvas1 = await rgbCanvas(config, orig);
        canvas2 = await blurCanvas(config, canvas1);
        canvas3 = await replacedCanvas(config, canvas1, canvas2);

        result = canvas3;
        break;
    }

    return result;
  },
};

export default CanvasMagicWand;
