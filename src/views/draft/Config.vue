<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import InputText from 'primevue/inputtext';
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from '@tauri-apps/plugin-notification';

let permissionGranted;

let AppConfig = ref({
    username: String,
    password: String,
    ip: String,
    isdebug: Boolean,
    volume: Number,
    notifications: [],
    timeout: Number,
    alarms: []
});

onMounted(async () => {
  // 你有发送通知的权限吗？
  permissionGranted = await isPermissionGranted();

  // 如果没有，我们需要请求它
  if (!permissionGranted) {
    const permission = await requestPermission();
    permissionGranted = permission === 'granted';
  }
  get_config();
})

async function get_config() {
  let res = await invoke('get_config');
  console.log(res);
  AppConfig.value = res;
}

async function get_config_field(f) {
  let res = await invoke('get_config_field',{field:f});
  console.log(res);
  sendNotification(res);
}

async function set_config_field(f,v) {
    console.log(f,v);
  await invoke('set_config_field',{field:f,value:v});
  get_config();
}

async function set_config_field_int(f,v) {
    console.log(f,v);
  await invoke('set_config_field',{field:f,value:parseInt(v)});
  get_config();
}

async function set_config_field_bool(f,v) {
    console.log(f,v);
  await invoke('set_config_field',{field:f,value:v === "true"});
  get_config();
}

async function set_config_field_float(f,v) {
    console.log(f,v);
  await invoke('set_config_field',{field:f,value:parseFloat(v)});
  get_config();
}

async function set_config_field_srr(f,v) {
    console.log(f,v);
  await invoke('set_config_field',{field:f,value:v.split(",")});
  get_config();
}

async function reset_config() {
  let res = await invoke('reset_config');
  AppConfig.value = res;
  get_config();
}

async function set_alarm(){
    const obj = { index: 3, name: "低电量", enname: "Low Power", level:1 };
    const obj2 = { index: 4, name: "电机报错", enname: "Motor error", level:2 };
    const arr = [obj,obj2];
    await invoke('set_config_field',{field:"alarms",value:arr});
    get_config();
}
</script>

<template>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.username" class="w-72"/>
            <Button @click="get_config_field('username')" class="ml-4">获取用户名称</Button>
            <Button @click="set_config_field('username',AppConfig.username)" class="ml-4">设置用户名称</Button>
            <label for="on_label">用户名称</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.password" class="w-72"/>
            <Button @click="get_config_field('password')" class="ml-4">获取用户密码</Button>
            <Button @click="set_config_field('password',AppConfig.password)" class="ml-4">设置用户密码</Button>
            <label for="on_label">用户密码</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.ip" class="w-72"/>
            <Button @click="get_config_field('ip')" class="ml-4">获取ip</Button>
            <Button @click="set_config_field('ip',AppConfig.ip)" class="ml-4">设置ip</Button>
            <label for="on_label">ip</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.isdebug" class="w-72"/>
            <Button @click="get_config_field('isdebug')" class="ml-4">获取模式</Button>
            <Button @click="set_config_field_bool('isdebug',AppConfig.isdebug)" class="ml-4">设置模式</Button>
            <label for="on_label">是否为调试模式</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.volume" class="w-72"/>
            <Button @click="get_config_field('volume')" class="ml-4">获取音量</Button>
            <Button @click="set_config_field_float('volume',AppConfig.volume)" class="ml-4">设置音量</Button>
            <label for="on_label">音量</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.notifications" class="w-72"/>
            <Button @click="get_config_field('notifications')" class="ml-4">获取消息数组</Button>
            <Button @click="set_config_field_srr('notifications',AppConfig.notifications)" class="ml-4">设置消息数组</Button>
            <label for="on_label">消息数组</label>
        </FloatLabel>
        <FloatLabel variant="on" class="p-4" fluid>
            <InputText id="on_label" v-model="AppConfig.timeout" class="w-72"/>
            <Button @click="get_config_field('timeout')" class="ml-4">获取超时时间</Button>
            <Button @click="set_config_field_int('timeout',AppConfig.timeout)" class="ml-4">设置超时时间</Button>
            <label for="on_label">超时时间</label>
        </FloatLabel>

  <Button @click="get_config" class="ml-4">获取当前配置</Button>
  <Button @click="reset_config" class="ml-2">恢复默认配置</Button>
  <Button @click="set_alarm" class="ml-2">设置报警列表</Button>
</template>

<style>
</style>