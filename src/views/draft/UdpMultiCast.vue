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
    <p>Udp Multicast</p>
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




        整个 IPv4 组播地址空间是 224.0.0.0 到 239.255.255.255,但被划分为多个特定用途的子范围：

        地址范围 名称 是否可路由 用途描述
        224.0.0.0 - 224.0.0.255 本地网络控制 ❌ 不可路由 网络协议通信(如OSPF,RIP)
        224.0.1.0 - 224.0.1.255 互联网控制 ✅ 可路由 全局网络服务(如NTP)
        224.0.2.0 - 238.255.255.255 全局范围 ✅ 可路由 公共组播应用
        232.0.0.0 - 232.255.255.255 源特定组播(SSM) ✅ 可路由 指定源的组播
        233.0.0.0 - 233.255.255.255 GLOP地址 ✅ 可路由 按AS号分配的地址
        239.0.0.0 - 239.255.255.255 管理范围 ❌ 不可路由 组织内部私有组播
      </code>
    </pre>
  </div>

</template>

<style></style>