/* tslint:disable */
/* eslint-disable */
/**
* @param {string} image_data_url
* @param {string} level_name
* @returns {Uint8Array}
*/
export function convert_gif(image_data_url: string, level_name: string): Uint8Array;
/**
* @param {string} image_data_url
* @param {string} level_name
* @returns {Uint8Array}
*/
export function convert_image_pattern(image_data_url: string, level_name: string): Uint8Array;
/**
* @param {string} image_data_url
* @param {boolean} should_resize
* @param {number} width
* @param {number} height
* @param {string} level_name
* @returns {Uint8Array}
*/
export function convert(image_data_url: string, should_resize: boolean, width: number, height: number, level_name: string): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly convert_gif: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly convert_image_pattern: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly convert: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
