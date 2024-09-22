import alt from "alt-client"

const U64_MAX = 2n ** 64n - 1n

let current_generation_id = 1n

alt.on("baseObjectCreate", (base_object) => {
  if (base_object.isRemote) return
  base_object.generation_id = current_generation_id
})

alt.on("baseObjectRemove", () => {
  if (base_object.isRemote) return

  if ((current_generation_id + 1n) > U64_MAX) {
    alt.logError(
      "Client-side base object generation reached u64::MAX.\n" +
        "Next base object will use non-unique generation.\n" +
        "Consider opening issue in altv-rust repo: https://github.com/xxshady/altv-rust/issues.",
    )
    current_generation_id = 0n
  }
  current_generation_id += 1n
})
