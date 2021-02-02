importScripts('rust/gameboy_emulator.js');
delete WebAssembly.instantiateStreaming;

wasm_bindgen("rust/gameboy_emulator_bg.wasm")
  .then((wasm) => {
    const {check_x} = wasmFractal;
    onmessage = function(msg) {
      switch(msg.data.type) {
      case "checkx":
        break;
      }
    };
    postMessage({type: "init", value: true});
  })
  .catch( _ => {
    postMessage({
      type: "init",
      value: false,
      reason: "failed to fetch and instantiate the WASM"
    });

  });

const log = () => call(["console", "error"], arguments)
const error = () => call(["console", "error"], arguments)

function call(fn, args) {
  postMessage({
    type: "call",
    command: fn,
    args: Array.prototype.slice.call(args)
  })
}
