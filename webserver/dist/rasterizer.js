"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const canvas_1 = require("canvas");
class Rasterizer {
    constructor(res = 26) {
        this.res = res;
        this.canvas = new canvas_1.Canvas(res, res);
        this.ctx = this.canvas.getContext("2d");
    }
    reset() {
        this.ctx.globalCompositeOperation = "overlay";
        this.ctx.restore();
        this.ctx.clearRect(0, 0, this.res, this.res);
    }
    clip() {
        this.ctx.globalCompositeOperation = 'destination-in';
        this.ctx.fillStyle = "#fff";
        this.ctx.beginPath();
        this.ctx.arc(this.res / 2, this.res / 2, this.res / 2, 0, 2 * Math.PI, true);
        this.ctx.closePath();
        this.ctx.fill();
    }
    // Renders a given image from a url, and returns a base64 encoded version
    async rasterize(url) {
        let img;
        try {
            img = await (0, canvas_1.loadImage)(url);
        }
        catch (e) {
            console.log(e);
        }
        if (img) {
            this.reset();
            this.ctx.drawImage(img, 0, 0, this.res, this.res);
            this.clip();
            return this.canvas.toBuffer();
        }
        return new Buffer("");
    }
}
exports.default = Rasterizer;
