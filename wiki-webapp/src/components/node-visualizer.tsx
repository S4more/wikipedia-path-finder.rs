import WasmLoader from "../util/wasm-loader";
import { useEffect, useState, PropsWithoutRef } from "react";

function NodeVisualizer(props: PropsWithoutRef<{ id: string, nodes: string[], className: string }>) {
    const [width, setWidth] = useState(window.innerWidth);

    window.addEventListener("resize", () => {
        setWidth(window.innerWidth);
    })

    useEffect(() => {
        if (props.nodes.length > 0) {
            WasmLoader.loadWasm().then(wasm => {
                console.log(props.nodes.join(","));
                wasm.bevy(`#${props.id}`, props.nodes.join(","))
            });
        }
    }, [props.nodes]);

    return (
        <div className="w-screen h-screen absolute top-0 left-0">
            <canvas {...props} ></canvas>
        </div>
    )
}

export default NodeVisualizer;