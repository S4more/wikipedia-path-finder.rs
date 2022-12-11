"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const express_1 = __importDefault(require("express"));
const rasterizer_1 = __importDefault(require("./rasterizer"));
const wikipedia_1 = __importDefault(require("./wikipedia"));
const socket_io_1 = require("socket.io");
const http_1 = require("http");
const search_1 = __importDefault(require("./search"));
const title_index_loader_1 = __importDefault(require("./title-index-loader"));
const app = (0, express_1.default)();
const port = 8080;
const httpServer = (0, http_1.createServer)(app);
const socketServer = new socket_io_1.Server(httpServer);
const searchManager = new search_1.default();
searchManager.attachIo(socketServer);
(0, title_index_loader_1.default)(searchManager, 100000);
const rasterizer = new rasterizer_1.default(256);
let requests = 0;
app.get("/image/:title", async (req, res) => {
    wikipedia_1.default.fetchPageImage(req.params.title.replace(".png", "")).then(img => {
        try {
            rasterizer.rasterize(img).then(img => res.send(img));
        }
        catch (e) {
            res.sendStatus(404);
        }
    }).catch(err => {
        res.sendStatus(404);
    });
});
app.use(express_1.default.static("../wiki-webapp/dist"));
httpServer.listen(port);
