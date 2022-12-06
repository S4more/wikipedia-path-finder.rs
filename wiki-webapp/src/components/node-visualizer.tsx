import WasmLoader from "../util/wasm-loader";
import { useEffect, useState, PropsWithoutRef } from "react";

function NodeVisualizer(props: PropsWithoutRef<{ id: string, className: string }>) {
    const [width, setWidth] = useState(window.innerWidth);

    window.addEventListener("resize", () => {
        setWidth(window.innerWidth);
    })

    useEffect(() => {
        WasmLoader.loadWasm().then(wasm => {
            wasm.bevy(`#${props.id}`)
        });
    }, []);

    return (
        <div className="w-screen h-screen absolute top-0 left-0">
            <canvas {...props} ></canvas>
        </div>
    )
}

export default NodeVisualizer;