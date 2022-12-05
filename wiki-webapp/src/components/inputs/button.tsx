import NativeProps from "../../util/proptypes";

export default function ButtonBase(props: NativeProps<"button">) {
    return <button
        {...props}
        className="drop-shadow-sm inline-flex items-center justify-center rounded-md border border-transparent bg-white px-4 py-2 text-base font-medium text-indigo-600 hover:bg-indigo-50">
    </button>
}