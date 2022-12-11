import { useState } from "react";
import IconArrowRight from "../components/icons/icon-arrow-right";
import ButtonBase from "../components/inputs/button";
import ButtonSubmit from "../components/inputs/button-submit";
import NodeVisualizer from "../components/node-visualizer";
import PathList from "../components/path-list";
import TitleSelector from "../components/title-selector";
import { findPath } from "../util/path-api";

export default function Home() {

    const [visualizerOpen, setVisualizerOpen] = useState(false);
    const [from, setFrom] = useState("");
    const [to, setTo] = useState("");

    const [path, setPath] = useState<string[]>([]);

    const doSearch = () => {
        setVisualizerOpen(true);

        findPath(from, to).then(path => {
            setPath(path);
            setVisualizerOpen(true);
        });
    }

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
                    <TitleSelector titleChange={setFrom} />
                </div>

                <IconArrowRight className={visualizerOpen ? "-rotate-90" : ""} />

                <div className="before:content-['To'] before:absolute before:-top-5 w-full relative grow">
                    <TitleSelector titleChange={setTo} />
                </div>

                <div className="flex ml-4">
                    <ButtonBase className="mr-4">Back</ButtonBase>
                    <ButtonSubmit type="submit" onClick={(e) => {
                        e.preventDefault();
                        doSearch();
                    }}>Search</ButtonSubmit>
                </div>
            </form>
        </div>
    </>
}