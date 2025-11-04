import React from "react";

/**
 * Intercepts clicks on anchor elements and uses the History API to navigate instead,
 * thus preventing full page reloads for internal links. Internally updates and
 * returns a URL object representing the current location, which causes the calling
 * component to re-render on navigation.
 *
 * This is intended for top-level components that do their own routing based on the URL.
 */
export function useHistoryNavigation(): URL {
    const [url, setUrl] = React.useState<URL>(new URL(globalThis.location.href));

    React.useEffect(() => {
        // Check if navigation is already set up.
        // We use a data attribute so it is visible in the inspector which can be helpful for
        // understanding how the app is functioning.
        if (document.body.getAttribute("data-use-history-navigation") === "true") {
            console.warn("History navigation already initialized, skipping duplicate setup");
            return;
        }

        function handleClick(event: MouseEvent): void {
            const target = event.target as Element;
            const anchor = target.closest("a");
            if (anchor?.href) {
                const url = new URL(anchor.href);
                const currentUrl = new URL(globalThis.location.href);

                // Only handle internal links (same origin)
                if (url.origin === currentUrl.origin) {
                    event.preventDefault();
                    globalThis.history.pushState(null, "", url.pathname + url.search + url.hash);
                    setUrl(url);
                }
            }
        }

        function handlePopState(): void {
            setUrl(new URL(globalThis.location.href));
        }

        document.addEventListener("click", handleClick);
        globalThis.addEventListener("popstate", handlePopState);
        document.body.setAttribute("data-use-history-navigation", "true");

        return () => {
            document.removeEventListener("click", handleClick);
            globalThis.removeEventListener("popstate", handlePopState);
            document.body.removeAttribute("data-use-history-navigation");
        };
    }, []);

    return url;
}
