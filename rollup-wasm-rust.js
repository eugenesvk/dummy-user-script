import path      	from 'path'
import userscript	from 'rollup-plugin-userscript'
import pkg       	from './package.json' assert {type:'json'}
//               	Rust build
import rust      	from "@wasm-tool/rollup-plugin-rust"
import serve     	from "rollup-plugin-serve"
import livereload	from "rollup-plugin-livereload"
import terser    	from "@rollup/plugin-terser"

const is_watch = !!process.env.ROLLUP_WATCH;

const DIST    	= 'dist';
const FILENAME	= 'wasm1';

const rustOpt = {
  inlineWasm     	: true      	,// inline `.wasm` into `.js` Slower, size +33%, but no separate `.wasm` file. `true` → `serverPath` `nodejs` `importHook` ignored
  serverPath     	: "js/"     	,// server dir to load `.wasm` from. This is prepended to the URL, so you should put a / at the end of the directory, for example "/foo/".
  nodejs         	: false     	,// whether code will be run in Node (which doesn't support `fetch`)
  debug          	: false     	,// debug/release build (watch mode = debug)
  verbose        	: false     	,// display extra compilation information in the console
  cargoArgs      	: []        	,// extra args to `cargo build`
  wasmBindgenArgs	: []        	,// extra args to `wasm-bindgen`
  wasmOptArgs    	: ["-O"]    	,//       args to `wasm-opt`
  watchPatterns  	: ["src/**"]	,// files for watch mode. Relative to Cargo.toml, syntax npmjs.com/package/glob
  importHook     	: function (path) { return JSON.stringify(path)},  // customize the behavior for loading the .wasm file (advanced)
  experimental   	: {    	//
    directExports	: false	,// Changes the way that the modules are generated from ↓ (might need to set the Rollup `format` to "es" or "system")
     //   import wasm from "./path/to/Cargo.toml";
     //   async function loadWasm() { const exports = await wasm(); /* Use functions which were exported from Rust... */}
     // to ↓
     //   import { foo, bar } from "./path/to/Cargo.toml";    // Use functions which were exported from Rust...
    synchronous	: false	,// init Wasm synchronously. In the browser you can only use synchronous loading inside of Workers. requires `inlineWasm	: true`
 },
}
let bundleOpt = {
  extend                 	: true,
  esModule               	: true,
  indent                 	: false,
  // externalLiveBindings	: false, // with false circular dependencies and live bindings for external imports won't work
}

const cfg =[{
input  	: {index:"./Cargo.toml",},
output 	: {format:"iife",file:`${DIST}/js/${FILENAME}.user.js`,sourcemap:true,},
plugins	: [
  rust(rustOpt),
  is_watch && serve({contentBase:"dist",open:true,}),
  is_watch && livereload("dist"),
 !is_watch && terser(),
  userscript(path.resolve('src/UserScript meta.js'),meta => meta
    .replace('process.env.VERSION', pkg.version)
    .replace('process.env.AUTHOR' , pkg.author ))
  ]
}]

export default cfg
