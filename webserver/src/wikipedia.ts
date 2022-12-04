export default class Wikipedia {
    private static baseURL = "https://en.wikipedia.org/w/api.php"
    private static baseQuery = "?action=query&format=json"

    private static headers: RequestInit = {
        headers: {
            'Content-Type': 'application/json'
        },
    }

    public static async getImageUrl(imgName: string): Promise<string> {
        const url = `${Wikipedia.baseURL}${Wikipedia.baseQuery}&prop=imageinfo&iiprop=url&titles=${imgName}&continue=&origin=*`
        let data = await (await fetch(url, Wikipedia.headers)).json();
        let id = Object.keys(data.query.pages)[0];
        return data.query.pages[id].imageinfo[0].url;
    }

    public static async fetchPageUrl(title: string): Promise<string> {
        const url = `${Wikipedia.baseURL}?action=query&titles=${title}&format=json&prop=info&inprop=url&origin=*`;
        let data = await (await fetch(url, Wikipedia.headers)).json();
        let id = Object.keys(data.query.pages)[0];
        return data.query.pages[id].fullurl;
    }

    private static async fetchPageImageURL(title: string): Promise<string> {
        const url = `${Wikipedia.baseURL}?action=query&format=json&prop=pageimages&titles=${title}&pithumbsize=50&origin=*`
        let data = await (await fetch(url, Wikipedia.headers)).json();
        let id = Object.keys(data.query.pages)[0];
        if (data.query.pages[id].thumbnail) {
            return data.query.pages[id].thumbnail.source;
        } else {
            const url =
                `${Wikipedia.baseURL}?action=query&format=json&prop=images&titles=${title}&origin=*`;

            let data = await (await fetch(url, Wikipedia.headers)).json();

            let id = Object.keys(data.query.pages)[0];
            if (data.query.pages[id].images) {
                return await Wikipedia.getImageUrl(data.query.pages[id].images[0].title)
            }
        }
        return "https://upload.wikimedia.org/wikipedia/commons/thumb/f/f6/Wikipedia-logo-v2-wordmark.svg/125px-Wikipedia-logo-v2-wordmark.svg.png?20180129141506"
    }

    public static async fetchPageImage(title: string): Promise<string> {
        return await Wikipedia.fetchPageImageURL(title);
    }

    public static async getRandomArticle(): Promise<string> {
        const url = `${Wikipedia.baseURL}?action=query&generator=random&grnnamespace=0&grnlimit=1&prop=info&origin=*&format=json`;
        let data = await (await fetch(url, Wikipedia.headers)).json();
        let id = Object.keys(data.query.pages)[0];
        return data.query.pages[id].title;
    }
}

