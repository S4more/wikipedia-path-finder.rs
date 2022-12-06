import React from "react";


type NativeProps<T extends keyof JSX.IntrinsicElements> = JSX.IntrinsicElements[T];

export default NativeProps;