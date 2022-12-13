import { useEffect, useState } from "react";

export default function MapView() {
    const [imgUrl, setImgUrl] = useState("");

    useEffect(() => {
        let stop = false;

        const update = async () => {
            if (stop) return;
            let result = await (fetch("/path-image"));
            let text = await result.text();
            setImgUrl(text);
            setTimeout(() => update(), 250)
        }

        update()

        return () => {
            stop = true;
        }
    }, []);

    return <img style={{ "imageRendering": "pixelated" }} className="h-fit" src={imgUrl} />;
}