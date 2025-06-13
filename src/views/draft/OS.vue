<script setup>
import { ref, onMounted } from 'vue'
import { eol, platform, family, version, type, arch, locale, exeExtension, hostname } from '@tauri-apps/plugin-os';

const currentPlatform = ref("");
const current_eol = ref("");
const current_version = ref("");
const current_family = ref("");
const current_type = ref("");
const current_arch = ref("");
const current_locale = ref("");
const current_exeExtension = ref("");
const current_hostname = ref("");

onMounted(() => {
    getos();
})

async function getos() {
   currentPlatform.value = platform();
   current_eol.value = eol();
   current_version.value = version();
   current_family.value = family();
   current_type.value = type();
   current_arch.value = arch();
   current_locale.value = await locale();
   current_exeExtension.value = exeExtension();
   current_hostname.value = await hostname();

  console.log('platform:', currentPlatform.value);
  console.log('eol:', current_eol.value);
  console.log('version:', current_version.value);
  console.log('family:', current_family.value);
  console.log('type:', current_type.value);
  console.log('arch:', current_arch.value);
  console.log('locale:', current_locale.value);
  console.log('exeExtension:', current_exeExtension.value);
  console.log('hostname:', current_hostname.value);
}

function escapedValue() {
      return current_eol.value
        .replace(/\r/g, '\\r')  // 将回车符转换为\r
        .replace(/\n/g, '\\n')  // 将换行符转换为\n
        .replace(/\t/g, '\\t')  // 可选：将制表符转换为\t
        .replace(/\s/g, function(match) {
          // 可选：将其他空白字符转换为可见形式
          return match === ' ' ? ' ' : '\\x' + match.charCodeAt(0).toString(16);
        });
    }
</script>

<template>
    <div class="grid grid-cols-12 gap-8">
        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">platform</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{currentPlatform}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取当前操作系统的平台名称</span>
                <span class="text-muted-color" style="display: block;">返回值：如 'windows'、'macos'、'linux' </span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">eol</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl" >{{escapedValue()}}
                        </div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取当前操作系统的行结束符</span>
                <span class="text-muted-color" style="display: block;">返回值：如 '\r\n' (Windows) 或 '\n' (Unix/Linux/Mac)</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">version</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_version}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取当前操作系统的版本号</span>
                <span class="text-muted-color" style="display: block;">返回值：如 '10.0.19043' (Windows 10) 或 '11.6' (macOS Big Sur)</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">family</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_family}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取操作系统的家族类型</span>
                <span class="text-muted-color" style="display: block;">返回值：如 'windows'、'unix'、'darwin' 等</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">type</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_type}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取操作系统类型</span>
                <span class="text-muted-color" style="display: block;">返回值：如 'Windows_NT'、'Darwin'、'Linux' 等</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">arch</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_arch}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取系统架构</span>
                <span class="text-muted-color" style="display: block;">返回值：如 'x64'、'arm64'、'ia32' 等</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">locale</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_locale}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取系统区域设置</span>
                <span class="text-muted-color" style="display: block;">返回值：如 'zh-CN'、'en-US' 等语言代码</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">hostname</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_hostname}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取主机名</span>
                <span class="text-muted-color" style="display: block;">返回值：当前计算机的名称，如 'DESKTOP-ABC123'</span>
            </div>
        </div>

        <div class="col-span-12 lg:col-span-6 xl:col-span-4">
            <div class="card mb-0">
                <div class="flex justify-between mb-4">
                    <div>
                        <span class="block text-muted-color font-medium mb-4">exeExtension</span>
                        <div class="text-surface-900 dark:text-surface-0 font-medium text-xl">{{current_exeExtension}}</div>
                    </div>
                    <div class="flex items-center justify-center bg-blue-100 dark:bg-blue-400/10 rounded-border" style="width: 2.5rem; height: 2.5rem">
                        <i class="pi pi-shopping-cart text-blue-500 !text-xl"></i>
                    </div>
                </div>
                <span class="text-primary font-medium" style="display: block;">功能：获取可执行文件的扩展名</span>
                <span class="text-muted-color" style="display: block;">值：在 Windows 上为 '.exe'，在其他系统上为空字符串 ''</span>
            </div>
        </div>
    </div>
    

  <Button @click="getos" class="ml-2 mt-2" fluid >getos</Button>
</template>

<style>
  .raw-text {
    white-space: pre-wrap;  /* 保留空白和换行符 */
    font-family: monospace; /* 使用等宽字体，确保显示效果一致 */
    background-color: #f5f5f5; /* 添加背景色增强可读性 */
    padding: 10px;
    border-radius: 4px;
    overflow-x: auto; /* 当内容过长时添加水平滚动 */
  }
</style>