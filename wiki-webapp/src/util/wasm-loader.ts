import * as wasm from "node-visualizer"

export default class WasmLoader {
    private static wasmInstance: Promise<typeof wasm>;
    public static loadWasm(): Promise<typeof wasm> {
        if (this.wasmInstance != undefined) {
            return this.wasmInstance;
        } else {
            return WasmLoader.wasmInstance = new Promise<typeof wasm>(res => {
                console.warn("Loading Wasm")
                wasm.default().then(() => {
                    console.log("Wasm Loaded");
                    res(wasm);
                })
            });
        }
    }
}