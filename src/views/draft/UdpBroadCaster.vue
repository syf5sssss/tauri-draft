<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event'


let msg = ref("");
let data = ref("");
let str = ref("");
let port = ref(9988);
let target_port = ref(9998);

async function conn() {
  let res = await invoke('open_broadcast_service', { port: parseInt(port.value) });
  console.log(res);
}

async function send_client() {
  await invoke('send_broadcast_message', { message: str.value, port: parseInt(target_port.value) });
}

async function disconn() {
  await invoke('close_broadcast_service');
}

listen('broadcast-message', (event) => {
  console.log(event.payload);
  data.value = event.payload;
});

</script>

<template>
  <div class="card">
    <p>Udp BroadCaster</p>
    <p>{{ msg }}</p>
    <p>{{ data }}</p>
    <InputText v-model="port" class="ml-2 mt-2 w-48" />
    <Button @click="conn" class="ml-2 w-48">连接</Button>
    <Button @click="disconn" class="ml-2 w-48">关闭</Button>
    <br>
    <InputText v-model="str" class="ml-2 mt-2 " fluid />
    <br>
    <InputText v-model="target_port" class="ml-2 mt-2 w-48" />

    <br>
    <Button @click="send_client" class="ml-2 mt-2 " fluid>发送</Button>
    <br>
    <pre>
      <code>




        广播
        地址范围 255.255.255.255
        同一子网所有设备
      </code>
    </pre>
  </div>

</template>

<style></style>