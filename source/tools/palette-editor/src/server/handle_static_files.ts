import { cprintln } from "@raiment-shell";

type HandleStaticFilesOptions = {
    defaultFile?: string;
    stripPrefix?: string;
};

export async function handleStaticFiles(
    baseDir: string,
    req: Request,
    options: HandleStaticFilesOptions = {},
): Promise<Response> {
    const url = new URL(req.url);

    let pathname = url.pathname.replace(/^\//, "");
    if (
        options.stripPrefix &&
        pathname.startsWith(options.stripPrefix)
    ) {
        pathname = pathname.substring(options.stripPrefix.length);
    }
    const filePath = `${baseDir}/${pathname || "index.html"}`;

    return serveFile(
        filePath,
        options.defaultFile ? () => serveFile(`${baseDir}/${options.defaultFile}`) : undefined,
    );
}

async function serveFile(
    filePath: string,
    onNotFound?: () => Promise<Response>,
): Promise<Response> {
    try {
        const fileContent = await Deno.readFile(filePath);
        const contentTypeMap: Record<string, string> = {
            ".txt": "text/plain",
            ".json": "application/json",
            ".yaml": "application/x-yaml",
            ".yml": "application/x-yaml",

            ".js": "text/javascript",

            ".css": "text/css",
            ".html": "text/html",

            ".jpeg": "image/jpeg",
            ".jpg": "image/jpeg",
            ".png": "image/png",
            ".svg": "image/svg+xml",
        };

        const extension = filePath.slice(filePath.lastIndexOf("."));
        const contentType = contentTypeMap[extension] || "text/plain";

        cprintln("#555", `Serving [${filePath}](filename) [(${contentType})](filetype)`);
        return new Response(fileContent, {
            headers: { "Content-Type": contentType },
        });
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) {
            return onNotFound ? onNotFound() : new Response("Not Found", { status: 404 });
        } else {
            console.error(error);
            return new Response("Internal Server Error", { status: 500 });
        }
    }
}
