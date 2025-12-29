import React, { JSX } from "react";
import { Element, TagProps } from "./element.tsx";
import { handleKeyMapping, KeyMappingTable } from "../util/key_mapping.ts";
import { useCSSLocal } from "../hooks/use_css.ts";
import { css } from "../index.ts";

type TextareaProps =
    & Omit<TagProps<"textarea">, "tag">
    & {
        onKeyMap?: KeyMappingTable<React.KeyboardEvent<HTMLTextAreaElement>>;
    };

export function Textarea(
    {
        onKeyDown,
        onKeyMap,
        className,
        ...rest
    }: TextareaProps,
): JSX.Element {
    const localClass = useCSSLocal(css`
        .self {
            height: auto;
            resize: none;
            min-height: 1lh;
            max-height: 100vh;
            field-sizing: content;
        }
    `);

    const handleKeyDown = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (onKeyMap) {
            handleKeyMapping(evt, onKeyMap);
        }
        onKeyDown?.(evt);
    };

    className = [className, localClass].filter(Boolean).join(" ");

    return (
        <Element
            tag="textarea"
            className={className}
            onKeyDown={handleKeyDown}
            rows={1}
            {...rest as any}
        />
    );
}
