<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event'
import { useToast } from 'primevue/usetoast';

const toast = useToast();

let msg = ref("");
let data = ref("");
let str = ref("");
let ip = ref("127.0.0.1");
let port = ref(9999);

onMounted(() => {

});

const conns = ref([
  { name: 'All', key: '' },
]);

async function conn() {
  await invoke('tcp_client_connect', { address: ip.value+":"+port.value });
}

async function send() {
  await invoke('send_message', { message: str.value });
}

async function disconn() {
  await invoke('disconnect');
}

listen('client_msg', (event) => {
  msg.value = event.payload;
  toast.add({ severity: 'info', summary: 'Success', detail: msg.value, life: 3000 });
});

listen('client_data', (event) => {
  console.log(event.payload);
  data.value = event.payload;
});

</script>

<template>
  <div class="card">
    <p>Tcp Client</p>
    <p>{{ msg }}</p>
    <p>{{ data }}</p>
    <InputText v-model="ip" class="ml-2 mt-2 w-48" />
    <InputText v-model="port" class="ml-2 mt-2 w-48" />
    <Button @click="conn" class="ml-2 w-48">连接</Button>
    <Button @click="disconn" class="ml-2 w-48">关闭</Button>
    <br>
    <InputText v-model="str" class="ml-2 mt-2 " fluid />
    <br>
    <Button @click="send" class="ml-2 mt-2 " fluid>发送</Button>
  </div>

</template>

<style></style>