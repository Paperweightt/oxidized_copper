import * as esbuild from "esbuild";

const args = process.argv.slice(2);
const watch = args.includes("--watch");

const config = {
    entryPoints: ["scripts/main.ts"],
    outfile: "behavior_packs/Neon_Knockout/scripts/main.js",
    bundle: true,
    minify: true,
    platform: "node",
    treeShaking: true,
    format: "esm",
    sourcemap: false,
    logLevel: "info",
    external: ["@minecraft/server", "@minecraft/server-ui"],
};

async function build() {
    if (watch) {
        const ctx = await esbuild.context(config);
        await ctx.watch();
        console.log("Watching for changes...");
    } else {
        await esbuild.build(config);
        console.log("Build complete!");
    }
}

build().catch(() => process.exit(1));
