import React, { JSX } from "react";
import { Element, TagProps } from "./element.tsx";
import { handleKeyMapping, KeyMappingTable } from "../util/key_mapping.ts";
import { useCSSLocal } from "../hooks/use_css.ts";
import { css } from "../index.ts";

type TextareaProps =
    & Omit<TagProps<"textarea">, "tag">
    & {
        onKeyMap?: KeyMappingTable<React.KeyboardEvent<HTMLTextAreaElement>>;
    }
    & {
        variant?: "auto-resize" | "bare";
    };

/**
 * Wrapper on native HTML element that provides all Element extensions but
 * also defaults to a variant CSS styling that auto-resizes the textarea.
 */
export function Textarea(
    {
        variant = "auto-resize",
        className,
        onKeyDown,
        onKeyMap,
        ...rest
    }: TextareaProps,
): JSX.Element {
    const localClass = useCSSLocal(
        variant === "auto-resize"
            ? css`
                .self {
                    height: auto;
                    resize: none;
                    min-height: 1lh;
                    max-height: 100vh;
                    field-sizing: content;
                }
            `
            : "",
    );
    className = [className, localClass].filter(Boolean).join(" ");

    const handleKeyDown = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
        if (onKeyMap) {
            handleKeyMapping(evt, onKeyMap);
        }
        onKeyDown?.(evt);
    };

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
