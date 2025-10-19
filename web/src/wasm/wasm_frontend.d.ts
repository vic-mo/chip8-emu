/* tslint:disable */
/* eslint-disable */
export class WasmEmulator {
  free(): void;
  [Symbol.dispose](): void;
  constructor();
  /**
   * Load a ROM from a byte array
   */
  load_rom(rom_data: Uint8Array): void;
  /**
   * Execute one instruction cycle
   */
  tick(): void;
  /**
   * Tick the delay and sound timers
   */
  tick_timers(): void;
  /**
   * Reset the emulator
   */
  reset(): void;
  /**
   * Set key pressed state
   * key: 0-15 (hex keypad)
   * pressed: true if pressed, false if released
   */
  key_press(key: number, pressed: boolean): void;
  /**
   * Get the display buffer as a flat array
   * Returns array of 0s and 1s (64*32 = 2048 elements)
   */
  get_display(): Uint8Array;
  /**
   * Get screen width
   */
  screen_width(): number;
  /**
   * Get screen height
   */
  screen_height(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmemulator_free: (a: number, b: number) => void;
  readonly wasmemulator_new: () => number;
  readonly wasmemulator_load_rom: (a: number, b: number, c: number) => void;
  readonly wasmemulator_tick: (a: number) => void;
  readonly wasmemulator_tick_timers: (a: number) => void;
  readonly wasmemulator_reset: (a: number) => void;
  readonly wasmemulator_key_press: (a: number, b: number, c: number) => void;
  readonly wasmemulator_get_display: (a: number) => [number, number];
  readonly wasmemulator_screen_width: (a: number) => number;
  readonly wasmemulator_screen_height: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
