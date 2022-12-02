import WasmLoader from "../util/wasm-loader";
import { useEffect, useState } from "react";

function NodeVisualizer({ id }: { id: string }) {
    const [width, setWidth] = useState(window.innerWidth);

    window.addEventListener("resize", () => {
        setWidth(window.innerWidth);
    })

    useEffect(() => {
        WasmLoader.loadWasm().then(wasm => {
            wasm.bevy(`#${id}`)
        });
    }, []);

    return (
        <div className="bg-gray-50">
            <div className="mx-auto max-w-7xl py-12 px-4 sm:px-6 lg:flex lg:items-center lg:justify-between lg:py-16 lg:px-8">
                <div className="mt-8 flex lg:mt-0 lg:flex-shrink-0">
                    <canvas id={id} width={width}></canvas>
                </div>
            </div>
        </div>
    )
}

export default NodeVisualizer;