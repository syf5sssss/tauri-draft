<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import Toolbar from 'primevue/toolbar';
import IconField from 'primevue/iconfield';
import InputText from 'primevue/inputtext';
import FloatLabel from 'primevue/floatlabel';
import Panel from 'primevue/panel';
import { toNaiveDateTime } from '@/util/dateutil';

const mapmsg = ref();
const formattedMapMsg = ref();

const teacher = ref({
  id: null,
  name: null,
  age: null,
  height: null,
  birthday: null,
  name_filter: null,
  age_min: null,
  age_max: null,
  birthday_start: null,
  birthday_end: null
});

function clearteacher() {
  teacher.value = {
    id: null,
    name: null,
    age: null,
    height: null,
    birthday: null,
    name_filter: null,
    age_min: null,
    age_max: null,
    birthday_start: null,
    birthday_end: null
  }
}

onMounted(async () => {
  initteachers();
  selteacher();
})


async function initteachers() {
  await invoke('init_teachers');
}
async function getteacher() {
  let res = await invoke('get_teacher', { id: parseInt(teacher.value.id) });
  formattedMapMsg.value = JSON.stringify(res, null, 2);
  mapmsg.value = "查询老师获得结果:" + formattedMapMsg.value;
}
async function addteacher() {
  await invoke('add_teacher', { name: teacher.value.name, age: parseInt(teacher.value.age), height: parseFloat(teacher.value.height), birthday: toNaiveDateTime(teacher.value.birthday) });
  mapmsg.value = "添加老师完成 teacher:" + JSON.stringify(teacher.value, null, 2);
  selteacher();
}
async function delteacher() {
  await invoke('delete_teacher', { id: parseInt(teacher.value.id) });
  mapmsg.value = "删除老师完成 id:" + teacher.value.id;
  selteacher();
}
async function updateteacher() {
  await invoke('update_teacher', { id: parseInt(teacher.value.id), name: teacher.value.name, age: parseInt(teacher.value.age), height: parseFloat(teacher.value.height), birthday: toNaiveDateTime(teacher.value.birthday) });
  mapmsg.value = "修改老师完成 id:" + teacher.value.id;
  selteacher();
}
async function selteacher() {
  let res = await invoke('query_teachers', { nameFilter: teacher.value.name_filter, ageMin: parseInt(teacher.value.age_min), ageMax: parseInt(teacher.value.age_max), birthdayStart: toNaiveDateTime(teacher.value.birthday_start), birthdayEnd: toNaiveDateTime(teacher.value.birthday_end) });
  formattedMapMsg.value = JSON.stringify(res, null, 2);
  mapmsg.value = "查询老师获得结果:" + res.length ? res.length : formattedMapMsg.value;
}
</script>

<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-plus" class="mr-2" label="增加" @click="addteacher()" />
      <Button icon="pi pi-trash" class="mr-2" label="删除" @click="delteacher" />
      <Button icon="pi pi-file-edit" class="mr-2" label="修改" @click="updateteacher" />
      <Button icon="pi pi-eye" class="mr-2" label="查询" @click="selteacher()" />
      <Button icon="pi pi-eye" class="mr-2" label="查询id" @click="getteacher()" />
      <Button icon="pi pi-eye" class="mr-2" label="清空" @click="clearteacher()" />
    </template>
  </Toolbar>
  <Panel header="条件筛选" toggleable :collapsed="true" class="mt-2">
    <div class="flex flex-wrap gap-2 items-center justify-between">
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.id" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">ID</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.name" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">名称</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.age" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.height" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">身高</label>
      </FloatLabel>
      <DatePicker v-model="teacher.birthday" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
    </div>
    <div class="flex flex-wrap gap-2 items-center justify-between">
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.name_filter" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">名称</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.age_min" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄大于</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="teacher.age_max" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄小于</label>
      </FloatLabel>
      <DatePicker v-model="teacher.birthday_start" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
      <DatePicker v-model="teacher.birthday_end" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
    </div>
  </Panel>
  <Message class="mt-2" severity="success">
    {{ mapmsg }}
  </Message>
  <Message class="mt-2" severity="success">
    {{ teacher }}
  </Message>
  <div>
    <pre>{{ formattedMapMsg }}</pre>
  </div>
</template>

<style scoped>
pre {
  white-space: pre-wrap;
  /* 允许自动换行 */
  word-break: break-all;
  /* 允许在单词内换行 */
  padding: 15px;
  background-color: #f8f8f8;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  /* 使用等宽字体 */
  max-height: 500px;
  /* 设置最大高度 */
  overflow: auto;
  /* 添加滚动条 */
  line-height: 1.5;
  /* 增加行高 */
}
</style>