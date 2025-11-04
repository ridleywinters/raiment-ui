import { JSX } from "react";
import { Div } from "../../../../modules/raiment-ui/src/components/div.tsx";
import { NavigationBar } from "./navigation_bar.tsx";

export function ToolAppFrame({ children }: { children: JSX.Element }): JSX.Element {
    return (
        <Div data-component="ToolAppFrame">
            <Div>
                <NavigationBar />
            </Div>
            {children}
        </Div>
    );
}
