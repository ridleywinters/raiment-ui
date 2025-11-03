import React from "react";
import type { JSX } from "react";
import { PaletteEditorApp } from "@/apps/palette_editor/palette_editor_app.tsx";
import { Div } from "@raiment-ui";

const routes2: [string, string, () => JSX.Element][] = [
    ["Palette Editor", "palette-editor", () => <PaletteEditorApp />],
];

export function AppView(): JSX.Element {
    React.useLayoutEffect(() => {
        document.title = "Raiment Tools";
    }, []);

    const pathname = globalThis.location.pathname;

    for (const [name, path, componentFunc] of routes2) {
        if (pathname.startsWith(`/${path}`)) {
            return componentFunc();
        }
    }
    return <HomeView />;
}

function HomeView(): JSX.Element {
    return (
        <Div sl="m32">
            <Div>
                Raiment Tools
            </Div>
            <Div sl="flex-col">
                {routes2.map(([name, path]) => (
                    <Div key={path} sl="flex-row-center my4">
                        <Div sl="mr8">
                            <Div
                                style={{
                                    height: 6,
                                    width: 6,
                                    borderRadius: "50%",
                                    backgroundColor: "#888",
                                }}
                            />
                        </Div>
                        <Div>
                            <a
                                href={`/${path}`}
                                style={{
                                    color: "#007bff",
                                    textDecoration: "none",
                                }}
                            >
                                {name}
                            </a>
                        </Div>
                    </Div>
                ))}
            </Div>
        </Div>
    );
}
