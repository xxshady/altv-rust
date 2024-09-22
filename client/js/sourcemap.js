import alt from "alt-client"
import { SourceMapConsumer } from "source-map"

export async function init() {
  const sourcemap = alt.File.read("/client/wasm.map", "utf-8")
  const wasmMappings = alt.File.read("/client/mappings.wasm", "binary")
  SourceMapConsumer.initialize({ "lib/mappings.wasm": wasmMappings })

  const consumer = await new SourceMapConsumer(sourcemap)

  Error.prepareStackTrace = (err, frames) => {
    return err.stack.split("\n").map((frameStr, idx) => {
      if (!frameStr.includes("wasm://")) return frameStr

      const frame = frames[idx - 1]
      alt.Utils.assert(frame != null)

      const column = frame.getColumnNumber()
      alt.Utils.assert(column != null)

      const original = consumer.originalPositionFor({ line: 1, column: column - 1 })
      if (original?.line == null) {
        return frameStr
      }

      return frameStr.replace(
        /\(wasm:\/\/.*\)/,
        `(${original.source}:${original.line}:${original.column + 1})`,
      )
    }).join("\n")
  }
}
