import { type Ref } from 'vue'
const useWasm = (url: string, target: Ref) => {
  if (!target.value) {
    WebAssembly.instantiateStreaming(fetch(url)).then((obj: WebAssembly.WebAssemblyInstantiatedSource) => {
      target.value = obj.instance.exports
      console.log(target.value)
    })
  }
}
export { useWasm }
