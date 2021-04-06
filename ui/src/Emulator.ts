interface Wasm {
    press_button: Function,
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
        this.wasm?.init(data);
    }

    press_button(b: Button): number {
        return this.wasm?.press_button(b);
    }

    update() {
        // TODO: something with CPU timing
        let cycles_per_frame = 70256;
        let cycles_per_second = 4194304;
        return this.wasm?.update(70256 / 2);
    }

}
