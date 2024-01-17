import init from "./pkg/taguchi_tables.js";
import wasm_blob from "url:./pkg/taguchi_tables_bg.wasm";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await init(wasm_blob);

  console.log("Here2");

  helloWorld.taguchi({ something: 1 } as any);

  //   // Call the Add function export from wasm, save the result
  //   const addResult = helloWorld.add(24, 24);

  //   // Set the result onto the body
  //   document.body.textContent = `Hello World! addResult: ${addResult}`;
};

runWasm();

const form: HTMLFormElement = document.getElementById("config")! as HTMLFormElement;

function handle_submit(e: SubmitEvent) {
  e.preventDefault();

  const formData = new FormData(form);

  let fields = {};

  // <https://stackoverflow.com/a/46774073/383609>
  formData.forEach((value, key) => {
    // Reflect.has in favor of: object.hasOwnProperty(key)
    if (!Reflect.has(fields, key)) {
      fields[key] = value;
      return;
    }

    if (!Array.isArray(fields[key])) {
      fields[key] = [fields[key]];
    }

    fields[key].push(value);
  });

  console.log(fields);
}

form.addEventListener("submit", handle_submit);
