#!/usr/bin/env -S deno run --allow-read --allow-write --allow-run --allow-env
import { sh } from "@raiment-shell";

const SRC_DIR = sh.template("$REPO_ROOT/extern/expanded");
const DST_DIR = "./base/sounds";

const files: Record<string, string> = {
    "swing1": "rpg_sound_pack/battle/swing.ogg",
    "swing2": "rpg_sound_pack/battle/swing2.ogg",
    "swing3": "rpg_sound_pack/battle/swing3.ogg",
    "hit1": "rpg_sound_pack/misc/random6.ogg",
    "critical1": "rpg_sound_pack/NPC/ogre/ogre3.ogg",
};

async function copyFiles(dstDir: string, files: Record<string, string>) {
    sh.cprintln(`Building files for ${dstDir}...`);
    sh.mkdir(dstDir);

    for (const [name, file] of Object.entries(files)) {
        const srcPath = `${SRC_DIR}/${file}`;
        const srcBase = file.split("/")[0];
        const attrPath = `${SRC_DIR}/${srcBase}/attribution.meta.md`;
        const dstPath = `${dstDir}/${name}.ogg`;
        const dstAttrPath = `${dstDir}/${name}.meta.md`;

        await sh.copy(srcPath, dstPath);
        await sh.copy(attrPath, dstAttrPath);
        sh.cprintln(
            `[:check:](green) built [${name}.ogg](goldenrod)`,
        );
    }
}
await copyFiles(DST_DIR, files);
