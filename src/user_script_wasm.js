import * as wasm from "./user_script_wasm_bg.wasm";
import { __wbg_set_wasm } from "./user_script_wasm_bg.js";
__wbg_set_wasm(wasm);
export * from "./user_script_wasm_bg.js";

wasm.__wbindgen_start();
