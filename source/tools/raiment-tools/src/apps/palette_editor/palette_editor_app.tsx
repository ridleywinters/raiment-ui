import * as core from "@raiment-core";
import { type ColorHexString } from "@raiment-core";
import { Div, invokeDownload, useEventListener } from "@raiment-ui";
import React, { JSX } from "react";
import { Palette } from "./palette.ts";
import { serverAPI } from "@/util/server_api.tsx";

export function PaletteEditorApp(): JSX.Element {
    const [palette, setPalette] = React.useState<Palette | null>(null);

    React.useEffect(() => {
        const go = async () => {
            const gplContent = await serverAPI.readFile("palette/palette.gpl", "text") as string;
            const colors = core.parseGIMPPalette(gplContent);
            const pal = Palette.fromGIMPPalette(colors ?? []);
            setPalette(pal);
        };
        go();
    }, []);

    if (!palette) {
        return <div>Loading palette...</div>;
    }

    return <AppView2 palette={palette} />;
}

function AppView2({ palette }: { palette: Palette }): JSX.Element {
    useEventListener(palette.events, "update");

    const exportGPL = React.useCallback(() => {
        const colors = palette.computeAll();
        const gplContent = core.stringifyGIMPPalette(colors);
        invokeDownload("palette.gpl", gplContent, "text/plain");
    }, [palette]);

    const savePalette = React.useCallback(async () => {
        const colors = palette.computeAll();
        const gplContent = core.stringifyGIMPPalette(colors);
        await serverAPI.writeFile("palette/palette.gpl", gplContent, "text");
        alert("Palette saved successfully.");
    }, [palette]);

    return (
        <Div sl="mt32 mx64">
            <Div sl="flex-row-center gap-16">
                <h1>Palette Editor v0.1</h1>
                <button
                    type="button"
                    onClick={exportGPL}
                    style={{
                        padding: "8px 16px",
                        fontSize: "14px",
                        cursor: "pointer",
                        backgroundColor: "#4CAF50",
                        color: "white",
                        border: "none",
                        borderRadius: "4px",
                    }}
                >
                    Export Palette
                </button>

                <button
                    type="button"
                    onClick={savePalette}
                    style={{
                        padding: "8px 16px",
                        fontSize: "14px",
                        cursor: "pointer",
                        backgroundColor: "#4CAF50",
                        color: "white",
                        border: "none",
                        borderRadius: "4px",
                    }}
                >
                    Save Palette
                </button>
            </Div>

            <Div sl="flex-row-center gap-32">
                <Div>
                    <Div sl="flex-row-center gap-8 mb-16">
                        {palette.base.map((_color, idx) => (
                            <ColorPicker
                                key={idx}
                                value={palette.getBase(idx)}
                                onChange={(newColor) => {
                                    palette.setBase(idx, newColor);
                                }}
                            />
                        ))}
                    </Div>
                    <Div sl="flex-column mb-16">
                        {palette.colors.map((colorSet, idx) => (
                            <PaletteRow key={idx} palette={palette} rowIndex={idx} />
                        ))}
                    </Div>
                </Div>
                <div>
                    <div style={{ height: 40 }} />
                    <PaletteCanvas palette={palette} />
                    <div
                        style={{
                            color: "#555",
                            fontStyle: "italic",
                            maxWidth: "240px",
                            textAlign: "center",
                        }}
                    >
                        Right-click to save palette as an image
                    </div>
                </div>
            </Div>
            <div>
                Color count: {palette.computeAll().length}
            </div>
        </Div>
    );
}

function PaletteCanvas({ palette }: { palette: Palette }): JSX.Element {
    const BLOCK_SIZE = 32;
    const COLUMNS = 7;

    const ref = React.useRef<HTMLCanvasElement | null>(null);
    const gen = useEventListener(palette.events, "update");

    React.useEffect(() => {
        const canvas = ref.current;
        if (!canvas) {
            return;
        }
        const ctx = canvas.getContext("2d");
        if (!ctx) {
            return;
        }

        const colors = palette.computeAll();
        const rows = Math.ceil(colors.length / 7);
        const width = 7 * BLOCK_SIZE;
        const height = rows * BLOCK_SIZE;

        canvas.width = width;
        canvas.height = height;
        ctx.clearRect(0, 0, width, height);
        for (let y = 0; y < rows; y++) {
            for (let x = 0; x < COLUMNS; x++) {
                const index = y * COLUMNS + x;
                if (index >= colors.length) {
                    break;
                }
                const color = colors[index];
                ctx.fillStyle = color;
                ctx.fillRect(x * BLOCK_SIZE, y * BLOCK_SIZE, BLOCK_SIZE, BLOCK_SIZE);
            }
        }
    }, [gen]);

    return (
        <div
            style={{
                display: "flex",
                flexDirection: "column",
            }}
        >
            <canvas ref={ref} width={256} height={256} style={{ border: "1px solid #444" }} />
        </div>
    );
}

function PaletteRow({ palette, rowIndex }: { palette: Palette; rowIndex: number }): JSX.Element {
    return (
        <div
            style={{
                display: "flex",
                flexDirection: "row",
                gap: "8px",
                marginBottom: "4px",
            }}
        >
            <ColorPicker
                value={palette.get(rowIndex, "primary")}
                onChange={(newColor) => {
                    palette.set(rowIndex, "primary", newColor);
                }}
            />
            <div style={{ width: "12px" }} />
            <ColorPicker
                value={palette.get(rowIndex, "shade")}
                onChange={(newColor) => {
                    palette.set(rowIndex, "shade", newColor);
                }}
            />
            <ColorPicker
                value={palette.get(rowIndex, "highlight")}
                onChange={(newColor) => {
                    palette.set(rowIndex, "highlight", newColor);
                }}
            />
            <div style={{ width: "24px" }} />
            {palette.computeRow(rowIndex).map((color, cidx) => (
                <Div
                    key={cidx}
                    sl="width-32 height-32 cursor-pointer"
                    style={{
                        backgroundColor: color,
                        border: "1px solid #444",
                    }}
                    onClick={async () => {
                        try {
                            await navigator.clipboard.writeText(color);
                            console.log(`Copied ${color} to clipboard`);
                        } catch (err) {
                            console.error(
                                "Failed to copy color to clipboard:",
                                err,
                            );
                        }
                    }}
                />
            ))}
            <button
                type="button"
                onClick={() => palette.moveRow(rowIndex, "up")}
                style={{
                    padding: "4px 8px",
                    cursor: "pointer",
                    border: "none",
                    background: "transparent",
                    color: "#555",
                }}
            >
                ▲
            </button>
        </div>
    );
}

function ColorPicker({
    value,
    onChange,
}: {
    value: ColorHexString;
    onChange: (newColor: ColorHexString) => void;
}): JSX.Element {
    const timeoutRef = React.useRef<number | null>(null);
    const handleChange = React.useCallback((evt: React.ChangeEvent<HTMLInputElement>) => {
        const newColor = evt.target.value as ColorHexString;
        if (timeoutRef.current) {
            clearTimeout(timeoutRef.current);
        }
        timeoutRef.current = setTimeout(() => {
            onChange(newColor);
        }, 200);
    }, [onChange]);

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
