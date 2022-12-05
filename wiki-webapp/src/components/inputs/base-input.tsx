import NativeProps from "../../util/proptypes";

// export type BaseInputProps = NativeProps<HTMLInputElement> & Required<Pick<NativeProps<HTMLInputElement>, "type">>;

export default function BaseInput(props: NativeProps<"input">) {
    return <input {...props} className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm" />
}
