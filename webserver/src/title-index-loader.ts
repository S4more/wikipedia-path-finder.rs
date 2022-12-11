import { parser } from "stream-json";

import { chain } from "stream-chain";
import { streamArray } from "stream-json/streamers/StreamArray";

import fs from "fs";
import SearchManager from "./search";

const good_mem = (usage: any) => {
    for (let key in usage) {
        usage[key] = usage[key] / 1024 / 1024;
    }
    console.log(usage);
}

export default function loadIndexes(searchManager: SearchManager, limit: number) {
    let count = 0;
    let buffer: { title: string }[] = [];
    console.log("Indexer Starting")
    let fileStream = fs.createReadStream('./search_index/ordered_titles.json')
    const pipeline = chain([
        fileStream,
        parser(),
        streamArray(),
        data => ({ title: data.value })
    ]);

    let statusInterval = setInterval(() => {
        // good_mem(process.memoryUsage());
        console.log(`${(((count / limit) * 10000) | 0) / 100} `.padEnd(4, "0") + "%");
    }, 500)


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
    })

    pipeline.on("end", () => {
        console.log("DONE PARSING")
        clearInterval(statusInterval);
    });
}
