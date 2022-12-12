import PathList from "../components/path-list";

const API_PATH = `${window.location.origin}`.replace("8080", "8000");

async function rawPathFetch(from: string, to: string, hops: number): Promise<string> {
    return await (await fetch(`${API_PATH}/title/${from}/${to}/${hops}`)).text();
}

export async function findPath(from: string, to: string): Promise<string[]> {
    for (let i = 1; i < 8; i++) {
        let path = await rawPathFetch(from, to, i);
        if (path.charAt(0) == "[") {
            return JSON.parse(path);
        }
    }
    throw "";
}


function combinePaths(paths: string[][]) {
    let root: any = {};

    for (let path of paths) {
        let l = root;
        for (let layer of path) {
            if (l[layer]) {
                l = l[layer];
            } else {
                l[layer] = {};
                l = l[layer];
            }
        }
    }
    console.error(root);
}


export async function findCompoundPath(from: string, to: string) {
    console.warn("Compound")
    let paths = [];
    let s = new Set<string>();

    for (let i = 0; i < 128; i++) {
        let path = await findPath(from, to);
        console.log(i);
        paths.forEach(path => path.forEach(item => s.add(item)));
        console.error(s);
        paths.push(path);
    }


    combinePaths(paths);
    // console.error(s);

    // let paths = await Promise.all([...Array(8)].map((v) => await findPath(from, to).catch(console.error).then(res => {
    //     console.warn(res)
    //     return res;
    // })));
    console.warn(paths);
}