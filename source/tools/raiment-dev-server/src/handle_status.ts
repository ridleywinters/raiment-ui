export async function handleStatus(_request: Request): Promise<Response> {
    return new Response("OK", {
        headers: { "content-type": "text/plain" },
    });
}
