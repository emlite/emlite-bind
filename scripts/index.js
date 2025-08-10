import * as webref from "@webref/idl";
import { generate } from "./gen.js";

async function main() {
  // Note: clean() disabled - new generators use writeFileSync to completely replace files
  // This matches the wasmbind approach of file replacement rather than truncation
  // await clean();
  const specAst = await webref.parseAll();
  generate(specAst);
  console.log("Generation done!");
}

main().catch((err) => console.log("Generation failed ", err));
