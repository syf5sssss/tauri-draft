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
const selectedClient = ref('All');

onMounted(() => {

});

const conns = ref([
  { name: 'All', key: '' },
]);

async function conn() {
  await invoke('start_tcp_server', { ip: ip.value, port: port.value });
}

async function send_client() {
  if (selectedClient.value === "All") {
    await invoke('send_to_clients', { message: str.value });
  } else {
    await invoke('send_to_client', { clientAddr: selectedClient.value, message: str.value });
  }
}

async function disconn() {
  await invoke('stop_tcp_server');
}

async function flush() {
  console.log("刷新");
  let cos = await invoke('get_connstr');
  console.log(cos);
  conns.value = [{ name: 'All', key: '' }];
  if(cos && cos.length>0){
    let srr = cos.split(",");
    for(let i=0;i<srr.length;i++){
      conns.value.push({ name: srr[i], value: srr[i] });
    }
  }
}


listen('server_msg', (event) => {
  msg.value = event.payload;
  toast.add({ severity: 'info', summary: 'Success', detail: msg.value, life: 3000 });
});

listen('server_data', (event) => {
  console.log(event.payload);
  data.value = event.payload;
});

listen('conn_add', (event) => {
  console.log('add --  ', event.payload);
  conns.value.push({ name: event.payload, value: event.payload });
  toast.add({ severity: 'info', summary: 'Success', detail: "接入：" + event.payload, life: 3000 });
});

listen('conn_del', (event) => {
  console.log('del --  ', event.payload);
  conns.value = conns.value.filter(conn => conn.name !== event.payload);
  toast.add({ severity: 'info', summary: 'Success', detail: "掉线：" + event.payload, life: 3000 });
});
</script>

<template>
  <div class="card">
    <p>Tcp Server</p>
    <p>{{ msg }}</p>
    <p>{{ data }}</p>
    <InputText v-model="ip" class="ml-2 mt-2 w-48" />
    <InputText v-model="port" class="ml-2 mt-2 w-48" />
    <Button @click="conn" class="ml-2 w-48">连接</Button>
    <Button @click="disconn" class="ml-2 w-48">关闭</Button>
    <br>
    <div class="ml-2 mt-6 flex flex-col gap-2">
      <div class="flex flex-wrap gap-4">
        <div class="flex items-center gap-2">
          <Button @click="flush" class="ml-2 w-12" icon="pi pi-refresh" size="small"></Button>
        </div>
        <div v-for="c in conns" :key="c.key" class="flex items-center gap-2">
          <RadioButton v-model="selectedClient" :inputId="c.key" name="dynamic" :value="c.name" />
          <label :for="c.key">{{ c.name }}</label>
        </div>
      </div>
    </div>
    <br>
    <InputText v-model="str" class="ml-2 mt-2 " fluid />
    <br>
    <Button @click="send_client" class="ml-2 mt-2 " fluid>发送</Button>
  </div>

</template>

<style></style>