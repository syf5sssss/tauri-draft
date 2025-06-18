<script setup>
import { ref, onMounted, onBeforeUnmount } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event'
import { useToast } from 'primevue/usetoast';
import ProgressBar from 'primevue/progressbar';

const toast = useToast();

let msg = ref("");
let msg2 = ref("");
let msg3 = ref("");
let value1 = ref("");
const value2 = ref(0);
const interval = ref();

onMounted(() => {
    
});

onBeforeUnmount(() => {
    endProgress();
});


const startProgress = () => {
    interval.value = setInterval(() => {
      if(value2.value<100){
        let newValue = value2.value + Math.floor(Math.random() * 10) + 1;
        if (newValue >= 100) {
          value2.value = 100;
          toast.add({ severity: 'info', summary: 'Success', detail: 'Process Completed', life: 1000 });
        }else{
          value2.value = newValue;
        }
      }
    }, 500);
};

const endProgress = () => {
    clearInterval(interval.value);
    interval.value = null;
};

async function start_time() {
  await invoke('start_time');
}

function openprogress() {
  msg3.value = "准备中。。。。";
  startProgress();
}

async function stop_time() {
  await invoke('stop_time');
}

async function stop_timer() {
  await invoke('stop_timer');
}

async function start_timer() {
  await invoke('start_timer',{interval:1000});
}

async function set_shared_value() {
  await invoke('set_shared_value',{value:value1.value});
}

async function get_shared_value() {
  let res = await invoke('get_shared_value');
  toast.add({ severity: 'info', summary: 'Info', detail: res, life: 3000 });
}

async function get_last_update() {
  let res = await invoke('get_last_update');
  toast.add({ severity: 'info', summary: 'Info', detail: res, life: 3000 });
}

async function workdownprogress() {
  await invoke('progress_update');
}

listen('update_time', (event) => {
  msg.value = event.payload;
});

listen('value_updated', (event) => {
  msg2.value = event.payload;
});

listen('progress_update', (event) => {
  value2.value = event.payload;
  if(value2.value === 100){
    value2.value = 0;
    msg3.value = "";
    toast.add({ severity: 'info', summary: 'Success', detail: '全部完成', life: 1000 });
  }
});
listen('progress_msg', (event) => {
  msg3.value = event.payload;
});
</script>

<template>
  <p>{{msg}}</p>
  <p>{{msg2}}</p>

  <Button @click="start_time" class="ml-2 w-48">开始更新时间</Button>
  <Button @click="stop_time" class="ml-2 w-48">停止更新时间</Button>
  <br>
  <Button @click="start_timer" class="ml-2 mt-2 w-48">静态变量开启监听</Button>
  <Button @click="stop_timer" class="ml-2 mt-2 w-48">静态变量关闭监听</Button>
  <br>
  <InputText v-model="value1" class="ml-2 mt-2 w-48"/>
  <Button @click="set_shared_value" class="ml-2 mt-2 w-48">设置静态变量</Button>
  <br>
  <Button @click="get_shared_value" class="ml-2 mt-2 w-48">获取静态变量值</Button>
  <Button @click="get_last_update" class="ml-2 mt-2 w-48">获取静态变量时间</Button>

  <br>
  <Button @click="openprogress" class="ml-2 mt-2 w-48">进度条</Button>
  <Button @click="workdownprogress" class="ml-2 mt-2 w-48">下班进度条</Button>

  <br>
  <div class="card">
    <ProgressBar :value="value2" />
    {{msg3}}
  </div>
</template>

<style>
</style>