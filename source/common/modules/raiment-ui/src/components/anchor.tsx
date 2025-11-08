import React, { JSX } from "react";
import { StyleLanguage, useStyleLanguage } from "../hooks/use_style_language.tsx";

type DataAttrs = { [key in `data-${string}`]?: string | number | boolean };
type EventHandlers = {
    [key in `on${string}`]?: (e: React.SyntheticEvent<HTMLAnchorElement>) => void;
};

type AnchorProps =
    & {
        sl?: StyleLanguage;
        children?: React.ReactNode;
        className?: string;
        style?: React.CSSProperties;
        href: HTMLAnchorElement["href"];
    }
    & DataAttrs
    & EventHandlers;

export function Anchor({
    sl,
    className,
    style,
    children,
    href,
    ...props
}: AnchorProps): JSX.Element {
    const slClassName = useStyleLanguage(sl);
    const computedClass = [className, slClassName].filter((c) => c).join(" ") || undefined;
    return (
        <a
            // Show the component first (if set) for easier inspection
            data-component={props["data-component"]}
            className={computedClass}
            style={style}
            href={href}
            {...props}
        >
            {children}
        </a>
    );
}
