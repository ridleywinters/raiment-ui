import React, { JSX } from "react";
import { Element, TagProps } from "./element.tsx";
import { handleKeyMapping, KeyMappingTable } from "../util/key_mapping.ts";

type TextareaProps =
    & Omit<TagProps<"textarea">, "tag">
    & {
        onKeyMap?: KeyMappingTable<React.KeyboardEvent<HTMLTextAreaElement>>;
    };

export function Textarea(
    {
        onKeyDown,
        onKeyMap,
        ...rest
    }: TextareaProps,
): JSX.Element {
    const handleKeyDown = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (onKeyMap) {
            handleKeyMapping(evt, onKeyMap);
        }
        onKeyDown?.(evt);
    };
    return (
        <Element
            tag="textarea"
            onKeyDown={handleKeyDown}
            {...rest as any}
        />
    );
}
