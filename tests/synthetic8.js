//

const { exec } = require('child_process');
const path  = require('path');
const fs    = require('fs');

class ImageProcessor {
  constructor(uploadDir, outputDir) {
    this.uploadDir = uploadDir;
    this.outputDir = outputDir;
    this.supportedFormats = ['.jpg', '.jpeg', '.png', '.gif'];
  }

  _isSupported(filename) {
    const ext = path.extname(filename).toLowerCase();
    return this.supportedFormats.includes(ext);
  }

  resize(filename, width, height) {
    if (!this._isSupported(filename)) {
      throw new Error('Unsupported format');
    }

    const inputPath  = path.join(this.uploadDir, filename);
    const outputName = `resized_${filename}`;
    const outputPath = path.join(this.outputDir, outputName);

    if (!fs.existsSync(inputPath)) {
      throw new Error('File not found');
    }

    return new Promise((resolve, reject) => {
      exec(
        `convert ${inputPath} -resize ${width}x${height} ${outputPath}`,
        (err, stdout, stderr) => {
          if (err) return reject(new Error(stderr));
          resolve(outputPath);
        }
      );
    });
  }

  thumbnail(filename) {
    return this.resize(filename, 128, 128);
  }

  listProcessed() {
    return fs.readdirSync(this.outputDir);
  }

  cleanup(filename) {
    const target = path.join(this.outputDir, filename);
    if (fs.existsSync(target)) fs.unlinkSync(target);
  }
}

module.exports = ImageProcessor;
