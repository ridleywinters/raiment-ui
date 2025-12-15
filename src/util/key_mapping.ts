export type KeyMappingTable<T> = Record<string, (evt: T) => void>;

export function handleKeyMapping<
    T extends {
        key: string;
        ctrlKey: boolean;
        altKey: boolean;
        metaKey: boolean;
        shiftKey: boolean;
        preventDefault(): void;
        stopPropagation(): void;
    },
>(
    evt: T,
    table: KeyMappingTable<T>,
): void {
    let key = evt.key;
    if (key === " ") {
        key = "Space";
    }
    if (evt.shiftKey) {
        key = `Shift+${key}`;
    }
    if (evt.altKey || evt.metaKey) {
        key = `Alt+${key}`;
    }
    if (evt.ctrlKey) {
        key = `Ctrl+${key}`;
    }

    const tableKeys = Object.keys(table);
    let handler = undefined;
    let stopEvent = true;
    for (let i = 0; handler === undefined && i < tableKeys.length; i++) {
        const baseKey = tableKeys[i];
        const baseHandler = table[baseKey];

        const subkeys = baseKey.split(",").map((s) => s.trim());

        for (let k of subkeys) {
            if (k.endsWith("?")) {
                k = k.slice(0, -1);
                stopEvent = false;
            }
            if (k !== key) {
                continue;
            }
            handler = baseHandler;
            break;
        }
    }
    if (handler) {
        if (stopEvent) {
            evt.preventDefault();
            evt.stopPropagation();
        }
        handler(evt);
    } else {
        // console.log("No handler for key:", key);
    }
}
