"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const stream_json_1 = require("stream-json");
const stream_chain_1 = require("stream-chain");
const StreamArray_1 = require("stream-json/streamers/StreamArray");
const fs_1 = __importDefault(require("fs"));
const good_mem = (usage) => {
    for (let key in usage) {
        usage[key] = usage[key] / 1024 / 1024;
    }
    console.log(usage);
};
function loadIndexes(searchManager, limit) {
    let count = 0;
    let buffer = [];
    console.log("Indexer Starting");
    let fileStream = fs_1.default.createReadStream('../search_index/ordered_titles.json');
    const pipeline = (0, stream_chain_1.chain)([
        fileStream,
        (0, stream_json_1.parser)(),
        (0, StreamArray_1.streamArray)(),
        data => ({ title: data.value })
    ]);
    let statusInterval = setInterval(() => {
        // good_mem(process.memoryUsage());
        console.log(`${(((count / limit) * 10000) | 0) / 100} `.padEnd(4, "0") + "%");
    }, 500);
    pipeline.on('data', (data) => {
        buffer.push(data);
        count++;
        if (buffer.length == 100) {
            searchManager.insert([...buffer]);
            buffer.length = 0;
            if (count >= limit) {
                console.log("100%");
                clearInterval(statusInterval);
                good_mem(process.memoryUsage());
                pipeline.destroy();
            }
        }
        // if (count == 100000) {
        //     console.log("TOOK:", performance.now() - start);
        //     clearInterval(statusInterval);
        // }
    });
    pipeline.on("end", () => {
        console.log("DONE PARSING");
        clearInterval(statusInterval);
    });
}
exports.default = loadIndexes;
