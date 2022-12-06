#!/bin/bash

cd wiki-webapp; 
# npm run build-wasm;
npm run build;
cd ../webserver;
npm run start;

