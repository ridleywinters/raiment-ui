class ServerAPI {
    async readFile(path: string, format: "text" | "yaml" | "json"): Promise<string | object> {
        const resp = await fetch("/api/read-file", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({ path, format }),
        });
        if (!resp.ok) {
            throw new Error(`Failed to read file: ${resp.statusText}`);
        }
        if (format === "text") {
            return await resp.text();
        } else if (format === "yaml" || format === "json") {
            return await resp.json();
        } else {
            throw new Error(`Unsupported format: ${format}`);
        }
    }

    async writeFile(
        path: string,
        content: string,
        format: "text" | "yaml" | "json",
    ): Promise<void> {
        const resp = await fetch("/api/write-file", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                path,
                content,
                format,
            }),
        });
        if (!resp.ok) {
            throw new Error(`Failed to write file: ${resp.statusText}`);
        }
    }
}
export const serverAPI = new ServerAPI();
