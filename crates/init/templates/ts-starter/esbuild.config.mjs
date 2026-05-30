import * as esbuild from "esbuild";
import * as fs from "node:fs";
import * as path from "node:path";

const args = process.argv.slice(2);
const watch = args.includes("--watch");

const config = {
  entryPoints: ["scripts/main.ts"],
  outfile: "behavior_packs/pack0/scripts/main.js",
  bundle: true,
  minify: true,
  platform: "node",
  treeShaking: true,
  format: "esm",
  sourcemap: "external",
  logLevel: "info",
  external: ["@minecraft/server", "@minecraft/server-ui"],
  write: false,
};

async function main() {
  if (watch) {
    const watcher = fs.watch("scripts");
    watcher.addListener("change", async () => {
      try {
        build();
      } catch (error) {
        console.log(error);
      }
    });
  } else {
    build();
  }
}

async function build() {
  const result = await esbuild.build(config);
  const sourceMapPath = "./dist/debug/main.js.map";

  for (let out of result.outputFiles) {
    if (path.extname(out.path) === ".js") {
      fs.mkdirSync(path.dirname(out.path), { recursive: true });
      fs.writeFileSync(out.path, out.text);
    } else {
      fs.mkdirSync(path.dirname(sourceMapPath), { recursive: true });
      fs.writeFileSync(sourceMapPath, out.text);
    }
  }
}

main().catch(() => process.exit(1));
