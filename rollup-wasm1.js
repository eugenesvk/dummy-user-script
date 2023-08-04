import path      	from 'path';
import {wasm}    	from '@rollup/plugin-wasm' // bundle wasm as base64 strings
import userscript	from 'rollup-plugin-userscript';
import pkg       	from './package.json' assert {type:'json'};
//               	Rust build
import rust      	from "@wasm-tool/rollup-plugin-rust"
import serve     	from "rollup-plugin-serve"
import livereload	from "rollup-plugin-livereload"
import {terser}  	from "@rollup/plugin-terser"

const is_watch = !!process.env.ROLLUP_WATCH;

const DIST    	= 'dist';
const FILENAME	= 'wasm1';

const wasmOpt = { // null in sync raises an Cannot read properties of null (reading 'map')
  sync           	: []               	,//|null|                      	Array[...String]	each string represent a WebAssembly file to load synchronously
  maxFileSize    	: 14336            	,//|14336|(14kb)               	Number          	for inline files, above limit → copy to dest folder and load from a separate file at runtime. 0 → copy all files except for `sync` (always inlined)
  fileName       	: '[hash][extname]'	,//|'[hash][extname]'|         	String          	This option can be used to rename the emitted Wasm files. It accepts the following string replacements:
    //[hash]     	                   	 hash value of the             	                	file's contents
    //[name]     	                   	 name      of the imported file	                	(without extension)
    //[extname]  	                   	 extension of the imported file	                	(including the leading .)
  publicPath     	: ""               	,//|""|                        	String          	add in front of copied filenames
  targetEnv      	: "auto"           	,//|auto|browser¦node¦         	                	what code is emitted to instantiate the Wasm (both inline and separate)
    //auto       	                   	 will determine the environment at runtime and invoke the correct methods accordingly
    //auto-inline	                   	 always inlines the Wasm and will decode it according to the environment
    //browser    	                   	 omit emitting code that requires node.js builtin modules that may play havoc on downstream bundlers
    //node       	                   	 omit emitting code that requires fetch
}
let bundleOpt = {
  extend                 	: true,
  esModule               	: true,
  indent                 	: false,
  // externalLiveBindings	: false, // with false circular dependencies and live bindings for external imports won't work
}

const cfg1 =[{
input  	: {index:"./Cargo.toml",},
output 	: {dir:"dist/js",format:"iife",sourcemap:true,},
plugins	: [
  rust({serverPath:"js/",}),
  is_watch && serve({contentBase:"dist",open:true,}),
  is_watch && livereload("dist"),
 !is_watch && terser(),
],
}]

const cfg = {
input  	: 'src/wasm1.js',
output 	: {format:'es',file:`${DIST}/js/${FILENAME}.user.js`
 ,     	   globals:{...bundleOpt},sourcemap:true,},
plugins	: [wasm(wasmOpt),
  userscript(path.resolve('src/UserScript meta.js'),meta => meta
    .replace('process.env.VERSION', pkg.version)
    .replace('process.env.AUTHOR' , pkg.author ))
  ]
}

export default cfg
