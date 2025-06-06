<script setup>
import { FilterMatchMode } from '@primevue/core/api';
import { useToast } from 'primevue/usetoast';
import { onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

import Paginator from 'primevue/paginator';
import DataTable from 'primevue/datatable';
import Column from 'primevue/column';
import ColumnGroup from 'primevue/columngroup';
import Row from 'primevue/row';
import IconField from 'primevue/iconfield';
import InputIcon from 'primevue/inputicon';
import InputText from 'primevue/inputtext';
import FloatLabel from 'primevue/floatlabel';
import Card from 'primevue/card';
import Panel from 'primevue/panel';

import DatePicker from 'primevue/datepicker';


onMounted(async () => {
    search();
});

const toast = useToast();
const dt = ref();
const products = ref();
const productDialog = ref(false);
const deleteProductDialog = ref(false);
const deleteProductsDialog = ref(false);
const product = ref({});
const selectedProducts = ref();

// 分页状态
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

const filters = ref({
    global: { value: null, matchMode: FilterMatchMode.CONTAINS }
});
const submitted = ref(false);
const statuses = ref([
    { label: '多库存', value: 'INSTOCK' },
    { label: '低库存', value: 'LOWSTOCK' },
    { label: '无库存', value: 'OUTOFSTOCK' }
]);

async function search() {
    let res = await invoke('search', { query: '', currentPage: currentPage.value, pageSize: pageSize.value });
    console.log(res);
    console.log(res.total);
    products.value = res.data;
    totalRecords.value = res.total;
}

async function test() {
    const query = {
        title: "2"
    }
    let res = await invoke('dynamics_search', { query: query, currentPage: currentPage.value, pageSize: pageSize.value });
    console.log(res);
    console.log(res.total);
    products.value = res.data;
    totalRecords.value = res.total;
}

async function dynamics_search() {
    const query = {
        title: title_s.value,
        author: author_s.value,
        rating: rating_s.value ? parseInt(rating_s.value) : null,
        img: img_s.value,
        min_price: min_price_s.value ? parseFloat(min_price_s.value) : null,
        max_price: max_price_s.value ? parseFloat(max_price_s.value) : null,
        min_sales: min_sales_s.value ? parseInt(min_sales_s.value) : null,
        max_sales: max_sales_s.value ? parseInt(max_sales_s.value) : null,
        category: category_s.value,
        status: status_s.value ? status_s.value.value : null,
        min_publish_date: min_publish_date_s.value,
        max_publish_date: max_publish_date_s.value,
    }
    console.log(query);
    let res = await invoke('dynamics_search', { query: query, currentPage: currentPage.value, pageSize: pageSize.value });

    console.log(res);
    console.log(res.total);
    products.value = res.data;
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
    product.value = {};
    submitted.value = false;
    productDialog.value = true;
}

function hideDialog() {
    productDialog.value = false;
    submitted.value = false;
}

function saveProduct() {
    submitted.value = true;

    if (product?.value.name?.trim()) {
        if (product.value.id) {
            product.value.inventoryStatus = product.value.inventoryStatus.value ? product.value.inventoryStatus.value : product.value.inventoryStatus;
            products.value[findIndexById(product.value.id)] = product.value;
            toast.add({ severity: 'success', summary: 'Successful', detail: 'Product Updated', life: 3000 });
        } else {
            product.value.id = createId();
            product.value.code = createId();
            product.value.image = 'product-placeholder.svg';
            product.value.inventoryStatus = product.value.inventoryStatus ? product.value.inventoryStatus.value : 'INSTOCK';
            products.value.push(product.value);
            toast.add({ severity: 'success', summary: 'Successful', detail: 'Product Created', life: 3000 });
        }

        productDialog.value = false;
        product.value = {};
    }
}

function editProduct(prod) {
    product.value = { ...prod };
    productDialog.value = true;
}

function confirmDeleteProduct(prod) {
    product.value = prod;
    deleteProductDialog.value = true;
}

function deleteProduct() {
    products.value = products.value.filter((val) => val.id !== product.value.id);
    deleteProductDialog.value = false;
    product.value = {};
    toast.add({ severity: 'success', summary: 'Successful', detail: 'Product Deleted', life: 3000 });
}

function findIndexById(id) {
    let index = -1;
    for (let i = 0; i < products.value.length; i++) {
        if (products.value[i].id === id) {
            index = i;
            break;
        }
    }

    return index;
}

function createId() {
    let id = '';
    var chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    for (var i = 0; i < 5; i++) {
        id += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return id;
}

function exportCSV() {
    dt.value.exportCSV();
}

function confirmDeleteSelected() {
    deleteProductsDialog.value = true;
}

function deleteSelectedProducts() {
    products.value = products.value.filter((val) => !selectedProducts.value.includes(val));
    deleteProductsDialog.value = false;
    selectedProducts.value = null;
    toast.add({ severity: 'success', summary: 'Successful', detail: 'Products Deleted', life: 3000 });
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
const formatDateTime = (isoString) => {
    if (!isoString) return '';
    console.log(isoString);
    const date = new Date(isoString);
    console.log(date.getHours().toString());
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
</script>

<template>
    <div>
        <div class="card">
            <!-- <Toolbar class="mb-6">
                <template #start>
                    <Button label="test" icon="pi pi-plus" severity="secondary" class="mr-2" @click="test" />
                    <Select v-model="selectedStatus" :options="statuses" optionLabel="label" placeholder="库存状态"
                        showClear class="w-full md:w-56" />
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="value3" autocomplete="off" />
                        </IconField>
                        <label for="on_label">On Label</label>
                    </FloatLabel>
                </template>

<template #end>
                    <Button label="Export" icon="pi pi-upload" severity="secondary" @click="exportCSV($event)" />
                </template>
</Toolbar> -->
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
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="rating_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">等级</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="category_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">类别</label>
                    </FloatLabel>
                    <Select v-model="status_s" :options="statuses" optionLabel="label" placeholder="库存状态" showClear
                        class="w-48" />
                </div>
                <div class="flex flex-wrap gap-2 items-center justify-between mt-2">
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="min_sales_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">销量小于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="max_sales_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">销量大于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="min_price_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">价格小于</label>
                    </FloatLabel>
                    <FloatLabel variant="on">
                        <IconField>
                            <InputText id="on_label" v-model="max_price_s" autocomplete="off" class="w-48" />
                        </IconField>
                        <label for="on_label">价格大于</label>
                    </FloatLabel>
                    <DatePicker v-model="min_publish_date_s" showTime hourFormat="24" dateFormat="yy-mm-dd"
                        class="w-48" />
                    <DatePicker v-model="max_publish_date_s" showTime hourFormat="24" dateFormat="yy-mm-dd"
                        class="w-48" />
                </div>
            </Panel>


            <DataTable ref="dt" v-model:selection="selectedProducts" :value="products" dataKey="id" :rows="pageSize"
                :filters="filters">
                <template #header>
                    <div class="flex flex-wrap gap-2 items-center justify-between">
                        <div>
                            <Button label="New" icon="pi pi-plus" severity="secondary" class="mr-2" @click="openNew" />
                            <Button label="Delete" icon="pi pi-trash" severity="secondary" class="mr-2"
                                @click="confirmDeleteSelected"
                                :disabled="!selectedProducts || !selectedProducts.length" />
                            <Button label="清空" icon="pi pi-filter-slash" severity="secondary" class="mr-2"
                                @click="clear_search" />
                            <Button label="查询" icon="pi pi-search" severity="secondary" class="mr-2"
                                @click="dynamics_search" />
                        </div>
                        <div>
                            <Button label="Export" icon="pi pi-upload" severity="secondary"
                                @click="exportCSV($event)" />
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
                <Column header="封面" style="min-width: 6rem">
                    <template #body="slotProps">
                        <img :src="`/demo/images/draft/${slotProps.data.img}`" :alt="slotProps.data.img" class="rounded"
                            style="width: 6rem" />
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
                <Column field="category" header="类别" style="min-width: 6rem"></Column>
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
                        <Button icon="pi pi-pencil" outlined rounded class="mr-2"
                            @click="editProduct(slotProps.data)" />
                        <Button icon="pi pi-trash" outlined rounded severity="danger"
                            @click="confirmDeleteProduct(slotProps.data)" />
                    </template>
                </Column>
            </DataTable>
        </div>

        <Dialog v-model:visible="productDialog" :style="{ width: '450px' }" header="Product Details" :modal="true">
            <div class="flex flex-col gap-6">
                <img v-if="product.image" :src="`https://primefaces.org/cdn/primevue/images/product/${product.image}`"
                    :alt="product.image" class="block m-auto pb-4" />
                <div>
                    <label for="name" class="block font-bold mb-3">Name</label>
                    <InputText id="name" v-model.trim="product.name" required="true" autofocus
                        :invalid="submitted && !product.name" fluid />
                    <small v-if="submitted && !product.name" class="text-red-500">Name is required.</small>
                </div>
                <div>
                    <label for="description" class="block font-bold mb-3">Description</label>
                    <Textarea id="description" v-model="product.description" required="true" rows="3" cols="20" fluid />
                </div>
                <div>
                    <label for="inventoryStatus" class="block font-bold mb-3">Inventory Status</label>
                    <Select id="inventoryStatus" v-model="product.inventoryStatus" :options="statuses"
                        optionLabel="label" placeholder="Select a Status" fluid></Select>
                </div>

                <div>
                    <span class="block font-bold mb-4">Category</span>
                    <div class="grid grid-cols-12 gap-4">
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="category1" v-model="product.category" name="category"
                                value="Accessories" />
                            <label for="category1">Accessories</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="category2" v-model="product.category" name="category" value="Clothing" />
                            <label for="category2">Clothing</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="category3" v-model="product.category" name="category"
                                value="Electronics" />
                            <label for="category3">Electronics</label>
                        </div>
                        <div class="flex items-center gap-2 col-span-6">
                            <RadioButton id="category4" v-model="product.category" name="category" value="Fitness" />
                            <label for="category4">Fitness</label>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-12 gap-4">
                    <div class="col-span-6">
                        <label for="price" class="block font-bold mb-3">Price</label>
                        <InputNumber id="price" v-model="product.price" mode="currency" currency="USD" locale="en-US"
                            fluid />
                    </div>
                    <div class="col-span-6">
                        <label for="quantity" class="block font-bold mb-3">Quantity</label>
                        <InputNumber id="quantity" v-model="product.quantity" integeronly fluid />
                    </div>
                </div>
            </div>

            <template #footer>
                <Button label="Cancel" icon="pi pi-times" text @click="hideDialog" />
                <Button label="Save" icon="pi pi-check" @click="saveProduct" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteProductDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="product">Are you sure you want to delete <b>{{ product.name }}</b>?</span>
            </div>
            <template #footer>
                <Button label="No" icon="pi pi-times" text @click="deleteProductDialog = false" />
                <Button label="Yes" icon="pi pi-check" @click="deleteProduct" />
            </template>
        </Dialog>

        <Dialog v-model:visible="deleteProductsDialog" :style="{ width: '450px' }" header="Confirm" :modal="true">
            <div class="flex items-center gap-4">
                <i class="pi pi-exclamation-triangle !text-3xl" />
                <span v-if="product">Are you sure you want to delete the selected products?</span>
            </div>
            <template #footer>
                <Button label="No" icon="pi pi-times" text @click="deleteProductsDialog = false" />
                <Button label="Yes" icon="pi pi-check" text @click="deleteSelectedProducts" />
            </template>
        </Dialog>
    </div>
</template>

<style scoped></style>