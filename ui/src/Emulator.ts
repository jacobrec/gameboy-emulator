interface Wasm {
    press_button: Function,
    get_screen: Function,
    update: Function,
}

enum Button {
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
          const wasm = await import("rust/gameboy_emulator_bg.wasm");
          this.wasm = wasm;
        })()
    }

    press_button(b: Button): number {
        return this.wasm?.press_button(b);
    }

    get_screen(): number {
        return this.wasm?.get_screen();
    }

    update(): number {
        // TODO: something with CPU timing
        return this.wasm?.update(3);
    }

}
