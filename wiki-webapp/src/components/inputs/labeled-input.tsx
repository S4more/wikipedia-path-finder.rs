import NativeProps from "../../util/proptypes";
import TextInput from "./base-input";

export default function LabeledInput(props: NativeProps<"input">) {
    return <span>
        <label htmlFor={props.id}>
            {props.children}
        </label>

        <TextInput {...props} children={undefined} id={props.id} />
    </span>
}