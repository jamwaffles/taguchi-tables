import init from "./pkg/taguchi_tables.js";
import wasm_blob from "url:./pkg/taguchi_tables_bg.wasm";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init(wasm_blob);

  console.log("Here");

  helloWorld.greet();

  //   // Call the Add function export from wasm, save the result
  //   const addResult = helloWorld.add(24, 24);

  //   // Set the result onto the body
  //   document.body.textContent = `Hello World! addResult: ${addResult}`;
};

runWasm();
