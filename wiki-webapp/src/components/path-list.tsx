import { ComponentProps, PropsWithChildren } from "react";

import IconArrowRight from "./icons/icon-arrow-right";


export default function PathList({ path }: PropsWithChildren<{ path: string[] }>) {
    return <div className="path_list overflow-y-auto">
        {
            path.map((page, index) => <>
                <a
                    href={`https://en.wikipedia.org/wiki/${page.replaceAll(" ", "_")}`}
                    className="rounded-md cursor-pointer bold m-2 bg-white hover:bg-teal-100 px-6 py-2 shadow-sm"
                >
                    {page}
                </a>
                {index != path.length - 1 ? <IconArrowRight /> : ""}
            </>)
        }
    </div >
}