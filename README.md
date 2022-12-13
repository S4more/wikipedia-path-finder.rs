# Web Development IV 520 Final Project
Finding links between two pages in Wikipedia blazingly fast.

## Specs
The project was made using Rust, Web-Assembly, JavaScript and Wikipedia's data-dump published in October 2022

### Dataset
We are using Wikipedia's official data-dump, posted in October of 2022. 
The original size of the dumb was close to 100 GB, but after parsing out all the useless information, we ended up with a total of ~5 GB of data.
We are storing three different types of data:
1. The name of all 6 million pages on Wikipedia order by number of references towards the page
	We are using the index of the pages as their ID, so we can avoid repeating their names
2. A look-up table indexed by the page position (see 1) and an array of nodes that this page points to
3. A reverse look-up table, with all the nodes that point to a specific page

While the second and third files are redundant, utilising a reverse lookup table allowed us to reduce the boot up time and search time by a lot.

### Rust Path-Finding
We quickly realised that using Node for the path-finding algorithm wouldn't work. Node's performance limitation wouldn't allows us to even parse the original file and for that reason, we decided to go with Rust both for the algorithm and for the parsing.
We managed to make it really fast and for a single request, it can most of the time figure out the path in less than 10 ms.
We didn't had the time to test it on the entire Wikipedia, but we haven't found a single combination of pages that can't be found.

### Rust API
The API is integrated with our path-finding. We are using a crate ~library~ called Rocket so the Web and rust can exchange data.
There is a single endpoint that allows the user to specify the source page, the destination and the number of maximum number of hops.

### Visualisation
#### Tools used
 - [Bevy](https://bevyengine.org/) (A WASM compatible rust game engine)
 - [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) (Allowing the calling of rust code from JS)

The visualizer runs client side WASM, rendering using webgl2 to a canvas.

The node repulsion/connection system was written to support the ECS design used by Bevy, this makes it quite performant, and means that it will be able to take full advantage of any future web-concurrency support. 

The simulation uses basic 2D spatial hash implementation for culling collision checks, because the repulsion effect between nodes we end up with a very uniform density of nodes (which is ideal for the spatial hash). This means the simulation complexity scales linearly as the number of nodes and constraints increases. 

The largest benefit of the WASM implementation was not the raw speed increase, the native JS implementation was competitive when it came to average speed. However, even after applying extensive garbage reduction optimizations. The JavaScript version suffered form major stuttering every few seconds due to garbage collection. The WASM implementation using a fixed memory buffer has nearly perfect frame-time consistency.

Images are all loaded through the proxy server.
The server is responsible for

    - Fetching the images from wikipedia
    - Resizing them to a fixed size
    - Rounding the corners
    - Rasterizing them to standard format

This is all necessary since the images fetched form Wikipedia come in a variety of sizes and formats, which would be difficult for the game engine to render.

The simulation uses [verlet integration](https://en.wikipedia.org/wiki/Verlet_integration) to compute node motion and handle constraints. This method has very low overhead and was easy to implement computationally.

## Considerations
### JSON as database
We decided to not implement any formal database, since all the data needs to be loaded in memory. Fetching for the nodes from a database would result in millions of query per second and it would make the search too slow.
The initial load consists of 18_750_000 readings. Utilising multi-thread, with Rust, we managed to perform the entire load in under 15 seconds.

### Bugs
The algorithm is slow to test. For every query, we need to open Wikipedia ourselves and make sure that we got a valid path. For this reason, while the path is always valid, the output of the path may be wrong sometimes. 
We also didn't implement any loop correction, so eventually paths will have references to itself. 

### Lack hosting
Because our algorithm needs to allocate 8 GBs of RAM at the start, we couldn't find any solutions that would allow us to host it. We are still waiting for Oracle's e-mail so we can host it there.

## Running it yourself
(You need at least 16 gbs of memory to run it.)

1. Download and extract the dataset by running `bash download_indexs.sh`
   1. If this script does not work, download the required files directly from this (link)[https://drive.google.com/drive/folders/1ISFHh8L3-WUSBfhkbS5YpXfg-4m586WU]
   2. All files should be unzipped and placed into `search_index`
2. Run the server `cd webserver; npm run start` (you will maybe need to install the dependencies first) 
3. Run `.\path_finder.exe -t search_index/ordered_titles.json -p search_index/test_links.json -m server -i search_index/incoming_links.json`
4. Have fun!

***Warning this script will require ~1.2gb of download bandwidth and ~4.2gb of disk space***

## Team-7

 - Mauricio Murillo
 - Guilherme Machado
 - Noah Labrecque

