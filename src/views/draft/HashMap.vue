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

const student = ref({
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

function clearstudent() {
  student.value = {
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
  initstudents();
  selstudent();
})


async function initstudents() {
  await invoke('init_students');
}
async function getstudent() {
  let res = await invoke('get_student', { id: parseInt(student.value.id) });
  formattedMapMsg.value = JSON.stringify(res, null, 2);
  mapmsg.value = "查询学生获得结果:" + formattedMapMsg.value;
}
async function addstudent() {
  await invoke('add_student', { name: student.value.name, age: parseInt(student.value.age), height: parseFloat(student.value.height), birthday: toNaiveDateTime(student.value.birthday) });
  mapmsg.value = "添加学生完成 student:" + JSON.stringify(student.value, null, 2);
  selstudent();
}
async function delstudent() {
  await invoke('delete_student', { id: parseInt(student.value.id) });
  mapmsg.value = "删除学生完成 id:" + student.value.id;
  selstudent();
}
async function updatestudent() {
  await invoke('update_student', { id: parseInt(student.value.id), name: student.value.name, age: parseInt(student.value.age), height: parseFloat(student.value.height), birthday: toNaiveDateTime(student.value.birthday) });
  mapmsg.value = "修改学生完成 id:" + student.value.id;
  selstudent();
}
async function selstudent() {
  let res = await invoke('query_students', { nameFilter: student.value.name_filter, ageMin: parseInt(student.value.age_min), ageMax: parseInt(student.value.age_max), birthdayStart: toNaiveDateTime(student.value.birthday_start), birthdayEnd: toNaiveDateTime(student.value.birthday_end) });
  formattedMapMsg.value = JSON.stringify(res, null, 2);
  mapmsg.value = "查询学生获得结果:" + res.length ? res.length : formattedMapMsg.value;
}
</script>

<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-plus" class="mr-2" label="增加" @click="addstudent()" />
      <Button icon="pi pi-trash" class="mr-2" label="删除" @click="delstudent" />
      <Button icon="pi pi-file-edit" class="mr-2" label="修改" @click="updatestudent" />
      <Button icon="pi pi-eye" class="mr-2" label="查询" @click="selstudent()" />
      <Button icon="pi pi-eye" class="mr-2" label="查询id" @click="getstudent()" />
      <Button icon="pi pi-eye" class="mr-2" label="清空" @click="clearstudent()" />
    </template>
  </Toolbar>
  <Panel header="条件筛选" toggleable :collapsed="true" class="mt-2">
    <div class="flex flex-wrap gap-2 items-center justify-between">
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.id" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">ID</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.name" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">名称</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.age" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.height" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">身高</label>
      </FloatLabel>
      <DatePicker v-model="student.birthday" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
    </div>
    <div class="flex flex-wrap gap-2 items-center justify-between">
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.name_filter" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">名称</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.age_min" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄大于</label>
      </FloatLabel>
      <FloatLabel variant="on">
        <IconField>
          <InputText id="on_label" v-model="student.age_max" autocomplete="off" class="w-48" />
        </IconField>
        <label for="on_label">年龄小于</label>
      </FloatLabel>
      <DatePicker v-model="student.birthday_start" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
      <DatePicker v-model="student.birthday_end" showTime hourFormat="24" dateFormat="yy-mm-dd" class="w-48" />
    </div>
  </Panel>
  <Message class="mt-2" severity="success">
    {{ mapmsg }}
  </Message>
  <Message class="mt-2" severity="success">
    {{ student }}
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