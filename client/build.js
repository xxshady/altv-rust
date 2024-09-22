import { build } from "esbuild"
import { altvEsbuildRustWasm } from "altv-esbuild-rust-wasm"
import { altvEsbuild } from "altv-esbuild"
import fs from "fs"

await build({
  bundle: true,
  logLevel: "info",
  format: "esm",
  entryPoints: ["./js/main.js"],
  outdir: "./server/resources/rust/client",
  plugins: [
    altvEsbuild({ mode: "client" }),
    altvEsbuildRustWasm({
      target: "client",
      wasmPathForClientRead: "/client/rust_wasm_bg.wasm",
    }),
  ],
})

fs.copyFileSync("./rust_wasm/pkg/rust_wasm_bg.wasm.map", "./server/resources/rust/client/wasm.map")
fs.copyFileSync("./source-map/lib/mappings.wasm", "./server/resources/rust/client/mappings.wasm")

const serverJs = "./server/resources/rust/server.js"
if (!fs.existsSync(serverJs)) {
  fs.writeFileSync(serverJs, "")
}

// TODO: remove it from here since it's unrelated to clientside
fs.copyFileSync("../target/debug/rust_server.dll", "./server/resources/rust-server/server.dll")
