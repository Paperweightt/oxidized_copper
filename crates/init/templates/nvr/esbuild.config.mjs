import * as esbuild from "esbuild";
import { spawn } from "node:child_process";
import chokidar from "chokidar";
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
    chokidar.watch("./scripts").on("change", build);
  }
  build();
}

async function build() {
  esbuild
    .build(config)
    .then((result) => {
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
      notifyNvim("Esbuild suceded");
    })
    .catch((error) => {
      if (error.errors) {
        error.errors.forEach((err) => {
          notifyNvim(`Location: ${err.location?.file}:${err.location?.line}`, 3);
        });
      }
    });
}

function notifyNvim(message, level = 0) {
  spawn("nvr", ["-c", `lua require("notify")("${message}",${level})`]);
}

main().catch(() => process.exit(1));
