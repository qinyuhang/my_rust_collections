/* tslint:disable */
/* eslint-disable */
/**
* The start or the main fn in wasm
*/
export function start(): void;
/**
* @param {number} width
* @param {number} height
*/
export function generate_game(width: number, height: number): void;
/**
* @param {number} key
*/
export function handle_key(key: number): void;
/**
*/
export function next_frame(): void;
/**
* @param {number} scale
*/
export function set_scale(scale: number): void;
/**
* @returns {Uint8Array}
*/
export function get_png(): Uint8Array;
/**
* @returns {Uint8Array}
*/
export function get_png_with_scale(): Uint8Array;
/**
* @param {number} width
* @param {number} height
*/
export function resize(width: number, height: number): void;
/**
* @returns {boolean}
*/
export function get_is_finshed(): boolean;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly start: () => void;
  readonly generate_game: (a: number, b: number) => void;
  readonly handle_key: (a: number) => void;
  readonly next_frame: () => void;
  readonly set_scale: (a: number) => void;
  readonly get_png: (a: number) => void;
  readonly get_png_with_scale: (a: number) => void;
  readonly resize: (a: number, b: number) => void;
  readonly get_is_finshed: () => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__Fn_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd1e0b83fd91f47f3: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__Fn__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h93d1072a9911520f: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hb14350cc4da81db3: (a: number, b: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_start: () => void;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
