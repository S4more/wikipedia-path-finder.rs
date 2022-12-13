import { Canvas } from "canvas";

async function rawPathFetch(from: number, to: number, hops: number): Promise<string> {
    return await (await fetch(`http://localhost:8000/id/${from}/${to}/${hops}`)).text();
}

let colors = [
    "#12ff00",
    "#00d5d8",
    "#009bff",
    "#0006ff",
    "#dc00ad",
    "#ff0058",
    "#ff0000",
    "#ff0000"
]

interface PathResult {
    path: string[],
    hops: number,
}

export async function findPath(from: number, to: number): Promise<PathResult> {
    for (let i = 1; i < 8; i++) {
        let path = await rawPathFetch(from, to, i);
        if (path.charAt(0) == "[") {
            return { path: JSON.parse(path), hops: i }
        }
    }
    throw "";
}

export default class PathMap {
    private canvas: Canvas;
    private ctx: CanvasRenderingContext2D;

    private x = 0;
    private y = 0;

    private currentScale = 0;

    constructor(private scale = 10) {
        let resolution = 2 ** scale;
        this.canvas = new Canvas(resolution, resolution);
        this.ctx = this.canvas.getContext("2d");
    }

    getImage(): string {
        return this.canvas.toDataURL()
    }

    blit(x: number, y: number, color: string) {
        const currentRes = 2 ** this.currentScale;
        const maxRes = 2 ** this.scale;
        const factor = (maxRes / currentRes);
        this.ctx.fillStyle = color;
        this.ctx.fillRect(x * factor, y * factor, factor, factor);
    }

    async pollPoint(x: number, y: number) {
        const currentRes = 2 ** this.currentScale;
        const maxRes = 2 ** this.scale;
        const factor = (maxRes / currentRes);

        try {
            const path = await findPath(x * factor, y * factor);
            this.blit(x, y, colors[path.hops - 1]);
        } catch {
            this.blit(x, y, "blue");
        }
    }

    async pollOnce() {
        if (this.x >= 2 ** this.currentScale) {
            this.x = 0;
            this.y++;
        }

        if (this.y >= 2 ** this.currentScale) {
            this.currentScale += 1;
            this.y = 0;
            if (this.currentScale > this.scale)
                return;
        }

        await this.pollPoint(this.x, this.y);
        this.x++;
    }

    async pollForever() {
        let start = performance.now();
        // Try not to block for too long 
        while (performance.now() - start < 50) {
            await this.pollOnce();
        }
        if (this.currentScale > this.scale) return;
        setTimeout(() => this.pollForever());
    }

}
