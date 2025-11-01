import { ServerOptions, serverStart } from "@/server_start.ts";
import { parseArgs } from "jsr:@std/cli@^1.0.10/parse-args";

function main() {
    const cliArgs = parseArgs(Deno.args, {
        string: ["title", "port"],
        default: {
            title: "raiment-dev-server",
            port: "7000",
        },
    });

    const options: ServerOptions = {
        title: cliArgs.title,
        port: parseInt(cliArgs.port, 10),
    };
    serverStart(options);
}

main();
