type ControllerIDPair = {
    controller: ReadableStreamDefaultController<Uint8Array>;
    id: number;
};

export class SSEClientSet {
    nextID = 1;
    connections: ControllerIDPair[] = [];

    /**
     * Send a message to all connected clients.
     */
    broadcast(data: Record<string, any>) {
        const json = JSON.stringify(data);
        const msg = new TextEncoder().encode(`data: ${json}\r\n\r\n`);
        for (const connection of this.connections) {
            connection.controller.enqueue(msg);
        }
    }

    /**
     * Add a new client connection and return its ID.
     */
    add(controller: ReadableStreamDefaultController<Uint8Array>): number {
        const connection = {
            controller,
            id: this.nextID,
        };
        this.nextID += 1;
        this.connections.push(connection);
        return connection.id;
    }

    /**
     * Remove a client connection by its ID.
     */
    remove(id: number) {
        this.connections = this.connections.filter((connection) => connection.id !== id);
    }
}

export async function handleServerSideEvents(clients: SSEClientSet): Promise<Response> {
    let clientID: number | undefined;
    const body = new ReadableStream({
        start(controller) {
            clientID = clients.add(controller);
        },
        cancel() {
            clients.remove(clientID!);
        },
    });
    return new Response(body, {
        headers: {
            "Content-Type": "text/event-stream",
            "Cache-Control": "no-cache",
            Connection: "keep-alive",
        },
    });
}
