import NativeProps from "../../util/proptypes";

export default function SubmitButton(props: NativeProps<"button">) {
    return <button
        {...props}
        className="drop-shadow-sm inline-flex align-middle justify-center rounded-md border border-transparent bg-indigo-600 py-2 px-4 text-base font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2">
    </button>
}