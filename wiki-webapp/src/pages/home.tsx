import { useState } from "react";
import IconArrowRight from "../components/icons/icon-arrow-right";
import ButtonBase from "../components/inputs/button";
import ButtonSubmit from "../components/inputs/button-submit";
import NodeVisualizer from "../components/node-visualizer";
import TitleSelector from "../components/title-selector";

export default function Home() {

    const [visualizerOpen, setVisualizerOpen] = useState(false);

    return <>
        <div className={`flex w-full items-middle py-0 px-4 sm:px-6 lg:px-8 items-center ${visualizerOpen ? "h-full" : "h-screen"}`}>
            <div className="w-full flex flex-col items-center align-middle min-w-max">
                <form
                    className={
                        "px-8 py-4 backdrop-blur-md flex space-y-6 flex-row last:flex-col [&>*]:!mt-0 content-center items-center max-w-7xl last:max-w-3xl w-full z-20 bg-opacity-10 bg-black rounded-md border-gray-500 border border-opacity-50" +
                        ""} >
                    {!visualizerOpen ?
                        <h2 className="m-10 text-center text-3xl font-bold tracking-tight text-gray-900">
                            Find a path between pages
                        </h2> : undefined}

                    <div className="before:content-['From'] before:absolute before:-top-5 w-full relative grow">
                        <TitleSelector />
                    </div>

                    <IconArrowRight className={visualizerOpen ? "-rotate-90" : ""} />

                    <div className="before:content-['To'] before:absolute before:-top-5 w-full relative grow">
                        <TitleSelector />
                    </div>

                    <div className="flex ml-4">
                        <ButtonBase className="mr-4">Back</ButtonBase>
                        <ButtonSubmit type="submit" onClick={(e) => {
                            e.preventDefault();
                            setVisualizerOpen(true)
                        }}>Search</ButtonSubmit>
                    </div>
                </form>
                {visualizerOpen ? <NodeVisualizer id="canvas" className="peer" /> : undefined}
            </div>
        </div>
    </>
}