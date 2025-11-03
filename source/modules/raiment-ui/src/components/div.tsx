import React, { JSX } from "react";
import { StyleLanguage, useStyleLanguage } from "../hooks/use_style_language.tsx";

type DataAttrs = { [key in `data-${string}`]?: string | number | boolean };
type EventHandlers = {
    [key in `on${string}`]?: (e: React.SyntheticEvent<HTMLDivElement>) => void;
};

type DivProps =
    & {
        sl?: StyleLanguage;
        children?: React.ReactNode;
        className?: string;
        style?: React.CSSProperties;
    }
    & DataAttrs
    & EventHandlers;

export function Div({
    sl,
    className,
    style,
    children,
    ...props
}: DivProps): JSX.Element {
    const slClassName = useStyleLanguage(sl);
    const computedClass = [className, slClassName].filter((c) => c).join(" ") || undefined;
    return <div className={computedClass} style={style} {...props}>{children}</div>;
}
