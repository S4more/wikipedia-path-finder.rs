import { useEffect, useState } from "react";
import IconArrowRight from "../components/icons/icon-arrow-right";
import ButtonBase from "../components/inputs/button";
import ButtonSubmit from "../components/inputs/button-submit";
import NodeVisualizer from "../components/node-visualizer";
import PathList from "../components/path-list";
import TitleSelector from "../components/title-selector";
import { findPath } from "../util/path-api";

const cleanUrl = (str: string) =>
    str.replaceAll("%20", " ");

export default function Home() {
    const [error, setError] = useState("");
    const [visualizerOpen, setVisualizerOpen] = useState(false);
    const href = window.location.href.split("/");
    let urlParams = false;

    const doSearch = (from: string, to: string) => {
        findPath(from, to)
            .then(path => {
                setPath(path);
                setVisualizerOpen(true);
            }).catch(e => {
                setError("Could not find path");
                console.log("Could not find path");
            }).finally(() => console.log("Done Search"))
    }

    let [_to, _from] = ["", ""]
    if (href.includes("path")) {
        urlParams = true;
        [_to, _from] = [href.pop()!, href.pop()!].map(cleanUrl);
    }

    const [from, setFrom] = useState(_from);
    const [to, setTo] = useState(_to);
    const [path, setPath] = useState<string[]>([]);

    useEffect(() => {
        if (urlParams)
            doSearch(from, to);
    }, [])

    return <>
        <div className="main_container h-screen">
            {visualizerOpen ? <NodeVisualizer id="canvas" nodes={path} className="peer" /> : ""}
            {visualizerOpen ? <PathList path={path} /> : ""}
            <form className="shadow-md">
                {!visualizerOpen ?
                    <h2 className="m-10 text-center text-3xl font-bold text-gray-900">
                        Find a path between pages
                    </h2> : undefined}
                <div className="before:content-['From'] before:absolute before:-top-5 w-full relative grow">
                    <TitleSelector value={from} titleChange={setFrom} />
                </div>

                <IconArrowRight className={visualizerOpen ? "-rotate-90" : ""} />

                <div className="before:content-['To'] before:absolute before:-top-5 w-full relative grow">
                    <TitleSelector value={to} titleChange={setTo} />
                </div>

                <div className="flex ml-4">
                    <ButtonBase className="mr-4">Back</ButtonBase>
                    <ButtonSubmit type="submit" onClick={(e) => {
                        e.preventDefault();
                        window.location.pathname = (`/path/${from}/${to}`)
                    }}>Search</ButtonSubmit>
                </div>
                {error && <code className="text-red-600">{error}</code>}
            </form>
        </div>
    </>
}