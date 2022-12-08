import md5 from "md5";

const validImgExtensions = ["svg", "png", "jpg", "jpeg", "webp"];

const isImgUrl = (url: string) => validImgExtensions.some(ext => url.endsWith(`.${ext}`));

export default class Wikipedia {
    private static makeUrl = (title: string) =>
        `https://www.wikidata.org/w/api.php?action=wbgetentities&format=json&sites=enwiki&props=claims&titles=${title}`;

    private static headers: RequestInit = {
        headers: {
            'Content-Type': 'application/json'
        },
    }

    public static async fetchPageImage(title: string): Promise<string> {
        return new Promise(async (res, rej) => {
            const response = await fetch(Wikipedia.makeUrl(title), Wikipedia.headers);
            let data = await response.json()
            if (data["success"] != 1) return rej("failed");
            const id = Object.keys(data["entities"])[0];
            if (id == "-1") return rej("Invalid Title");

            data = Object.values(data.entities[id].claims)
                .flat()
                .filter((item: any) => {
                    if (!item?.mainsnak?.datavalue?.value) return false;
                    return typeof item.mainsnak.datavalue.value == "string";
                })
                .map((item: any) => item.mainsnak.datavalue.value as string)
                .filter((item) => isImgUrl(item as string));

            if (data.length == 0) return rej("Image not found");

            const img = data[0].replaceAll(" ", "_");
            let hash = md5(img);
            let [a, b] = hash.split("");
            let result = `https://upload.wikimedia.org/wikipedia/commons/${a}/${a}${b}/${img}`;
            res(result);
        })
    }
}

