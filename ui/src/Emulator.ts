interface Wasm {
    button_down: Function,
    button_up: Function,
    update: Function,
    init: Function,
}

export enum Button {
    Start = 0,
    Select,
    DUp,
    DDown,
    DLeft,
    DRight,
    A,
    B
}

export default class Emulator {
    wasm: Wasm | null = null

    constructor() {
        (async () => {
          await import("rust/gameboy_emulator_bg.wasm");
          const wasm = await import("rust/gameboy_emulator");
          this.wasm = wasm;
        })()
    }

    load_rom(data: Uint8Array) {
        let w: any = window;
        if (!w.has_loaded) {
            this.wasm?.init(data);
            w.has_loaded = true;
        }
        w.button_down = (e: Button) => this.button_down(e)
        w.button_up = (e: Button) => this.button_up(e)
    }

    button_down(b: Button): number {
        return this.wasm?.button_down(b);
    }
    button_up(b: Button): number {
        return this.wasm?.button_up(b);
    }

    update() {
        // TODO: something with CPU timing
        let cycles_per_frame = 70256;
        let cycles_per_second = 4194304;
        return this.wasm?.update(70256 / 2);
    }

}
