export function collapseClassName(
    className?: string | string[] | Record<string, boolean>,
): string | undefined {
    const t = typeof className;
    if (t === "string") {
        return className as string;
    } else if (Array.isArray(className)) {
        return (className as string[]).filter((c) => Boolean(c)).join(" ") || undefined;
    } else if (t === "object" && className !== null) {
        return Object.entries(className as Record<string, boolean>)
            .filter(([_, v]) => v)
            .map(([k, _]) => k)
            .join(" ") || undefined;
    }
}
