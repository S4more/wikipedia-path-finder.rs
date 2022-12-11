"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const lyra_1 = require("@lyrasearch/lyra");
class SearchManager {
    constructor() {
        this.db = (0, lyra_1.create)({
            defaultLanguage: "english",
            schema: {
                title: "string"
            }
        });
    }
    search(term) {
        const searchResult = (0, lyra_1.search)(this.db, { term, limit: 6 });
        return searchResult.hits.map(hit => hit.document.title);
    }
    handleConnection(socket) {
        console.log("Connected");
        socket.on("query", (query) => {
            socket.emit("query-result", this.search(query));
        });
    }
    attachIo(io) {
        io.on('connection', this.handleConnection.bind(this));
        console.log(io);
    }
    insert(items) {
        (0, lyra_1.insertBatch)(this.db, items, { language: 'english', batchSize: 100 });
    }
}
exports.default = SearchManager;
