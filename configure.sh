#!/bin/bash
cd offerzen-wasm/www
npm install
cd ..
wasm-pack build
cd pkg
npm link
cd ../www
npm link offerzen-wasm


