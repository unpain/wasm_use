<template>
  <div class="input_container">
    <input type="text" v-model="a">+
    <input type="text" v-model="b"> =
    <input type="text" v-model="res">
  </div>
</template>
<script setup lang="ts">
import { ref, watch } from 'vue'
import { useWasm } from '@/hooks/useWasm'
const a = ref(0)
const b = ref(0)
const res = ref(0)
const fuc = ref()
/**
 * @author fangshuo
 * 本方法使用js的WebAssembly.instantiateStreaming api取出由rust语言编写并构建的wasm文件中的方法并调用该方法
 */
const wasmFetch = (): void => {
  fuc.value = useWasm('/wasm_build/pkg/wasm_build_bg.wasm')
  res.value = fuc.value.add(a.value, b.value)
}
watch(() => [a.value, b.value], () => {
  wasmFetch()
})
</script>
<style scoped>
.input_container {
  display: flex;
}
</style>
