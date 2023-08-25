import * as wasm from "./safeson_bg.wasm";
import { __wbg_set_wasm } from "./safeson_bg.js";
__wbg_set_wasm(wasm);
export * from "./safeson_bg.js";
