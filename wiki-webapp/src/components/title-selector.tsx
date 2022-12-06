import { MouseEventHandler, PropsWithChildren, useEffect, useRef, useState } from "react"

export interface TitleSelectorProps {

}

const style_thinScroll = "scrollbar-thin scrollbar-thumb-rounded scrollbar-track-rounded-md scrollbar-track-gray-100 scrollbar-thumb-slate-300";

export interface TitleSelectorItemProps {
    title: string,
    isOpen: boolean,
    onSelect: (title: string) => void
}

function TitleSelectorItem(props: PropsWithChildren<TitleSelectorItemProps>): JSX.Element {
    let item_ref = useRef<HTMLButtonElement>(null);

    const click = (e: any) => {
        e.preventDefault();
        item_ref.current?.blur();
        props.onSelect(props.title);
    }

    return <button
        ref={item_ref}
        onClick={click}
        tabIndex={props.isOpen ? undefined : -1}
        className="flex text-sm px-3 py-2 hover:bg-blue-200 focus:bg-blue-200 focus:outline-none bg-white" >
        {props.title}
    </button>
}

export default function TitleSelector(props: TitleSelectorProps) {
    const [isOpen, setOpen] = useState(false);
    const [inputText, setInputText] = useState("");
    const [dropHeight, setDropHeight] = useState("0");

    const doFocus = () => {
        open();
    };

    const doBlur = (e: any) => {
        if (!e.currentTarget.contains(e.relatedTarget)) {
            close();
        }
    };

    let timeout = 0;

    const [suggestedTitles, setSuggestedTitles] = useState([
        "Securitas depot robbery",
        "Tonbridge",
        "Tonbridge and Malling",
        "Malling Abbey",
        "Curzon Park Abbey",
        "List of monastic houses in Cheshire"
    ]);

    const selectItem = (title: string) => {
        setInputText(title);
    }

    const close = () => {
        clearTimeout(timeout);
        setDropHeight("0");
        setOpen(false);
    }

    const open = () => {
        clearTimeout(timeout);
        setOpen(true);
        setDropHeight("10");
    }

    return <div
        onBlurCapture={doBlur}
        onFocusCapture={() => doFocus()}
        className="relative group shadow-sm focus:outline-none rounded-md h-fit border-gray-300 border p-0 z-10">
        <span className="flex items-center">
            <input
                type="text"
                value={inputText}
                onChange={(e) => setInputText(e.target.value)}
                className={`block w-full ${isOpen ? "rounded-t-md" : "rounded-md"} focus:ring-0 border-none sm:text-sm`}
            />

            <span className="-ml-8 h-6 flex justify-center pointer-events-none">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 15L12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" />
                </svg>
            </span>
        </span>

        <div style={{
            height: `${dropHeight}em`,
        }} className={`transition-spacing rounded-b-md overflow-y-scroll flex flex-col cursor-pointer absolute w-full ring-1 outline-1 ${isOpen ? "ring-gray-300" : "ring-transparent"}  ` + style_thinScroll}>
            {
                suggestedTitles.map(
                    (title: string) => <TitleSelectorItem isOpen={isOpen} key={title} onSelect={selectItem} title={title} />
                )
            }
        </div>
    </div >
}