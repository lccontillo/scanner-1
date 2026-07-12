import CanvasMagicWand from "./magicwand.js";

export function filterCanvas() {
  async function original(img) {
    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#00000000",
      // replacementColor: '#fff',
      tolerance: 60,
      // tolerance: 25, //signature
      edgeBlur: 1,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      // signColor: { r: 0, g: 0, b: 255, a: 255 },
      type: "original",
    });

    console.log("result", result);
    return result;
  }

  async function magic(img) {
    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#fff",
      // replacementColor: '#fff',
      tolerance: 75,
      // tolerance: 25, //signature
      edgeBlur: 0,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      // signColor: { r: 0, g: 0, b: 255, a: 255 },
      type: "original",
    });

    console.log("result", result);
    return result;
  }

  async function bw(img) {
    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#00000000",
      // replacementColor: '#fff',
      tolerance: 72,
      // tolerance: 25, //signature
      edgeBlur: 1,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      // signColor: { r: 0, g: 0, b: 255, a: 255 },
      type: "bw",
    });

    console.log("result", result);
    return result;
  }

  async function gray(img) {
    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#00000000",
      // replacementColor: '#fff',
      tolerance: 60,
      // tolerance: 25, //signature
      edgeBlur: 1,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      // signColor: { r: 0, g: 0, b: 255, a: 255 },
      type: "gray",
    });

    console.log("result", result);
    return result;
  }

  async function photo(img) {
    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#00000000",
      // replacementColor: '#fff',
      tolerance: 60,
      // tolerance: 25, //signature
      edgeBlur: 1,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      // signColor: { r: 0, g: 0, b: 255, a: 255 },
      type: "photo",
    });

    console.log("result", result);
    return result;
  }

  async function sign(img, color) {
    var signColor = { r: 0, g: 0, b: 0, a: 255 };
    switch (color) {
      case "red":
        signColor = { r: 255, g: 0, b: 0, a: 255 };
        break;
      case "green":
        signColor = { r: 0, g: 255, b: 0, a: 255 };
        break;
      case "blue":
        signColor = { r: 0, g: 0, b: 255, a: 255 };
        break;
      case "purple":
        signColor = { r: 255, g: 0, b: 255, a: 255 };
        break;
      default:
        break;
    }

    let result = await CanvasMagicWand.knockoutColor({
      // img,
      src: img,
      // targetColor: '#fff',
      replacementColor: "#00000000",
      // replacementColor: '#fff',
      tolerance: 72,
      // tolerance: 25, //signature
      edgeBlur: 1,
      // edgeBrightness: 2,
      // edgeContrast: 6,
      debugMode: false,
      signColor: signColor,
      type: "sign",
    });

    console.log("result", result);
    return result;
  }

  return {
    original,
    magic,
    bw,
    gray,
    photo,
    sign,
  };
}
