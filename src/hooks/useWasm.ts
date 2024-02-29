let wasm: WebAssembly.Exports | undefined
const useWasm = (url: string) => {
  if (!wasm) {
    WebAssembly.instantiateStreaming(fetch(url)).then((obj: WebAssembly.WebAssemblyInstantiatedSource) => {
      wasm = obj.instance.exports
    })
  }
  return wasm
}
export { useWasm }
