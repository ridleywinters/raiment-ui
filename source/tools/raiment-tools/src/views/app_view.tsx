import React from "react";
import type { JSX } from "react";
import { PaletteEditorApp } from "@/apps/palette_editor/palette_editor_app.tsx";
import { DungeonGeneratorApp } from "@/apps/dungeon_generator/index.tsx";
import { Div } from "@raiment-ui";
import { useHistoryNavigation } from "@/hooks/use_history_navigation.ts";
import { ToolAppFrame } from "@/components/tool_app_frame.tsx";

const routes2: [string, string, () => JSX.Element, string[]][] = [
    ["Palette Editor", "palette-editor", () => <PaletteEditorApp />, [
        "view/modify the internal color palette for the project",
    ]],
    ["Dungeon Generator", "dungeon-generator", () => <DungeonGeneratorApp />, [
        "work-in-progress procedural dungeon generator",
    ]],
];

export function AppView(): JSX.Element {
    React.useLayoutEffect(() => {
        document.title = "Raiment Tools";
    }, []);

    const url = useHistoryNavigation();

    const pathname = url.pathname;
    for (const [_name, path, componentFunc] of routes2) {
        if (pathname.startsWith(`/${path}`)) {
            return componentFunc();
        }
    }
    return <HomeView />;
}

function HomeView(): JSX.Element {
    return (
        <ToolAppFrame>
            <Div sl="m32">
                <Div sl="bold mb16">
                    Tools
                </Div>
                <Div sl="flex-col">
                    {routes2.map(([name, path, _ignored, description]) => (
                        <Div key={path} sl="flex-row-center my2">
                            <Div
                                sl="height-6 width-6 mr8"
                                style={{
                                    borderRadius: "50%",
                                    backgroundColor: "#888",
                                }}
                            />
                            <Div sl="width-200">
                                <a
                                    href={`/${path}`}
                                >
                                    {name}
                                </a>
                            </Div>
                            <Div>
                                <Div sl="ml32 italic fg-gray-60%">
                                    {description.join(" ")}
                                </Div>
                            </Div>
                        </Div>
                    ))}
                </Div>
            </Div>
        </ToolAppFrame>
    );
}
