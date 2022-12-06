import { Server, Socket } from 'socket.io';
import { create, insert, remove, search } from "@lyrasearch/lyra";


export default class SearchManager {
    private readonly db = create({
        defaultLanguage: "english",
        schema: {
            title: "string"
        }
    })

    private search(term: string) {
        const searchResult = search(this.db, { term, limit: 6 });
        console.log(searchResult);
        return searchResult.hits.map(hit => hit.document.title);
    }

    private handleConnection(socket: Socket) {
        console.log("Connected");
        socket.on("query", (query: string) => {
            socket.emit("query-result", this.search(query));
        });
    }

    attachIo(io: Server) {
        io.on('connection', this.handleConnection.bind(this))
        console.log(io);
    }

    insert(...items: string[]) {
        for (let item of items) {
            insert(this.db, { title: item });
        }
    }
}