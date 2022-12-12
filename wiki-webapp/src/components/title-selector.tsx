import { PropsWithChildren, useEffect, useRef, useState } from "react"
import io from 'socket.io-client';
export interface TitleSelectorProps {
    titleChange: (title: string) => void,
    value: string,
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
        className="text-sm px-3 py-2 h-[36px] hover:bg-blue-200 focus:bg-blue-200 focus:outline-none w-full bg-white overflow-ellipsis text-start" >
        {props.title}
    </button>
}

const socket = io();
export default function TitleSelector(props: TitleSelectorProps) {
    const [isOpen, setOpen] = useState(false);
    const [inputText, setInputText] = useState(props.value);

    useEffect(() => {
        socket.on('query-result', (data) => {
            setSuggestedTitles(data);
        });

        return () => {
            socket.off('query-result');
            socket.close();
        };
    }, []);

    useEffect(() => {
        socket?.emit("query", inputText);
        props.titleChange(inputText);
    }, [inputText])

    const doFocus = () => {
        open();
    };

    const doBlur = (e: any) => {
        if (!e.currentTarget.contains(e.relatedTarget)) {
            close();
        }
    };

    let timeout = 0;

    const [suggestedTitles, setSuggestedTitles] = useState([]);

    const selectItem = (title: string) => {
        setInputText(title);
        close();
    }

    const close = () => {
        clearTimeout(timeout);
        setOpen(false);
    }

    const open = () => {
        clearTimeout(timeout);
        setOpen(true);
    }

    return <div
        onBlurCapture={doBlur}
        onFocusCapture={() => doFocus()}
        className="relative group !mt-1 !mb-1 shadow-sm focus:outline-none rounded-md h-fit border-gray-300 border p-0">
        <span className="flex items-center">
            <input
                type="text"
                value={inputText}
                onChange={(e) => setInputText(e.target.value)}
                className={`block w-full ${isOpen && suggestedTitles.length > 0 ? "rounded-t-md" : "rounded-md"} focus:ring-0 border-none sm:text-sm mt-0 z-10`}
            />
            <span className="z-10 -ml-8 h-6 flex justify-center pointer-events-none">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" strokeWidth={1.5} stroke="currentColor" className="w-6 h-6">
                    <path strokeLinecap="round" strokeLinejoin="round" d="M8.25 15L12 18.75 15.75 15m-7.5-6L12 5.25 15.75 9" />
                </svg>
            </span>
        </span>

        <div style={{
            overflow: "hidden",
            height: `${suggestedTitles.length * (isOpen ? 36 : 0)}px`,
        }} className={`transition-spacing rounded-b-md z-50 flex flex-col overflow-hidden cursor-pointer absolute w-full ring-1 outline-1 ${isOpen && suggestedTitles.length > 0 ? "ring-gray-300" : "ring-transparent"}  ` + style_thinScroll}>
            {
                suggestedTitles.map(
                    (title: string) => <TitleSelectorItem isOpen={isOpen} key={title} onSelect={selectItem} title={title} />
                )
            }
        </div>
    </div>
}