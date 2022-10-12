import init, { App } from "rust-constraint-solver-wasm";

const runWasm = async () => {
  const wasm = await init();
  const app = new App();

  //run solver
  app.update_locations();
  //get pointer and create array of f64
  //we know there are 4 values.
  const points_ptr = app.get_points();
  const points = new Float64Array(wasm.memory.buffer, points_ptr, 4);
  console.log({ points });
};

(async () => {
  await runWasm();
})();
