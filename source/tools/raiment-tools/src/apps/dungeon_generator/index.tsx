import { JSX } from "react";
import { DungeonGeneratorAppView } from "./dungeon_generator_app_view.tsx";
import { ToolAppFrame } from "@/components/tool_app_frame.tsx";

export function DungeonGeneratorApp(): JSX.Element {
    return (
        <ToolAppFrame>
            <DungeonGeneratorAppView />
        </ToolAppFrame>
    );
}
