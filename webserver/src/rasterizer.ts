import { Canvas, CanvasRenderingContext2D, loadImage } from "canvas";

export default class Rasterizer {
    private canvas: Canvas;
    private ctx: CanvasRenderingContext2D;

    constructor(private res = 26) {
        this.canvas = new Canvas(res, res);
        this.ctx = this.canvas.getContext("2d");
    }

    // Renders a given image from a url, and returns a base64 encoded version
    public async rasterize(url: string): Promise<Buffer> {
        return new Promise((res, rej) => {
            // loadImage(url).then(img => {
            let start = performance.now();
            this.ctx.clearRect(0, 0, this.res, this.res);
            this.ctx.beginPath();
            this.ctx.fillStyle = "#" + `${((Math.random() * 0xFFF) | 0).toString(16)}`.padStart(3, "0");

            this.ctx.ellipse(this.res / 2, this.res / 2, this.res / 2, this.res / 2, 0, 0, 360);
            this.ctx.closePath();
            this.ctx.fill();

            // this.ctx.drawImage(img, 0, 0, this.res, this.res);
            res(this.canvas.toBuffer());
            console.log(performance.now() - start);
            // }).catch(rej);
        })
    }
}