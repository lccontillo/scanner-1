/* tslint:disable */
/* eslint-disable */
export function find_document(data: ImageData): Quad | undefined;
export function extract_document_shared(width: number, height: number, region: Quad, target_width: number, target_height?: number | null): ImageData;
export function find_document_yuv_shared(width: number, height: number): Quad | undefined;
export function find_document_shared(width: number, height: number): Quad | undefined;
export function apply_filter(data: ImageData, filter_type: string, sign_color_name?: string | null): ImageData;
export function extract_document(data: ImageData, region: Quad, target_width: number, target_height?: number | null): ImageData;
export function alloc(len: number): number;
export class Point {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  x: number;
  y: number;
}
export class Quad {
  free(): void;
  [Symbol.dispose](): void;
  constructor(ax: number, ay: number, bx: number, by: number, cx: number, cy: number, dx: number, dy: number);
  a: Point;
  b: Point;
  c: Point;
  d: Point;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly alloc: (a: number) => number;
  readonly apply_filter: (a: any, b: number, c: number, d: number, e: number) => any;
  readonly extract_document: (a: any, b: number, c: number, d: number) => any;
  readonly extract_document_shared: (a: number, b: number, c: number, d: number, e: number) => any;
  readonly find_document: (a: any) => number;
  readonly find_document_shared: (a: number, b: number) => number;
  readonly find_document_yuv_shared: (a: number, b: number) => number;
  readonly __wbg_get_point_x: (a: number) => number;
  readonly __wbg_get_point_y: (a: number) => number;
  readonly __wbg_get_quad_a: (a: number) => number;
  readonly __wbg_get_quad_b: (a: number) => number;
  readonly __wbg_get_quad_c: (a: number) => number;
  readonly __wbg_get_quad_d: (a: number) => number;
  readonly __wbg_point_free: (a: number, b: number) => void;
  readonly __wbg_quad_free: (a: number, b: number) => void;
  readonly __wbg_set_point_x: (a: number, b: number) => void;
  readonly __wbg_set_point_y: (a: number, b: number) => void;
  readonly __wbg_set_quad_a: (a: number, b: number) => void;
  readonly __wbg_set_quad_b: (a: number, b: number) => void;
  readonly __wbg_set_quad_c: (a: number, b: number) => void;
  readonly __wbg_set_quad_d: (a: number, b: number) => void;
  readonly quad_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
