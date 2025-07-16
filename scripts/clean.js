import fs from "fs/promises";
import path from "path";
import { OUT_SRC } from "./utils.js";
import { IGNOREDFILES } from "./ignored.js";

async function* walk(dir) {
  for (const dirent of await fs.readdir(dir, { withFileTypes: true })) {
    const p = path.join(dir, dirent.name);
    if (dirent.isDirectory()) yield* walk(p);
    else if (dirent.isFile()) yield p;
  }
}

export async function clean() {
  const targetDirs = [OUT_SRC];
  console.log(OUT_SRC);
  for (let dir of targetDirs) {
    for await (const file of walk(dir)) {
      if (IGNOREDFILES.has(path.basename(file))) continue;
      await fs.writeFile(file, "use super::*;\n\n", "utf8");
      //   console.log(`Trimmed ${path.relative(dir, file)}`);
    }
  }
}
