<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { basename, dirname, extname } from '@tauri-apps/api/path';
import { ask, confirm, message, open, save } from '@tauri-apps/plugin-dialog';
import TreeTable from 'primevue/treetable';
import Column from 'primevue/column';
import Checkbox from 'primevue/checkbox';
import Toolbar from 'primevue/toolbar';
import Dialog from 'primevue/dialog';
import { v4 as uuidv4 } from 'uuid';

const adddir_visible = ref(false);//创建目录
const updatename_visible = ref(false);//批量重命名弹出框
const nodes = ref();
const selectedKey = ref({});
const loading = ref(false);
const columns = ref({ name: true, path: true, formatted_size: true, entry_type: false, file_type: false, created: true, modified: false, accessed: false, is_empty: false });
const selectedPaths = ref([]);
const scanpath = ref("");//当前扫描的地址
const pathname = ref("");//创建目录的名称
const filename = ref("");//重命名的名称
const filenameone = ref("");//重命名的名称
const filenameonemap = ref({});//重命名的名称
const filenametype = ref("prefix");//重命名的名称
const selectedRow = ref();//选中的行数据
const nulldir_visible = ref(false);//空文件夹弹出框
const eqfile_visible = ref(false);//相似文件弹出框
const updatename_one_visible = ref(false);//重命名弹出框
const selectedNullDir = ref();//空文件夹选中项
const selectedEqFiles = ref();//相似文件选中项
const nulldirs = ref();//空文件夹列表数据
const eqfiles = ref();//相似文件列表数据



/**
 * 格式化文件大小为易读格式（增强版）
 * @param {number} bytes - 文件大小（字节）
 * @param {boolean} [useBinary=false] - 是否使用二进制单位 (KiB, MiB)
 * @returns {string} 格式化后的字符串
 */
function formatFileSize(bytes, useBinary = false) {
  if (isNaN(bytes) || bytes < 0) return '无效大小';
  if (bytes === 0) return '0 Bytes';

  const unit = useBinary ? 1024 : 1000;
  const units = useBinary
    ? ['Bytes', 'KiB', 'MiB', 'GiB', 'TiB']
    : ['Bytes', 'KB', 'MB', 'GB', 'TB'];

  const exponent = Math.floor(Math.log(bytes) / Math.log(unit));
  const value = bytes / Math.pow(unit, exponent);

  let decimals;
  if (value < 10) decimals = 2;
  else if (value < 100) decimals = 1;
  else decimals = 0;

  return value.toFixed(decimals) + ' ' + units[exponent];
}


// 计算选中的文件数量
const fileTypeCount = computed(() => {
  if (!nodes.value || !nodes.value.tree || !selectedKey.value) return 0;

  // 检查节点是否被选中
  function isNodeSelected(node) {
    return selectedKey.value.hasOwnProperty(node.key);
  }

  // 检查节点是否为文件类型
  function isNodeFile(node) {
    return node.data.entry_type === 'File';
  }

  function countFilesInTree(tree) {
    return tree.reduce((count, node) => {
      const currentCount = isNodeSelected(node) && isNodeFile(node) ? 1 : 0;
      const childrenCount = node.children ? countFilesInTree(node.children) : 0;
      return count + currentCount + childrenCount;
    }, 0);
  }

  return countFilesInTree(nodes.value.tree);
});

//打开文件所在目录
async function open_entry_location(path) {
  await invoke('open_entry_location', { path: path });
}

async function delete_entries() {
  try {
    // 重置选中的路径
    selectedPaths.value = [];

    // 检查是否有有效数据
    if (!nodes.value || !nodes.value.tree || !selectedKey.value || Object.keys(selectedKey.value).length === 0) {
      await message('请先选择要删除的项目', { title: '提示', kind: 'info' });
      return;
    }

    // 递归收集选中的路径
    function collectSelectedPaths(nodes) {
      for (const node of nodes) {
        // 检查当前节点是否被选中
        const selection = selectedKey.value[node.key];
        if (selection && selection.checked) {
          selectedPaths.value.push(node.data.path);
        }

        // 递归处理子节点
        if (node.children && node.children.length > 0) {
          collectSelectedPaths(node.children);
        }
      }
    }

    // 从根节点开始收集
    collectSelectedPaths(nodes.value.tree);

    // 如果没有选中任何项目
    if (selectedPaths.value.length === 0) {
      await message('没有选中任何项目', { title: '提示', kind: 'info' });
      return;
    }

    // 确认删除操作
    const confirmed = await confirm(
      `确定要永久删除 ${selectedPaths.value.length} 个文件/目录吗？此操作不可撤销！`,
      {
        title: '确认删除',
        kind: 'warning',
        okLabel: '永久删除',
        cancelLabel: '取消'
      }
    );

    if (confirmed) {
      // 调用后端删除API
      await invoke('delete_entries', { paths: selectedPaths.value });
      get_directory_info_reload();
      // 清空选择
      selectedKey.value = {};
    }
  } catch (error) {
    console.error('删除操作出错:', error);
    await message(`删除失败: ${error}`, {
      title: '错误',
      kind: 'error',
      duration: 5000
    });
  }
}


async function delete_entries_one(path) {
  try {
    // 重置选中的路径
    selectedPaths.value = [];

    // 检查是否有有效数据
    if (!nodes.value || !nodes.value.tree || !selectedKey.value || Object.keys(selectedKey.value).length === 0 || path.length === 0) {
      await message('请先选择要删除的项目', { title: '提示', kind: 'info' });
      return;
    }
    selectedPaths.value.push(path);

    // 确认删除操作
    const confirmed = await confirm(
      `确定要永久删除 ${selectedPaths.value.length} 个文件/目录吗？此操作不可撤销！`,
      {
        title: '确认删除',
        kind: 'warning',
        okLabel: '永久删除',
        cancelLabel: '取消'
      }
    );

    if (confirmed) {
      // 调用后端删除API
      await invoke('delete_entries', { paths: selectedPaths.value });
      get_directory_info_reload();
      // 清空选择
      selectedKey.value = {};
    }
  } catch (error) {
    console.error('删除操作出错:', error);
    await message(`删除失败: ${error}`, {
      title: '错误',
      kind: 'error',
      duration: 5000
    });
  }
}

//扫描目录
async function get_directory_info() {
  const file = await open({
    multiple: false,
    directory: true,
  });
  scanpath.value = file;
  loading.value = true;
  let res = await invoke('get_directory_info', { path: file, recursive: true });
  nodes.value = res;
  loading.value = false;
}

async function get_directory_info_reload() {
  if (scanpath.value.length === 0) {
    console.log("还没有选中扫描地址");
    return;
  }
  loading.value = true;
  let res = await invoke('get_directory_info', { path: scanpath.value, recursive: true });
  nodes.value = res;
  loading.value = false;
}

function create_dir(node) {
  if (node && node.path && node.path.length > 0) {
    selectedRow.value = node;
    adddir_visible.value = true;
  }
}

function create_base_dir() {
  if (scanpath.value && scanpath.value.length > 0) {
    adddir_visible.value = true;
  }
}


function cancelname() {
  adddir_visible.value = false;
}

async function confirmname() {
  if (pathname.value && pathname.value.length > 0 && selectedRow.value && selectedRow.value.path && selectedRow.value.path.length > 0) {
    console.log(selectedRow.value.path + "\\" + pathname.value);
    await invoke('create_entry', { path: selectedRow.value.path + "\\" + pathname.value, isDirectory: true });
  }
  if (pathname.value && pathname.value.length > 0 && scanpath.value && scanpath.value.length > 0) {
    console.log(scanpath.value + "\\" + pathname.value);
    await invoke('create_entry', { path: scanpath.value + "\\" + pathname.value, isDirectory: true });
  }
  pathname.value = "";
  adddir_visible.value = false;
  get_directory_info_reload();
}

function null_dir() {
  // 重置选中的路径
  nulldirs.value = [];

  // 检查是否有有效数据
  if (!nodes.value || !nodes.value.tree) {
    message("没有可检测的内容", {
      title: '错误',
      kind: 'info',
      duration: 5000
    });
    return;
  }

  // 递归收集选中的路径
  function collectSelectedNullDir(nodes) {
    for (const node of nodes) {
      if (node.data.is_empty && node.data.entry_type === 'Directory') {
        nulldirs.value.push(node.data);
      }

      // 递归处理子节点
      if (node.children && node.children.length > 0) {
        collectSelectedNullDir(node.children);
      }
    }
  }

  // 从根节点开始收集
  collectSelectedNullDir(nodes.value.tree);

  // 如果没有选中任何项目
  if (nulldirs.value.length === 0) {
    console.log("没有找到空目录");
    message("没有找到空目录", {
      title: '错误',
      kind: 'info',
      duration: 5000
    });
    return;
  }

  nulldir_visible.value = true;
}

function cancelnull() {
  nulldir_visible.value = false;
}

async function confirmnull() {
  if (selectedNullDir.value.length && selectedNullDir.value.length > 0) {
    const pathArray = selectedNullDir.value.map(item => item.path);
    // 确认删除操作
    const confirmed = await confirm(
      `确定要永久删除 ${selectedNullDir.value.length} 个文件/目录吗？此操作不可撤销！`,
      {
        title: '确认删除',
        kind: 'warning',
        okLabel: '永久删除',
        cancelLabel: '取消'
      }
    );

    if (confirmed) {
      // 调用后端删除API
      await invoke('delete_entries', { paths: pathArray });
      get_directory_info_reload();
    }

    nulldir_visible.value = false;
  }
}

function updatename_dialog() {
  updatename_visible.value = true;
}

//批量重命名文件
async function update_filenames(name, type) {
  try {
    // 重置选中的路径
    selectedPaths.value = [];

    // 检查是否有有效数据
    if (!nodes.value || !nodes.value.tree || !selectedKey.value || Object.keys(selectedKey.value).length === 0) {
      await message('请先选择要重命名的项目', { title: '提示', kind: 'info' });
      return;
    }

    // 递归收集选中的路径
    function collectSelectedPaths(nodes) {
      for (const node of nodes) {
        // 检查当前节点是否被选中
        const selection = selectedKey.value[node.key];
        if (selection && selection.checked && node.data.entry_type === 'File') {
          selectedPaths.value.push(node.data.path);
        }

        // 递归处理子节点
        if (node.children && node.children.length > 0) {
          collectSelectedPaths(node.children);
        }
      }
    }

    // 从根节点开始收集
    collectSelectedPaths(nodes.value.tree);

    // 如果没有选中任何项目
    if (selectedPaths.value.length === 0) {
      await message('没有选中任何项目', { title: '提示', kind: 'info' });
      return;
    }

    //将选中的文件批量重命名
    let map = generateRenameMap(selectedPaths.value, name, type);
    // 确认删除操作
    const confirmed = await confirm(
      `确定要重命名 ${selectedPaths.value.length} 个文件吗？此操作不可撤销！`,
      {
        title: '确认重命名',
        kind: 'warning',
        okLabel: '永久重命名',
        cancelLabel: '取消'
      }
    );

    if (confirmed) {
      // 调用后端删除API
      await invoke('rename_entries', { renameMap: map });
      get_directory_info_reload();
      // 清空选择
      filename.value = "";
      selectedKey.value = {};
    }
  } catch (error) {
    console.error('重命名操作出错:', error);
    await message(`重命名失败: ${error}`, {
      title: '错误',
      kind: 'error',
      duration: 5000
    });
  }
}

// 生成重命名映射
const generateRenameMap = (filePaths, baseName, type) => {
  const map = {};
  filePaths.forEach((path, index) => {
    const dir = path.substring(0, path.lastIndexOf('\\')) + '\\';
    const ext = path.substring(path.lastIndexOf('.'));
    if (type === 'prefix') {
      map[path] = `${dir}${baseName}_${index + 1}${ext}`;
    }
    if (type === 'uuid') {
      map[path] = `${dir}${uuidv4()}${ext}`;
    }

  });
  return map;
};

function cancelrename() {
  updatename_visible.value = false;
}

async function confirmrename() {
  update_filenames(filename.value, filenametype.value);
  updatename_visible.value = false;
}

async function renameone(path) {
  filenameone.value = await basename(path);
  filenameonemap.value[path] = path;
  updatename_one_visible.value = true;
}

function cancelonerename() {
  updatename_one_visible.value = false;
}

async function confirmonerename() {
  const firstValue = Object.keys(filenameonemap.value)[0];
  const dir = await dirname(firstValue);
  const newpath = `${dir}\\${filenameone.value}`;
  filenameonemap.value[Object.keys(filenameonemap.value)[0]] = newpath;
  await invoke('rename_entries', { renameMap: filenameonemap.value });
  get_directory_info_reload();
  filenameone.value = "";
  filenameonemap.value = {};
  updatename_one_visible.value = false;
}

async function move() {
  const dirPath = await open({
    title: '移动到目录',
    directory: true, // 选择目录
    multiple: false // 单选
  });

  if (dirPath && dirPath.length > 0) {
    console.log('选择的目录路径:', dirPath);
    // 重置选中的路径
    selectedPaths.value = [];

    // 检查是否有有效数据
    if (!nodes.value || !nodes.value.tree || !selectedKey.value || Object.keys(selectedKey.value).length === 0) {
      await message('请先选择要删除的内容', { title: '提示', kind: 'info' });
      return;
    }

    // 递归收集选中的路径
    function collectSelectedPaths(nodes) {
      for (const node of nodes) {
        // 检查当前节点是否被选中
        const selection = selectedKey.value[node.key];
        if (selection && selection.checked && node.data.entry_type === 'File') {
          selectedPaths.value.push(node.data.path);
        }

        // 递归处理子节点
        if (node.children && node.children.length > 0) {
          collectSelectedPaths(node.children);
        }
      }
    }

    // 从根节点开始收集
    collectSelectedPaths(nodes.value.tree);

    // 如果没有选中任何项目
    if (selectedPaths.value.length === 0) {
      await message('没有选中任何内容', { title: '提示', kind: 'info' });
      return;
    }

    // 调用后端删除API
    await invoke('move_entries', { sources: selectedPaths.value, targetDir: dirPath, overwrite: true });
    get_directory_info_reload();
    // 清空选择
    selectedKey.value = {};
  }
}


function openeqfiles(type) {
  // 重置选中的路径
  eqfiles.value = [];

  // 检查是否有有效数据
  if (!nodes.value || !nodes.value.tree) {
    message("没有可检测的内容", {
      title: '错误',
      kind: 'info',
      duration: 5000
    });
    return;
  }

  // 递归收集选中的路径
  function collectSelectedEqFiles(nodes) {
    for (const node of nodes) {
      if (node.data && node.data.entry_type === 'File') {
        eqfiles.value.push(node.data);
      }

      // 递归处理子节点
      if (node.children && node.children.length > 0) {
        collectSelectedEqFiles(node.children);
      }
    }
  }

  // 从根节点开始收集
  collectSelectedEqFiles(nodes.value.tree);

  console.log(eqfiles.value);
  const fmap = new Map();
  //名称
  if (type === 1) {
    eqfiles.value.forEach(file => {
      if (!fmap.has(file.name)) {
        fmap.set(file.name, []);
      }
      fmap.get(file.name).push(file);
    });
  }
  //大小
  if (type === 2) {
    // 构建文件大小到文件数组的映射
    eqfiles.value.forEach(file => {
      if (!fmap.has(file.size)) {
        fmap.set(file.size, []);
      }
      fmap.get(file.size).push(file);
    });
  }
  eqfiles.value = Array.from(fmap.values())
    .filter(group => group.length > 1)
    .flat();
  console.log(eqfiles.value);
  // 如果没有选中任何项目
  if (eqfiles.value.length === 0) {
    console.log("没有找到相似文件");
    message("没有找到相似文件", {
      title: '错误',
      kind: 'info',
      duration: 5000
    });
    return;
  }

  eqfile_visible.value = true;
}

function canceleqfile() {
  eqfile_visible.value = false;
}

async function confirmeqfile() {
  if (selectedEqFiles.value.length && selectedEqFiles.value.length > 0) {
    const pathArray = selectedEqFiles.value.map(item => item.path);
    // 确认删除操作
    const confirmed = await confirm(
      `确定要永久删除 ${selectedEqFiles.value.length} 个文件吗？此操作不可撤销！`,
      {
        title: '确认删除',
        kind: 'warning',
        okLabel: '永久删除',
        cancelLabel: '取消'
      }
    );

    if (confirmed) {
      // 调用后端删除API
      await invoke('delete_entries', { paths: pathArray });
      get_directory_info_reload();
    }

    eqfile_visible.value = false;
  }
}


</script>

<template>
  <Toolbar>
    <template #start>
      <Button icon="pi pi-folder-open" class="mr-2" @click="get_directory_info" label="扫描" />
      <Button v-if="nodes" icon="pi pi-reply" class="mr-2" label="移动到" @click="move" />
      <Button v-if="nodes" icon="pi pi-plus" class="mr-2" label="增加" @click="create_base_dir()" />
      <Button v-if="nodes" icon="pi pi-trash" class="mr-2" label="删除" @click="delete_entries" />
      <Button v-if="nodes" icon="pi pi-file-edit" class="mr-2" label="重命名" @click="updatename_dialog" />
      <Button v-if="nodes" icon="pi pi-folder" class="mr-2" label="空目录" @click="null_dir" />
      <Button v-if="nodes" icon="pi pi-eye" class="mr-2" label="按名称相似的文件" @click="openeqfiles(1)" />
      <Button v-if="nodes" icon="pi pi-eye" class="mr-2" label="按大小相似的文件" @click="openeqfiles(2)" />
    </template>
  </Toolbar>
  <Message v-if="nodes" class="mt-2" severity="success">
    <div class="text-l font-bold" v-if="nodes">存储大小：{{ formatFileSize(nodes.total_size) }}
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;目录数：{{ nodes.total_dirs }}
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;文件数：{{ nodes.total_files }}
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;选中文件数：{{ fileTypeCount }}
      &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; 路径：{{ scanpath }}</div>
  </Message>
  <Message v-if="nodes" class="mt-2" severity="success">
    <div class=" flex flex-wrap justify-between gap-4 ">
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.name" binary />
        <label> 名称 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.path" binary />
        <label> 路径 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.formatted_size" binary />
        <label> 大小 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.entry_type" binary />
        <label> 枚举类型 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.file_type" binary />
        <label> 文件类型 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.created" binary />
        <label> 创建时间 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.modified" binary />
        <label> 修改时间 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.accessed" binary />
        <label> 访问时间 </label>
      </div>
      <div class="flex items-center gap-2">
        <Checkbox v-model="columns.is_empty" binary />
        <label> 是否为空目录 </label>
      </div>
    </div>
  </Message>
  <div class="card mt-2" v-if="nodes">
    <TreeTable :value="nodes.tree" scrollable :loading="loading" tableStyle="min-width: 50rem" size="small"
      selectionMode="checkbox" v-model:selectionKeys="selectedKey" showGridlines class="mt-2">
      <Column v-if="columns.name" field="name" header="名称" expander style="min-width: 250px">
        <template #body="slotProps">
          <div @dblclick="open_entry_location(slotProps.node.data.path)" style="cursor: pointer; padding: 0.5rem;"
            class="hover:bg-gray-100">
            {{ slotProps.node.data.name }}
            <span v-if="slotProps.node.children" style="margin-left: 0.5rem; color: #888;">
              ({{ slotProps.node.children.length }})
            </span>
          </div>
        </template>
      </Column>
      <Column v-if="columns.path" field="path" header="路径" style="min-width: 200px"></Column>
      <Column v-if="columns.formatted_size" field="formatted_size" header="大小" style="min-width: 80px"></Column>
      <Column v-if="columns.entry_type" field="entry_type" header="枚举类型" style="min-width: 80px"></Column>
      <Column v-if="columns.file_type" field="file_type" header="文件类型" style="min-width: 80px"></Column>
      <Column v-if="columns.created" field="created" header="创建时间" style="min-width: 150px"></Column>
      <Column v-if="columns.modified" field="modified" header="修改时间" style="min-width: 150px"></Column>
      <Column v-if="columns.accessed" field="accessed" header="访问时间" style="min-width: 150px"></Column>
      <Column v-if="columns.is_empty" field="is_empty" header="是否为空目录" style="min-width: 80px"></Column>
      <Column style="min-width: 190px" header="操作">
        <template #body="slotProps">
          <div class="flex flex-wrap gap-2">
            <Button type="button" icon="pi pi-search" rounded severity="success"
              @click="open_entry_location(slotProps.node.data.path)" size="small" />
            <Button type="button" icon="pi pi-file-edit" rounded severity="success" size="small"
              @click="renameone(slotProps.node.data.path)" />
            <Button type="button" icon="pi pi-trash" rounded severity="success" size="small"
              @click="delete_entries_one(slotProps.node.data.path)" />
            <Button type="button" icon="pi pi-reply" rounded severity="success" size="small" />
            <Button v-if="slotProps.node.data.entry_type === 'Directory'" type="button" icon="pi pi-plus" rounded
              severity="success" size="small" @click="create_dir(slotProps.node.data)" />
          </div>
        </template>
      </Column>
    </TreeTable>
  </div>

  <Dialog v-model:visible="adddir_visible" modal header="创建目录" :style="{ width: '25rem' }">
    <div class="flex items-center gap-4 mb-4">
      <label class="font-semibold w-24">目录名称</label>
      <InputText class="flex-auto" autocomplete="off" v-model="pathname" />
    </div>
    <div class="flex justify-end gap-2">
      <Button type="button" label="取消" severity="secondary" @click="cancelname"></Button>
      <Button type="button" label="确认" @click="confirmname"></Button>
    </div>
  </Dialog>

  <Dialog v-model:visible="updatename_one_visible" modal header="重命名">
    <div class="flex items-center gap-4 mb-4">
      <InputText class="flex-auto" autocomplete="off" v-model="filenameone" />
    </div>
    <div class="flex justify-end gap-2">
      <Button type="button" label="取消" severity="secondary" @click="cancelonerename"></Button>
      <Button type="button" label="确认" @click="confirmonerename"></Button>
    </div>
  </Dialog>

  <Dialog v-model:visible="updatename_visible" modal header="批量重命名" :style="{ width: '25rem' }">
    <div class="flex flex-wrap gap-4">
      <div class="flex items-center gap-2">
        <RadioButton v-model="filenametype" inputId="ingredient1" name="rename" value="prefix" />
        <label for="ingredient1">前缀</label>
      </div>
      <div class="flex items-center gap-2">
        <RadioButton v-model="filenametype" inputId="ingredient2" name="rename" value="uuid" />
        <label for="ingredient2">UUID</label>
      </div>
    </div>
    <div class="flex items-center gap-4 mb-4" v-if="filenametype === 'prefix'">
      <label class="font-semibold w-24">名称前缀</label>
      <InputText class="flex-auto" autocomplete="off" v-model="filename" />
    </div>
    <div class="flex justify-end gap-2">
      <Button type="button" label="取消" severity="secondary" @click="cancelrename"></Button>
      <Button type="button" label="确认" @click="confirmrename"></Button>
    </div>
  </Dialog>

  <Dialog v-model:visible="nulldir_visible" modal header="查找空目录" :style="{ width: '90%' }">
    <div class="card">
      <DataTable v-model:selection="selectedNullDir" :value="nulldirs" dataKey="path" tableStyle="min-width: 50rem">
        <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
        <Column field="name" header="名称"></Column>
        <Column field="path" header="路径"></Column>
        <Column field="formatted_size" header="大小"></Column>
        <Column field="created" header="创建时间"></Column>
        <Column field="modified" header="修改时间"></Column>
        <Column field="accessed" header="访问时间"></Column>
      </DataTable>
      <div class="flex justify-end gap-2">
        <Button type="button" label="取消" severity="secondary" @click="cancelnull"></Button>
        <Button type="button" label="删除" @click="confirmnull"></Button>
      </div>
    </div>
  </Dialog>

  <Dialog v-model:visible="eqfile_visible" modal header="查找空目录" :style="{ width: '90%' }">
    <div class="card">
      <DataTable v-model:selection="selectedEqFiles" :value="eqfiles" dataKey="path" tableStyle="min-width: 50rem">
        <Column selectionMode="multiple" headerStyle="width: 3rem"></Column>
        <Column field="name" header="名称"></Column>
        <Column field="path" header="路径"></Column>
        <Column field="formatted_size" header="大小"></Column>
        <Column field="created" header="创建时间" style="min-width: 180px"></Column>
        <Column field="modified" header="修改时间" style="min-width: 180px"></Column>
        <Column field="accessed" header="访问时间" style="min-width: 180px"></Column>
      </DataTable>
      <div class="flex justify-end gap-2">
        <Button type="button" label="取消" severity="secondary" @click="canceleqfile"></Button>
        <Button type="button" label="删除" @click="confirmeqfile"></Button>
      </div>
    </div>
  </Dialog>
</template>
<style></style>