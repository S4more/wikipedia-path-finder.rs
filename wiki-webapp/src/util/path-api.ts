const API_PATH = `${window.location.origin}`.replace("8080", "8000");

async function rawPathFetch(from: string, to: string, hops: number): Promise<string> {
    return await (await fetch(`${API_PATH}/title/${from}/${to}/${hops}`)).text();
}

export function findPath(from: string, to: string): Promise<string[]> {
    return new Promise(async (res, rej) => {
        for (let i = 1; i < 8; i++) {
            let path = await rawPathFetch(from, to, i);
            if (path.charAt(0) == "[") {
                res(JSON.parse(path));
            }
        }
        rej("Path not found");
    });
}