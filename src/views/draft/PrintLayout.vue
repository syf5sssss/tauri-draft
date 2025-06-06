<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { getdate } from '../../util/dateutil.js'

let msg = ref('');

onMounted(() => {
    msg.value = getdate();
})

async function greet() {
    let res = await invoke('greet', { name: 'tauri' });
    msg.value = res;
}

async function greetbook() {
    let res = await invoke('greetbook');
    msg.value = res;
}

async function greetbooks() {
    let res = await invoke('greetbooks');
    msg.value = res;
}

async function multiplication99() {
    let res = await invoke('multiplication99');
    msg.value = res;
}

async function connect_db() {
    let res = await invoke('connect_db', { dbip: '127.0.0.1', username: 'sa', password: 'Nanhui-380' });
    console.log(res);
    msg.value = res;
}

async function check_and_create_traft_db() {
    let res = await invoke('check_and_create_traft_db', { dbname: 'draft' });
    console.log(res);
    msg.value = res;
}

async function selbook() {
    let res = await invoke('selbook');
    msg.value = res;
}
</script>

<template>
    <div class="multiplication-table">{{ msg }}</div>
    <div>
        <pre>
      <code>
        #[tauri::command]
        fn greetbook() -> String {
            let bk = dto::Book {
                id: 1,
                title: "The Rust Programming Language".to_string(),
                author: "Steve Klabnik".to_string(),
                category: "Programming".to_string(),
            };

            format!("{:?}", bk)
        }
      </code>
      <code>
        //String 可修改内容（可变）可以追加、插入、删除字符 需要修改或拥有数据时用 String
        //&str 总是不可变的（只读视图）无法修改其指向的内容 只需要读取或查看数据时用 &str

        // &str -> String
        let s: &str = "hello";
        let owned: String = s.to_string();   // 方法1
        let owned: String = String::from(s); // 方法2
        let owned: String = s.into();        // 方法3 (类型推断时)

        // String -> &str
        let owned = String::from("hello");
        let slice: &str = &owned;         // 方法1: 自动解引用强制转换
        let slice: &str = owned.as_str(); // 方法2: 显式方法

        //Rust 的这种区分解决了系统编程中的关键问题：

        //零成本抽象：&str 提供对文本的安全访问而不增加开销

        //明确所有权：防止内存泄漏和悬垂指针

        //高效内存使用：避免不必要的复制

        //安全并发：不可变的 &str 可以安全地在线程间共享
      </code>
    </pre>
    </div>
    <Button @click="greet">greet</Button>
    <Button @click="greetbook" class="ml-2">greetBook</Button>
    <Button @click="greetbooks" class="ml-2">greetBooks</Button>
    <Button @click="multiplication99" class="ml-2">multiplication99</Button>
    <Button @click="connect_db" class="ml-2">connect_db</Button>
    <Button @click="check_and_create_traft_db" class="ml-2 mt-2">check_and_create_traft_db</Button>
    <Button @click="selbook" class="ml-2">selbook</Button>
</template>

<style>
.multiplication-table {
    white-space: pre-wrap;
    font-family: 'Courier New', monospace;
    line-height: 1.5;
    padding: 1rem;
    background: #f8f9fa;
    border-radius: 4px;
    tab-size: 8;
}
</style>