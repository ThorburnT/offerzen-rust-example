import * as wasm from "offerzen-wasm";

function convert() {
    var input = document.getElementById("input").value;
    var result = wasm.c_to_j(input);
    document.getElementById("output").value = result;
}

let btn = document.getElementById("conv");
btn.onclick = function() {
  convert();
} 