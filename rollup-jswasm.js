import path      	from 'path'
import userscript	from 'rollup-plugin-userscript'
import pkg       	from './package.json' assert {type:'json'}
//               	Rust build
import serve     	from "rollup-plugin-serve"
import livereload	from "rollup-plugin-livereload"
import terser    	from "@rollup/plugin-terser"

const is_watch = !!process.env.ROLLUP_WATCH;

const DIST    	= 'dist';
const FILENAME	= 'wasm1';

const terserOpt = {
  format    	: {
    comments	: "all", // preserve UserScript comments |some| keeps JSDoc-style comments that contain "@license", "@copyright", "@preserve" or start with ! ¦true¦all¦ preserve all comments ¦false¦ omit comments, a regular expression string (e.g. /^!/) or a function
  },
}

const cfg =[
{input 	: {'index.user':"./src/user_script_wasm.js",},
output 	: {format:"es",dir:`${DIST}/js`},
plugins	: [
  is_watch && serve({contentBase:"dist",open:true,}),
  is_watch && livereload("dist"),
 !is_watch && terser(terserOpt),
  userscript(path.resolve('src/UserScript meta.js'),meta => meta
    .replace('process.env.VERSION', pkg.version)
    .replace('process.env.AUTHOR' , pkg.author ))
  ]
},
]

export default cfg
