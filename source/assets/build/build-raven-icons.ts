#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env

import { sh } from "@raiment-shell";

const SRC_DIR = sh.template(
    "$REPO_ROOT/extern/expanded/raven_fantasy_icons_free",
);
const DST_DIR_ICONS = sh.template("./base/icons");
const DST_DIR_SPRITES = sh.template("./base/sprites");

const attributionFile = `${SRC_DIR}/attribution.meta.md`;

const iconFiles = {
    "bow": "fb1789.png",
    "sword": "fb1508.png",
    "axe": "fb1778.png",
    "glove": "fb1488.png",
    "shovel": "fb1470.png",
    "chest": "fb6.png",
    "question": "fb13.png",
    "map": "fb8.png",
    "camp": "fb23.png",
    "feather": "fb7.png",
    "flag_green": "fb42.png",
    "bowl": "fb53.png",
    "key": "fb71.png",
    "torch": "fb81.png",
    "letter": "fb96.png",
    "book": "fb97.png",
    "diamond": "fb168.png",
    "foot": "fb658.png",
    "heart": "fb659.png",
};

const spriteFiles = {
    "coin-gold": "fb131.png",
    "coin-silver": "fb132.png",
};

async function copyFiles(dstDir: string, files: Record<string, string>) {
    sh.cprintln(`Building files for ${dstDir}...`);
    sh.mkdir(dstDir);
    for (const [name, file] of Object.entries(files)) {
        const sourcePath = `${SRC_DIR}/${file}`;
        const outputPath = `${dstDir}/${name}.png`;
        const metaOutputPath = `${dstDir}/${name}.meta.md`;

        await sh.copy(sourcePath, outputPath);
        await sh.copy(attributionFile, metaOutputPath);
        sh.cprintln(
            `[:check:](green) built [${name}.png](goldenrod)`,
        );
    }
}

await copyFiles(DST_DIR_ICONS, iconFiles);
await copyFiles(DST_DIR_SPRITES, spriteFiles);
