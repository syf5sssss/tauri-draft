<script setup>
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event'


let msg = ref("");
let data = ref("");
let str = ref("");
let ip = ref("0.0.0.0");
let port = ref(9988);
let target_port = ref(9998);
let multicast_addr = ref("232.252.252.252");

async function conn() {
  await invoke('open_udp_service', { addr: ip.value + ":" + port.value, multicastAddr: multicast_addr.value });
}

async function send_client() {
  await invoke('send_udp_message', { message: str.value, targetAddr: multicast_addr.value + ":" + target_port.value });
}

async function disconn() {
  await invoke('close_udp_service');
}

listen('multicast-message', (event) => {
  console.log(event.payload);
  data.value = event.payload;
});

</script>

<template>
  <div class="card">
    <p>Udp BroadCaster</p>
    <p>{{ msg }}</p>
    <p>{{ data }}</p>
    <InputText v-model="ip" class="ml-2 mt-2 w-48" />
    <InputText v-model="port" class="ml-2 mt-2 w-48" />
    <Button @click="conn" class="ml-2 w-48">连接</Button>
    <Button @click="disconn" class="ml-2 w-48">关闭</Button>
    <br>
    <InputText v-model="str" class="ml-2 mt-2 " fluid />
    <br>
    <InputText v-model="multicast_addr" class="ml-2 mt-2 w-48" />
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