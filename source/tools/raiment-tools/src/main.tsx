import { useServerSideEvents } from "@raiment-ui";
import { JSX } from "react";
import { createRoot } from "react-dom/client";
import { AppView } from "./views/app_view.tsx";

function AppWrapper(): JSX.Element {
    useServerSideEvents("/api/events", (data: any) => {
        switch (data.type) {
            case "app.reload":
                globalThis.location.reload();
                break;
            default:
                console.warn("Unknown server-side event type:", data.type);
                break;
        }
    });
    return <AppView />;
}

function main(): void {
    const el = document.getElementById("root")!;
    createRoot(el).render(<AppWrapper />);
}
main();
