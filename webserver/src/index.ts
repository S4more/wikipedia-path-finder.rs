import express, { Express, Request, Response } from 'express';

const app = express();
const port = 8080;


app.use(express.static("../wiki-webapp/dist"));


app.listen(port, () => {
    console.log(`Server is running at http://localhost:${port}`);
});