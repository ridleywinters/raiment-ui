import { JSX } from "react";
import { Div } from "@raiment-ui";

export function NavigationBar(): JSX.Element {
    return (
        <Div
            data-component="NavigationBar"
            sl="flex-row-center px16 py4"
            style={{
                borderBottom: "1px solid #555",
                boxShadow: "0 2px 4px rgba(0, 0, 0, 0.1)",
                lineHeight: 1.0,
            }}
        >
            <Div sl="bold">Raiment Tools</Div>
            <Div
                sl="height-100% ml12 mr24 width-0 height-16"
                style={{
                    borderRight: "4px dotted #ccc",
                }}
            />
            <Div sl="flex-row-center gap-8">
                <a href="/">
                    Home
                </a>
            </Div>
        </Div>
    );
}
