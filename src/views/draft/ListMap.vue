<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';


onMounted(async () => {
  get_config();
})

async function get_config() {
  let res = await invoke('get_config');
  console.log(res);
}

</script>

<template>
  <Button @click="get_config" class="ml-2">报警列表</Button>
</template>

<style>
</style>