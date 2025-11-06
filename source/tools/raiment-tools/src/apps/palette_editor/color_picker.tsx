import React, { type JSX } from "react";
import type { HexColor } from "@raiment-core";
import { Div, useThrottledCallback } from "@raiment-ui";

/**
 * Wrapper on <input type="color"> with throttled onChange to reduce performance impact of
 * rapidly changing colors (e.g. when the user is dragging cursor over the color picker).
 * Fires at most once every 200ms but also at least once every 200ms during continuous input.
 */
export function ColorPicker({
    value,
    onChange,
}: {
    value: HexColor;
    onChange: (newColor: HexColor) => void;
}): JSX.Element {
    const handleChange = useThrottledCallback(
        (evt: React.ChangeEvent<HTMLInputElement>) => {
            onChange(evt.target.value as HexColor);
        },
        200,
    );

    return (
        <Div>
            <input
                type="color"
                value={value}
                onChange={handleChange}
            />
        </Div>
    );
}
