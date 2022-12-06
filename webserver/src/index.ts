import express from 'express';
import Rasterizer from './rasterizer';
import Wikipedia from './wikipedia';
import { Server } from 'socket.io';
import { createServer } from 'http';
import SearchManager from './search';

const app = express();
const port = 8080;
const httpServer = createServer(app)

const socketServer = new Server(httpServer);

const searchManager = new SearchManager();
console.log(searchManager);
searchManager.attachIo(socketServer);

// Add to the searcher
// TODO Change this
setInterval(() => {
    Wikipedia.getRandomArticle().then(title => searchManager.insert(title)).catch(console.log);
}, 50);

const rasterizer = new Rasterizer(256);
let requests = 0;
app.get("/random/*", async (req, res) => {
    Wikipedia.getRandomArticle().then(title => {
        Wikipedia.fetchPageImage(title).then(img => {
            rasterizer.rasterize(img).then(img => {
                res.send(img);
                requests++;
                console.log(requests);
            });
        })
    }).catch(e => {
        console.log(e);
    })
})

app.get("/image/:title", async (req, res) => {
    try {
        let image = await Wikipedia.fetchPageImage(req.params.title.replace(".png", ""));
        rasterizer.rasterize(image).then(img => res.send(img));
    } catch (e) {
        res.status(404);
    }
});

app.get("/wiki/random", async (req, res) => {
    res.send(await Wikipedia.getRandomArticle());
})

app.use(express.static("../wiki-webapp/dist"));

httpServer.listen(port);