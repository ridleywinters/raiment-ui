import React, { ElementType, JSX } from "react";
import { StyleLanguage, useStyleLanguage } from "../hooks/use_style_language.tsx";
import { collapseClassName } from "../internal/collapse_class_name.tsx";

export type TagProps<T extends ElementType> =
    & {
        tag: T;
        ref?: React.Ref<React.ComponentRef<T>>;
        sl?: StyleLanguage;
        cl?: string | string[];
    }
    & { [key in `data-${string}`]?: string | number | boolean }
    & React.ComponentPropsWithoutRef<T>;

/**
 * Generic wrapper on native HTML elements that provides:
 *
 * - `sl` prop for "style language" shortcuts (dynamic tailwind-like classes)
 * - `cl` shortcut for className (string or array of strings)
 */
export function Element<T extends ElementType>({
    tag,
    ref,
    sl,
    cl,
    className,
    children,
    ...props
}: TagProps<T>): JSX.Element {
    const Component = tag;
    const slClassName = useStyleLanguage(sl);
    const clClassName = collapseClassName(cl);
    const computedClass = [slClassName, className, clClassName] //
        .filter((c) => Boolean(c)) //
        .join(" ") ??
        undefined;

    return (
        <Component
            data-component={props["data-component"]}
            ref={ref}
            className={computedClass}
            {...props as any}
        >
            {children}
        </Component>
    );
}

export function createExtendedElement<T extends ElementType>(
    tag: T,
    baseProps: Partial<TagProps<T>> = {},
) {
    return function WrappedElement(props: Omit<TagProps<T>, "tag">): JSX.Element {
        return <Element tag={tag} {...baseProps} {...props as any} />;
    };
}
