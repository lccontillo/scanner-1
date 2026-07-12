const express = require("express");
const fs = require("fs");
const path = require("path");
const sharp = require("sharp");

const app = express();
const PORT = 3000;

// Serve raw image file
app.get("/image/:name", (req, res) => {
  const imagePath = path.join(__dirname, "images", req.params.name);

  if (!fs.existsSync(imagePath)) {
    return res.status(404).json({ error: "Image not found" });
  }

  res.sendFile(imagePath);
});

// Return RGBA raw data
app.get("/image/:name/rgba", async (req, res) => {
  const imagePath = path.join(__dirname, "images", req.params.name);

  if (!fs.existsSync(imagePath)) {
    return res.status(404).json({ error: "Image not found" });
  }

  try {
    const image = sharp(imagePath);
    const { data, info } = await image
      .raw()
      .toBuffer({ resolveWithObject: true });

    res.json({
      width: info.width,
      height: info.height,
      channels: info.channels, // should be 4 (RGBA)
      rgba: Array.from(data), // convert Buffer → array of integers
    });
  } catch (err) {
    console.error(err);
    res.status(500).json({ error: "Failed to process image" });
  }
});

app.listen(PORT, () => {
  console.log(`Server running on http://localhost:${PORT}`);
});
