import { build, emptyDir } from "https://deno.land/x/dnt@0.37.0/mod.ts";

const version = Deno.args[0];
if (!version) {
  console.error("Version not provided.");
  Deno.exit(1);
}

const entryPoint = Deno.args[1];
if (!entryPoint) {
  console.error("Entry point not provided.");
  Deno.exit(1);
}

await emptyDir("./dist");

await build({
  entryPoints: [entryPoint],
  outDir: "./dist",
  typeCheck: "both",
  test: false,
  shims: {},
  compilerOptions: {
    lib: ["ESNext", "DOM"],
  },
  packageManager: "pnpm",
  package: {
    name: "safeson-js",
    version,
    description: "A JSON-compatible serialization format",
    author: "Roj <ez@roj.im>",
    license: "AGPL-3.0-or-later",
    repository: {
      type: "git",
      url: "git+https://github.com/roj1512/SafeSON.git",
    },
  },
  postBuild() {
    Deno.copyFileSync("LICENSE", "dist/LICENSE");
  },
});
