/* @ts-self-types="./mdkit.d.ts" */

import * as wasm from "./mdkit_bg.wasm";
import { __wbg_set_wasm } from "./mdkit_bg.js";
__wbg_set_wasm(wasm);
wasm.__wbindgen_start();
export {
    markdown_to_html
} from "./mdkit_bg.js";
