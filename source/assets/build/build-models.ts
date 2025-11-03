#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env

/** */
import { sh } from "@raiment-shell";
import * as core from "@raiment-core";

const SRC_DIR = sh.template("$REPO_ROOT/source/content/models");
const ATTRIBUTION_FILE = `${SRC_DIR}/attribution.meta.md`;
const DST_DIR = sh.template("$REPO_ROOT/source/assets/base/models");

const models: Array<{ source: string; destination: string }> = [];
for (const file of await sh.glob(`${SRC_DIR}/**/*.blend`)) {
    const relativePath = file.substring(SRC_DIR.length + 1);
    const destinationPath = `${DST_DIR}/${relativePath}`.replace(/\.blend$/, ".gltf");
    models.push({ source: file, destination: destinationPath });
    sh.cprintln(
        `Found model: [{{source}}](goldenrod) -> [{{destination}}](goldenrod)`,
        {
            source: file,
            destination: destinationPath,
        },
    );
}

for (const model of models) {
    await sh.mkdir(sh.dirname(model.destination));

    await convertBlendToGLTF(model.source, model.destination);
    sh.cprintln(
        `[:check:](green) Converted [{{source}}](cyan) to [{{destination}}](cyan)`,
        {
            source: model.source,
            destination: model.destination,
        },
    );
}

sh.cprintln(
    `[:check:](green) Found {{count}} models in total.`,
    { count: models.length },
);

async function convertBlendToGLTF(source: string, destination: string): Promise<void> {
    // Path to the Python script relative to this file
    const scriptPath = new URL(
        "./build-models-convert-gltf.py",
        import.meta.url,
    ).pathname;

    // Run Blender in background mode with the script
    await sh.exec("blender", [
        "--background",
        "--python",
        scriptPath,
        "--",
        source,
        destination,
    ]);
}
