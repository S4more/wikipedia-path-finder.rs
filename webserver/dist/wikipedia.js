"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const md5_1 = __importDefault(require("md5"));
const validImgExtensions = ["svg", "png", "jpg", "jpeg", "webp"];
const isImgUrl = (url) => validImgExtensions.some(ext => url.endsWith(`.${ext}`));
class Wikipedia {
    static async fetchPageImage(title) {
        return new Promise(async (res, rej) => {
            const response = await fetch(Wikipedia.makeUrl(title), Wikipedia.headers);
            let data = await response.json();
            if (data["success"] != 1)
                return rej("failed");
            const id = Object.keys(data["entities"])[0];
            if (id == "-1")
                return rej("Invalid Title");
            data = Object.values(data.entities[id].claims)
                .flat()
                .filter((item) => {
                if (!item?.mainsnak?.datavalue?.value)
                    return false;
                return typeof item.mainsnak.datavalue.value == "string";
            })
                .map((item) => item.mainsnak.datavalue.value)
                .filter((item) => isImgUrl(item));
            if (data.length == 0)
                return rej("Image not found");
            const img = data[0].replaceAll(" ", "_");
            let hash = (0, md5_1.default)(img);
            let [a, b] = hash.split("");
            let result = `https://upload.wikimedia.org/wikipedia/commons/${a}/${a}${b}/${img}`;
            res(result);
        });
    }
}
exports.default = Wikipedia;
Wikipedia.makeUrl = (title) => `https://www.wikidata.org/w/api.php?action=wbgetentities&format=json&sites=enwiki&props=claims&titles=${title}`;
Wikipedia.headers = {
    headers: {
        'Content-Type': 'application/json'
    },
};
