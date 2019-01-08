The idea of this repository is to demonstrate some of the cool and useful features of rust. In this example I hope to demonstrate how you can use the same rust code to run in wildly different environments. The web-browser, via web assembly, and regular machines (GNU+Linux in my case). On the extreme end of the spectrum, rust can also run on bare metal micro-controllers (not covered in this example). All this freedom with the powerful abstractions, safety and speed of rust. 

This is a small and simple example of converting a CSV to JSON in a web app, and a CLI app using the same shared code.

##  Notes
This was done on a GNU+Linux machine, it should work out of the box on pretty much any *nix. I do not have a Windows machine to test with, but I can't think of obvious reasons it shouldn't work on windows (besides the fact that the configuration script is in bash). With that in mind, this guide is written in mind for *nix, but should give an indication of what to do on other platforms.

The repo in actual fact contains 3 rust crates and a node project. These could have been split into different repositories, but for the sake of the example it is all merged into one. 
```
offerzen-rust-example (rust crate) CLI application
|
|---> csv-to-json (rust crate) shared logic for CLI app and wasm
|
|---> offerzen-wasm (rust crate)
  |
  |-----> www (node project) dev webserver and small project to showcase wasm
```

## Prerequisites

1. rust -> https://www.rust-lang.org/tools/install
2. install the nightly toolchain of rust
```
rustup toolchain add nightly
```
3. node + npm -> https://nodejs.org/en/download/ or from your favorite package manager or https://github.com/creationix/nvm if you need to manage multiple versions of node. Or however else you'd like to install node
4. wasm-pack -> https://rustwasm.github.io/wasm-pack/installer/

## Guide

1. Clone this repo
```
git clone https://github.com/ThorburnT/offerzen-rust-example.git
```
2. Enter the directory 
```
cd offerzen-rust-example
```
3. Build the cli app
```
cargo +nightly build --release
```
4. Run the configuration script for the web assembly part. This mainly does some node related things which aren't entirely in the scope for this example but necessary for it to work, and builds the  `offerzen-wasm` crate using the `wasm-pack` utility which builds the crate using rust and generates all the needed glue code between rust and javascript. It uses the built in rust target `wasm32-unknown-unknown` to generate the web assembly. Feel free to inspect the contents of this script if you wish to understand what it is doing
```
bash configure.sh
```
5. Everything should now be built and properly setup if everything went well. Time to test

The CLI app
```
cat example.csv | ./target/release/offerzen-rust-example
```
You should now have some JSON spit out in your terminal.

The wasm web app
```
cd offerzen-wasm/www/
npm start
```
now navigate to the url webpack gives you in the output. Mine was http://localhost:8081/


Paste a csv (with headers) into the first text box, and hit jsonify. (You can find some example csv in the repo, example.csv) Verify the output is valid json. Pretty cool, right? (Can you tell I am a backend developer?)

6. Tinker with it. 