/*
// ↓ added via userscript rollup plugin to make Violentmonkey recognize this as a user script
// ==UserScript==
// ==/UserScript==

// 0 Prep: convert our was base64 text to a buffer
const wasm_base64 = 'AGFzbQEAAAABhYCAgAABYAABfwOCgICAAAEABISAgIAAAXAAAAWDgICAAAEAAQaBgICAAAAHkYCAgAACBm1lbW9yeQIABG1haW4AAAqKgICAAAGEgICAAABBKgs='
let buf = null;
let raw = globalThis.atob(wasm_base64); // atob() function decodes a string of data which has been encoded using Base64 encoding
let rawLength = raw.length;
buf = new Uint8Array(new ArrayBuffer(rawLength));
for(let i = 0; i < rawLength; i++) {   buf[i] = raw.charCodeAt(i);}
console.log("wasm_base64 → Array¦=", buf)

// 1 Sync way of getting a module
const wasmMod  = WebAssembly.Module
const wasmInst = WebAssembly.Instance
const importObject = {imports:{imported_func(arg){console.log(arg);},},};
const wasmSourceBuffer	= buf
const wasmModule      	= new WebAssembly.Module  (wasmSourceBuffer) //Creates a new Module object
const wasmInstance    	= new WebAssembly.Instance(wasmModule,importObject) // object is a stateful, executable instance of a WebAssembly.Module, contain all the Exported WebAssembly functions that allow calling into WebAssembly code from JavaScript
  // (optional)importObject containing the values to be imported into the newly-created Instance
    // functions or WebAssembly.Memory objects
    // There must be one matching property for each declared import of module or else a WebAssembly.LinkError is thrown
const wasmInstanceExports = wasmInstance.exports // object containing as its members all the functions exported from the WebAssembly module instance, to allow them to be accessed and used by JavaScript. Read-only.
console.log("wasmInstance.exports.main()¦=", wasmInstanceExports.main());
// The WebAssembly.Instance() constructor function can be called to synchronously instantiate a given WebAssembly.Module object, for example:

// 2 Async way of getting a module
  // The preferred way to get an Instance is asynchronously, for example using the WebAssembly.instantiateStreaming() function like this:
  // WebAssembly.instantiateStreaming(source, importObject) function compiles and instantiates a WebAssembly module directly from a streamed underlying source. This is the most efficient, optimized way to load Wasm code
    // A Response object (interface of the Fetch API represents the response to a request)
    // a promise that will fulfill with one, representing the underlying source of a Wasm module you want to stream, compile, and instantiate
// WebAssembly.instantiateStreaming(raw).then(
//   (obj) => obj.instance.exports.main(),);

import sample from './sample.wasm';
// FAILS doc example with {instance}
// sample().then(({ instance }) => {console.log(instance.exports.main());});

// FINALY WORKS! was some mistake with {instance}
// const load_wasm_mod = sample()
// console.log("sample()", load_wasm_mod)
// const load_wasm_mod_instantiate = WebAssembly.instantiate(load_wasm_mod)
// console.log("WebAssembly.v(sample())", load_wasm_mod_instantiate)
// WebAssembly.instantiateStreaming(raw).then(
  // (obj) => obj.instance.exports.main(),);
// sample().then(({instance}) => { // instance = undefined
sample().then((wasmModule) => { // wasmModule = WebAssembly.Module
  console.log("wasmModule¦=",wasmModule);
  const wasmInstance	= new WebAssembly.Instance(wasmModule)
  console.log(wasmInstance.exports.main()); // 42
});

const load_wasm_mod = sample()
console.log("sample()", load_wasm_mod)
const load_wasm_mod_instantstream = WebAssembly.instantiateStreaming(load_wasm_mod) // can't work since we'd need a fetch

bug https://github.com/rollup/plugins/issues/1551
var compileFunc = stream ? WebAssembly.compileStreaming : WebAssembly.compile;
 A Promise that resolves to a WebAssembly.Module object representing the compiled module.

var instantiateFunc = stream ? WebAssembly.instantiateStreaming : WebAssembly.instantiate;
  → A Promise that resolves to a ResultObject which contains two fields:
  module: A WebAssembly.Module object representing the compiled WebAssembly module. This Module can be instantiated again, shared via postMessage(), or cached.
  instance: A WebAssembly.Instance object that contains all the Exported WebAssembly functions.

https://wasdk.github.io/WasmFiddle/
wasm1_42.wasm
int main() {               return 42;}
wasm2_44_log44.wasm
void import_log(int arg);
int main() {import_log(44);return 44;}
→ (import "env" "import_log" (func $import_log (param i32)))
so use 'env' instead of 'imports' in JS
*/

import wasm1 from './wasm/wasm1_42.wasm'
import wasm2 from './wasm/wasm2_44_log44.wasm'
const wasmMem = new WebAssembly.Memory({ initial: 10, maximum: 100 })
const wasmOpt = {
  // js: {mem:wasmMem},
  env         	:{ // env is default from wasmfiddle, not imports
    import_log	: (arg) => console.log(arg)
  }
};

wasm1({}     ).then(({ instance }) => {console.log(instance.exports.main())})
wasm2(wasmOpt).then(({ instance }) => {console.log(instance.exports.main())})
