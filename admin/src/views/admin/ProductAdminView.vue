<template>
  <div class="space-y-4">
    <h1 class="text-3xl font-bold">{{ $t("products.title") }}</h1>

    <div class="bg-white p-4 rounded shadow">
      <div class="flex flex-wrap gap-3 items-end">
        <el-input v-model="query.name" style="width: 200px" :placeholder="$t('products.search_name')" />
        <el-input v-model="query.product_id" style="width: 200px" :placeholder="$t('products.search_product_id')" />
        <el-select v-model="query.status" style="width: 160px" :placeholder="$t('products.search_status')">
          <el-option :label="$t('common.lang_en') && $t('products.status_1')" :value="1" />
          <el-option :label="$t('products.status_0')" :value="0" />
        </el-select>
        <el-button type="primary" @click="handleSearch">{{ $t("common.search") }}</el-button>
        <el-button @click="handleReset">{{ $t("common.reset") }}</el-button>
      </div>
    </div>

    <div class="flex justify-between items-center">
      <div></div>
      <el-button type="primary" @click="handleCreate">{{ $t("common.create") }}</el-button>
    </div>

    <el-table :data="list" v-loading="loading" border>
      <el-table-column prop="id" label="ID" width="80" />
      <el-table-column prop="name" :label="$t('products.name')" />
      <el-table-column prop="product_id" :label="$t('products.product_id')" />
      <el-table-column prop="price" :label="$t('products.price')" />
      <el-table-column prop="app_id" :label="$t('products.app_id')" />
      <el-table-column prop="add_valid_days" :label="$t('products.add_valid_days')" />
      <el-table-column prop="status" :label="$t('products.status')" />
      <el-table-column fixed="right" :label="$t('common.actions')" width="180">
        <template #default="{ row }">
          <el-button link type="primary" @click="handleEdit(row)">{{ $t("common.edit") }}</el-button>
          <el-button link type="danger" @click="handleDelete(row)">{{ $t("common.delete") }}</el-button>
        </template>
      </el-table-column>
    </el-table>

    <div class="flex justify-end">
      <el-pagination
        v-model:current-page="page"
        v-model:page-size="pageSize"
        :total="total"
        layout="prev, pager, next, jumper"
        @current-change="load"
        @size-change="load"
      />
    </div>

    <el-dialog v-model="formVisible" :title="isEdit ? $t('common.edit') : $t('common.create')" width="600">
      <el-form label-width="140px" ref="formRef" :rules="rules" :model="form">
        <el-form-item :label="$t('products.name')" prop="name"><el-input v-model="form.name" /></el-form-item>
        <el-form-item :label="$t('products.product_id')" prop="product_id"><el-input v-model="form.product_id" /></el-form-item>
        <el-form-item :label="$t('products.app_id')" prop="app_id">
          <el-select v-model="(form as any).app_id" filterable style="width: 100%" :placeholder="$t('products.app_id')">
            <el-option v-for="app in appOptions" :key="app.id" :label="app.name" :value="app.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('products.price')" prop="price"><el-input-number v-model="form.price" /></el-form-item>
        <el-form-item :label="$t('products.add_valid_days')" prop="add_valid_days"><el-input-number v-model="form.add_valid_days" /></el-form-item>
        <el-form-item :label="$t('products.image_url')" prop="image_url"><el-input /></el-form-item>
        <el-form-item :label="$t('products.tags')" prop="tags"><el-input-tag v-model="form.tags" delimiter="|"  /></el-form-item>
        <el-form-item :label="$t('products.status')" prop="status">
          <el-select v-model="form.status" style="width: 160px">
            <el-option :label="$t('products.status_1')" :value="1" />
            <el-option :label="$t('products.status_0')" :value="0" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('products.remark')" prop="remark"><el-input  type="textarea" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="formVisible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submit(formRef)">{{ $t("common.save") }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { ElMessage, ElMessageBox, ElInputTag } from "element-plus";
import { fetchProducts, createProduct, updateProduct, deleteProduct, fetchApps } from "@/apis";
import type { AppModel } from "@/types/apps";
import type { ProductModel, AddProductReq, UpdateProductReq, ListProductsParams } from "@/types/products";
import type { FormInstance ,FormRules} from "element-plus";
import { useI18n } from "vue-i18n";
const { t } = useI18n();

const loading = ref(false);
const list = ref<ProductModel[]>([]);
const total = ref(0);
const page = ref(1);
const pageSize = ref(10);

const query = reactive<ListProductsParams>({ name: "", product_id: "", status: undefined });

const formVisible = ref(false);
const isEdit = ref(false);
const currentId = ref<number | null>(null);
const emptyForm = {
  name: "",
  price: 0,
  app_id: undefined,
  product_id: "",
  add_valid_days: 0,
  image_url: "",
  tags: [],
  status: 1,
  remark: "",
}
const form = reactive<AddProductReq | UpdateProductReq>({...emptyForm});

const formRef = ref<FormInstance>();
const appOptions = ref<AppModel[]>([]);
const rules = reactive<FormRules<AddProductReq>>({
  name: [{ required: true, message: t("products.input_name") }],
  product_id: [{ required: true, message: t("products.input_product_id") }],
  app_id: [{ required: true, message: t("products.input_app_id") }],
  price: [{ required: true, message: t("products.input_price"),min:1 ,type:"number"}],
  add_valid_days: [{ required: true, message: t("products.input_add_valid_days") }],
});

const resetForm = () => {
  Object.assign(form, emptyForm);
  currentId.value = null;
  isEdit.value = false;
};

const load = async () => {
  loading.value = true;
  try {
    const res = await fetchProducts({ page: page.value, page_size: pageSize.value, ...query });
    list.value = res.list;
    total.value = res.total;
    page.value = res.page;
  } finally {
    loading.value = false;
  }
};

const loadApps = async () => {
  const res = await fetchApps({ page: 1, page_size: 1000 });
  appOptions.value = res.list;
};

const handleSearch = () => {
  page.value = 1;
  load();
};
const handleReset = () => {
  Object.assign(query, { name: "", product_id: "", status: undefined });
  handleSearch();
};

const handleCreate = async () => {
  resetForm();
  isEdit.value = false;
  formVisible.value = true;
};
const handleEdit = (row: ProductModel) => {
  isEdit.value = true;
  currentId.value = row.id;
  Object.assign(form, row);
  formVisible.value = true;
};
const handleDelete = async (row: ProductModel) => {
  await ElMessageBox.confirm(`${row.name}`, "Confirm delete?");
  await deleteProduct(row.id);
  ElMessage.success("Deleted");
  load();
};

const submit = async (formRef: FormInstance|undefined) => {
  if(!formRef) return;
  const valid = await formRef.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form"));
    return;
  }
  if (isEdit.value && currentId.value) {
    await updateProduct(currentId.value, form as UpdateProductReq);
    ElMessage.success("Updated");
  } else {
    await createProduct(form as AddProductReq);
    ElMessage.success("Created");
  }
  formVisible.value = false;
  load();
};

onMounted(async () => {
  await Promise.all([load(), loadApps()]);
});
</script>
