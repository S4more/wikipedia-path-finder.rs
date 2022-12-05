import { Canvas, CanvasRenderingContext2D, loadImage } from "canvas";

export default class Rasterizer {
    private canvas: Canvas;
    private ctx: CanvasRenderingContext2D;

    constructor(private res = 26) {
        this.canvas = new Canvas(res, res);
        this.ctx = this.canvas.getContext("2d");
    }

    reset() {
        this.ctx.globalCompositeOperation = "overlay"
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
    public async rasterize(url: string): Promise<Buffer> {
        let img;
        try {
            img = await loadImage(url);
        } catch (e) {
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
