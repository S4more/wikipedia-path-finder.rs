import express from 'express';
import Rasterizer from './rasterizer';
import Wikipedia from './wikipedia';
import { Server } from 'socket.io';
import { createServer } from 'http';
import SearchManager from './search';
import loadIndexes from './title-index-loader';

const app = express();
const port = 8080;
const httpServer = createServer(app)

const socketServer = new Server(httpServer);

const searchManager = new SearchManager();
searchManager.attachIo(socketServer);

loadIndexes(searchManager, 100_000);

const rasterizer = new Rasterizer(256);
let requests = 0;

app.get("/image/:title", async (req, res) => {
    Wikipedia.fetchPageImage(req.params.title.replace(".png", "")).then(img => {
        try {
            rasterizer.rasterize(img).then(img => res.send(img));
        } catch (e) {
            res.sendStatus(404);
        }
    }).catch(err => {
        res.sendStatus(404);
    })
});


app.use(express.static("../wiki-webapp/dist"));

httpServer.listen(port);