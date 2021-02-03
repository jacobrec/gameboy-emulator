interface Wasm {
        check_x: Function,
        update: Function,
}
export default class Emulator {
    wasm: Wasm | null = null
    constructor() {
        (async () => {
          const wasm = await import("rust/gameboy_emulator_bg.wasm");
          this.wasm = wasm;
        })()
    }

    check_x(): number {
        return this.wasm?.check_x();
    }

    update(): number {
        // TODO: something with CPU timing
        return this.wasm?.update(3);
    }

}
