<script setup>
import { useToast } from 'primevue/usetoast';
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import Paginator from 'primevue/paginator';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import IconField from 'primevue/iconfield';
import InputText from 'primevue/inputtext';
import FloatLabel from 'primevue/floatlabel';
import Panel from 'primevue/panel';
import DatePicker from 'primevue/datepicker';
import Toast from 'primevue/toast';
import FileUpload from 'primevue/fileupload';

const imgpath = ref();
const toast = useToast();
const dt = ref();
const books = ref();
const bookDialog = ref(false);
const deleteBookDialog = ref(false);
const deleteBooksDialog = ref(false);
const book = ref({});
const selectedBooks = ref();

const currentPage = ref(1);
const pageSize = ref(10);
const totalRecords = ref(0);

const author_s = ref(null);
const title_s = ref(null);
const rating_s = ref(null);
const img_s = ref(null);
const min_price_s = ref(null);
const max_price_s = ref(null);
const min_sales_s = ref(null);
const max_sales_s = ref(null);
const category_s = ref(null);
const status_s = ref(null);
const min_publish_date_s = ref(null);
const max_publish_date_s = ref(null);

const statuses = ref([
    { label: '多库存', value: 'INSTOCK' },
    { label: '低库存', value: 'LOWSTOCK' },
    { label: '无库存', value: 'OUTOFSTOCK' }
]);
const categorys = ref([
    { label: '医术', value: '1' },
    { label: '哲学', value: '2' },
    { label: '其他', value: '0' }
]);
const ratings = ref([
    { label: '一级', value: '1' },
    { label: '二级', value: '2' },
    { label: '三级', value: '3' },
    { label: '四级', value: '4' },
    { label: '五级', value: '5' }
]);

onMounted(async () => {
    imgpath.value = await invoke('get_pictures_dir');
    console.log(imgpath.value);
    search();
});

async function search() {
    let res = await invoke('search', { query: '', currentPage: currentPage.value, pageSize: pageSize.value });
    // console.log(res);
    // console.log(res.total);
    books.value = res.data;
    totalRecords.value = res.total;
}

async function dynamics_search() {
    const query = {
        title: title_s.value,
        author: author_s.value,
        rating: rating_s.value ? parseInt(rating_s.value.value) : null,
        img: img_s.value,
        min_price: min_price_s.value ? parseFloat(min_price_s.value) : null,
        max_price: max_price_s.value ? parseFloat(max_price_s.value) : null,
        min_sales: min_sales_s.value ? parseInt(min_sales_s.value) : null,
        max_sales: max_sales_s.value ? parseInt(max_sales_s.value) : null,
        category: category_s.value ? category_s.value.value : null,
        status: status_s.value ? status_s.value.value : null,
        min_publish_date: min_publish_date_s.value ? min_publish_date_s.value.toISOString().split('.')[0] : null,
        max_publish_date: max_publish_date_s.value ? max_publish_date_s.value.toISOString().split('.')[0] : null
    }
    // console.log(query);
    let res = await invoke('dynamics_search', { query: query, currentPage: currentPage.value, pageSize: pageSize.value });
    // console.log(res);
    // console.log(res.total);
    books.value = res.data;
    totalRecords.value = res.total;
}

async function clear_search() {
    author_s.value = null;
    title_s.value = null;
    rating_s.value = null;
    img_s.value = null;
    min_price_s.value = null;
    max_price_s.value = null;
    min_sales_s.value = null;
    max_sales_s.value = null;
    category_s.value = null;
    status_s.value = null;
    min_publish_date_s.value = null;
    max_publish_date_s.value = null;
}

function formatCurrency(value) {
    if (value) return value.toLocaleString('zh-CN', { style: 'currency', currency: 'CNY' });
    // if (value) return value.toLocaleString('en-US', { style: 'currency', currency: 'USD' });
    return;
}

function openNew() {
    book.value = {};
    bookDialog.value = true;
}

function hideDialog() {
    bookDialog.value = false;
}

async function saveBook() {
    if (book?.value) {
        if (book.value.id) {
            book.value.status = book.value.status.value ? book.value.status.value : book.value.status;
            book.value.category = book.value.category.value ? book.value.category.value : book.value.category;
            book.value.rating = book.value.rating.value ? book.value.rating.value : book.value.rating;
            book.value.rating = parseInt(book.value.rating);
            book.value.publish_date = new Date(book.value.publish_date);
            console.log(book.value);
            invoke('update', {
                book: book.value
            }).then(result => {
                console.log('更新成功,ID:', result);
                toast.add({ severity: 'success', summary: '成功', detail: '书籍已更新', life: 3000 });
                search();
            }).catch(error => {
                console.error('更新失败:', error);
                toast.add({ severity: 'error', summary: '失败', detail: '书籍未更新', life: 3000 });
            });
        } else {
            book.value.status = book.value.status.value;
            book.value.rating = parseInt(book.value.rating.value);
            book.value.category = book.value.category.value;
            console.log(book.value);
            invoke('create', {
                book: book.value
            }).then(result => {
                console.log('添加成功,ID:', result);
                toast.add({ severity: 'success', summary: '成功', detail: '书籍已添加', life: 3000 });
                search();
            }).catch(error => {
                console.error('添加失败:', error);
                toast.add({ severity: 'error', summary: '失败', detail: '书籍未添加', life: 3000 });
            });
        }
        bookDialog.value = false;
        book.value = {};
    }
}

function editBook(prod) {
    book.value = { ...prod };
    const originalStatus = book.value.status;
    const originalCategory = book.value.category;
    const originalRating = book.value.rating.toString();

    const selectedStatus = statuses.value.find(
        item => item.value === originalStatus
    );
    const selectedCategory = categorys.value.find(
        item => item.value === originalCategory
    );
    const selectedRating = ratings.value.find(
        item => item.value === originalRating
    );

    if (selectedStatus) {
        book.value.status = selectedStatus;
    } else {
        console.log("没找到状态匹配项，设置为第一个选项或空值");
        book.value.status = statuses.value[0] || null;
    }
    if (selectedCategory) {
        book.value.category = selectedCategory;
    } else {
        console.log("没找到类型匹配项，设置为第一个选项或空值");
        book.value.category = categorys.value[0] || null;
    }
    if (selectedRating) {
        book.value.rating = selectedRating;
    } else {
        console.log("没找到等级匹配项，设置为第一个选项或空值");
        book.value.rating = ratings.value[0] || null;
    }
    bookDialog.value = true;
}

function confirmDeleteBook(prod) {
    book.value = prod;
    deleteBookDialog.value = true;
}

function deleteBook() {
    console.log(book.value.id);
    invoke('delete', {
        id: book.value.id
    }).then(result => {
        console.log('删除成功,ID:', result);
        toast.add({ severity: 'success', summary: '成功', detail: '书籍已删除', life: 3000 });
        search();
    }).catch(error => {
        console.error('删除失败:', error);
        toast.add({ severity: 'error', summary: '失败', detail: '书籍未删除', life: 3000 });
    });

    deleteBookDialog.value = false;
    book.value = {};
}


function exportCSV() {
    dt.value.exportCSV();
}

function confirmDeleteSelected() {
    deleteBooksDialog.value = true;
}

function deleteSelectedBooks() {
    const ids = selectedBooks.value.map(book => book.id);
    invoke('deletes', {
        ids: ids
    }).then(result => {
        console.log('删除成功,ID:', result);
        toast.add({ severity: 'success', summary: '成功', detail: '书籍已删除', life: 3000 });
        search();
    }).catch(error => {
        console.error('删除失败:', error);
        toast.add({ severity: 'error', summary: '失败', detail: '书籍未删除', life: 3000 });
    });

    deleteBooksDialog.value = false;
    selectedBooks.value = null;
    toast.add({ severity: 'success', summary: 'Successful', detail: '书籍已删除', life: 3000 });
}

function getStatusLabel(status) {
    switch (status) {
        case 'INSTOCK':
            return 'success';

        case 'LOWSTOCK':
            return 'warn';

        case 'OUTOFSTOCK':
            return 'danger';

        default:
            return null;
    }
}

function getCategoryLabel(category) {
    switch (category) {
        case '0':
            return '其他';

        case '1':
            return '医书';

        case '2':
            return '哲学';

        default:
            return '其他';
    }
}

const formatDateTime = (isoString) => {
    if (!isoString) return '';
    const date = new Date(isoString);
    if (isNaN(date.getTime())) return '无效日期';

    return `${date.getFullYear()}年${(date.getMonth() + 1).toString().padStart(2, '0')}月${date.getDate().toString().padStart(2, '0')}日 ${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}:${date.getSeconds().toString().padStart(2, '0')}`;
};
const getLabelByValue = (targetValue) => {
    const found = statuses.value.find(item => item.value === targetValue);
    return found ? found.label : '未知状态';
};

// 分页变化事件处理
const onPageChange = (event) => {
    currentPage.value = event.page + 1;
    console.log("onPageChange: currentPage", currentPage.value);
    search();
};

// 每页行数变化事件处理
const onPageSizeChange = (newSize) => {
    console.log("onPageSizeChange:", newSize);
    pageSize.value = newSize;
    currentPage.value = 1;
};

const onSelect = async (event) => {
    const file = event.files[0];
    if (!file) return;

    try {
        const buffer = await file.arrayBuffer();
        let filename = await invoke('save_image', {
            bytes: Array.from(new Uint8Array(buffer))
        });
        book.value.img = filename;
    } catch (error) {
        console.error('上传失败:', error);
    }
};

</script>

<template>
    <div>
        <div class="card">
            <Panel header="条件筛选" toggleable :collapsed="true">
                <div class="flex flex-wrap gap-2 items-center justify-between">
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="author_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">作者</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="title_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">标题</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="img_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">图片</label>
                    </FloatLabel>
                    <!-- <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="rating_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">等级</label>
                    </FloatLabel> -->
                    <Select v-model="rating_s" :options="ratings" optionLabel="label" placeholder="等级" showClear
                        class="w-48" />
                    <!-- <FloatLabel variant="on">
                        <IconField>getRatingLabel
                            <InputText id="on_label" v-model="category_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">类别</label>
                    </FloatLabel>

                    <Select id="category" v-model="book.category" :options="categorys" optionLabel="label"
                        placeholder="选择一个类别" fluid></Select> -->

                    <Select v-model="category_s" :options="categorys" optionLabel="label" placeholder="类别" showClear
                        class="w-48" />

                    <Select v-model="status_s" :options="statuses" optionLabel="label" placeholder="库存状态" showClear
                        class="w-48" />
                </div>
                <div class="flex flex-wrap gap-2 items-center justify-between mt-2">
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="min_sales_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">销量大于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="max_sales_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">销量小于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="min_price_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">价格大于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="max_price_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">价格小于</label>
                    </FloatLabel>
                    <DatePicker v-model="min_publish_date_s" showTime hourFormat="24" dateFormat="yy-mm-dd"
                        class="w-48" />
                    <DatePicker v-model="max_publish_date_s" showTime hourFormat="24" dateFormat="yy-mm-dd"
                        class="w-48" />
                </div>
            </Panel>

            <DataTable ref="dt" v-model:selection="selectedBooks" :value="books" dataKey="id" :rows="pageSize">
                <template #header>
                    <div class="flex flex-wrap gap-2 items-center justify-between">
                        <div>
                            <Button label="新建" icon="pi pi-plus" severity="secondary" class="mr-2" @click="openNew" />
                            <Button label="删除" icon="pi pi-trash" severity="secondary" class="mr-2"
                                @click="confirmDeleteSelected" :disabled="!selectedBooks || !selectedBooks.length" />
                            <Button label="清空" icon="pi pi-filter-slash" severity="secondary" class="mr-2"
                                @click="clear_search" />
                            <Button label="查询" icon="pi pi-search" severity="secondary" class="mr-2"
                                @click="dynamics_search" />
                        </div>
                        <div>
                            <Button label="导出" icon="pi pi-upload" severity="secondary" @click="exportCSV($event)" />
                        </div>
                    </div>
                </template>
                <template #footer>
                    <Paginator :rows="pageSize" :totalRecords="totalRecords" :rowsPerPageOptions="[10, 20, 30]"
                        @page="onPageChange" @update:rows="onPageSizeChange">
                    </Paginator>
                </template>

                <Column selectionMode="multiple" style="width: 3rem" :exportable="false"></Column>
                <Column field="author" header="作者" style="min-width: 6rem;text-align: center;"></Column>
                <Column field="title" header="标题" style="min-width: 10rem"></Column>
                <!-- :src="`/demo/images/draft/${slotProps.data.img}`" -->
                <Column header="封面" style="min-width: 6rem">
                    <template #body="slotProps">
                        <img :src="`http://asset.localhost/${imgpath}/Draft/Img/${slotProps.data.img}`"
                            :alt="slotProps.data.img" class="rounded" style="width: 6rem" />
                    </template>
                </Column>
                <Column field="price" header="价格" style="min-width: 6rem">
                    <template #body="slotProps">
                        {{ formatCurrency(slotProps.data.price) }}
                    </template>
                </Column>
                <Column field="sales" header="销量" style="min-width: 6rem">
                    <template #body="slotProps">
                        {{ slotProps.data.sales }}
                    </template>
                </Column>
                <Column field="publish_date" header="上市" style="min-width: 10rem">
                    <template #body="slotProps">
                        {{ formatDateTime(slotProps.data.publish_date) }}
                    </template>
                </Column>
                <Column field="category" header="类别" style="min-width: 6rem">
                    <template #body="slotProps">
                        <Tag :value="getCategoryLabel(slotProps.data.category)"></Tag>
                    </template>
                </Column>
                <Column field="rating" header="等级" style="min-width: 6rem">
                    <template #body="slotProps">
                        <Rating :modelValue="slotProps.data.rating" :readonly="true" />
                    </template>
                </Column>
                <Column field="status" header="状态" style="min-width: 6rem">
                    <template #body="slotProps">
                        <Tag :value="getLabelByValue(slotProps.data.status)"
                            :severity="getStatusLabel(slotProps.data.status)" />
                    </template>
                </Column>
                <Column :exportable="false" style="min-width: 8rem" header="操作">
                    <template #body="slotProps">
                        <Button icon="pi pi-pencil" outlined rounded class="mr-2" @click="editBook(slotProps.data)" />
                        <Button icon="pi pi-trash" outlined rounded severity="danger"
                            @click="confirmDeleteBook(slotProps.data)" />
                    </template>
                </Column>
            </DataTable>
        </div>
        <Toast />
        <Dialog v-model:visible="bookDialog" :style="{ width: '450px' }" header="书籍详情" :modal="true">
            <div class="flex flex-col gap-6">
                <img v-if="book.img" :src="`http://asset.localhost/${imgpath}/Draft/Img/${book.img}`" :alt="book.img"
                    class="block m-auto pb-4" />
                <FileUpload mode="basic" name="demo[]" url="" accept="image/*" :maxFileSize="100000000"
                    @select="onSelect" chooseLabel="封面" auto />
                <div>
                    <label for="title" class="block font-bold mb-3">标题</label>
                    <InputText id="title" v-model.trim="book.title" required="true" fluid />
                </div>
                <div>
                    <label for="author" class="block font-bold mb-3">作者</label>
                    <InputText id="author" v-model.trim="book.author" required="true" fluid />
                </div>
                <div>
                    <label for="status" class="block font-bold mb-3">库存状态</label>
                    <Select id="status" v-model="book.status" :options="statuses" optionLabel="label"
                        placeholder="选择一个状态" fluid></Select>
                </div>
                <div>
                    <label for="rating" class="block font-bold mb-3">等级</label>
                    <Select id="rating" v-model="book.rating" :options="ratings" optionLabel="label"
                        placeholder="选择一个等级" fluid></Select>
                </div>

                <div>
                    <label for="category" class="block font-bold mb-4">类别</label>
                    <Select id="category" v-model="book.category" :options="categorys" optionLabel="label"
                        placeholder="选择一个类别" fluid></Select>
                </div>

                <div>
                    <label for="publish_date" class="block font-bold mb-3">发布时间</label>
                    <DatePicker v-model="book.publish_date" showTime hourFormat="24" dateFormat="yy-mm-dd" fluid />
                </div>

                <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-6">
                        <label for="price" class="block font-bold mb-3">价格</label>
                        <InputNumber id="price" v-model="book.price" mode="currency" currency="CNY" locale="zh-CN"
                            fluid />
                    </div>
                    <div class="col-span-6">
                        <label for="sales" class="block font-bold mb-3">销量</label>
                        <InputNumber id="sales" v-model="book.sales" integeronly fluid />
                    </div>
                </div>
            </div>

            <template #footer>
                <Button label="取消" icon="pi pi-times" text @click="hideDialog" />
                <Button label="保存" icon="pi pi-check" @click="saveBook" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteBookDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="book">你想要删除 <b>{{ book.name }}</b>?</span>
            </div>
            <template #footer>
                <Button label="取消" icon="pi pi-times" text @click="deleteBookDialog = false" />
                <Button label="确认" icon="pi pi-check" @click="deleteBook" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteBooksDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="book">你确定想要删除选中的书籍吗?</span>
            </div>
            <template #footer>
                <Button label="取消" icon="pi pi-times" text @click="deleteBooksDialog = false" />
                <Button label="确认" icon="pi pi-check" text @click="deleteSelectedBooks" />
            </template>
        </Dialog>
    </div>
</template>

<style scoped></style>