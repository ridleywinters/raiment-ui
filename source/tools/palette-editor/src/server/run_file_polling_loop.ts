export function runFilePollingLoop(
    filename: string,
    callback: (opts: { current: number | undefined; previous: number | undefined }) => void,
) {
    const status = {
        timestamp: undefined as number | undefined,
    };

    const pollFile = async (filename: string) => {
        const current = await getFileTimestamp(filename);
        if (current === undefined) {
            return;
        }
        if (status.timestamp === undefined) {
            status.timestamp = current;
            return;
        }
        if (status.timestamp === current) {
            return;
        }
        const previous = status.timestamp;
        status.timestamp = current;
        callback({ current, previous });
    };
    const pollLoop = async () => {
        await pollFile(filename);
        setTimeout(pollLoop, Math.floor(Math.random() * 100) + 400);
    };
    pollLoop();
}

async function getFileTimestamp(filePath: string): Promise<number | undefined> {
    try {
        const fileInfo = await Deno.stat(filePath);
        return fileInfo.mtime?.getTime();
    } catch (error) {
        if (error instanceof Deno.errors.NotFound) {
            return undefined;
        } else {
            throw error;
        }
    }
}
