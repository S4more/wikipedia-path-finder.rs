import express from 'express';
import Rasterizer from './rasterizer';
import Wikipedia from './wikipedia';

const app = express();
const port = 8080;

const rasterizer = new Rasterizer(256);
let img = "";
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

app.listen(port, () => {
    console.log(`Server is running at http://localhost:${port}`);
});