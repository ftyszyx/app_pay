<template>
  <div class="space-y-4">
    <h1 class="text-3xl font-bold">{{ $t('orders.title') }}</h1>

    <el-card shadow="hover">
      <div class="flex flex-wrap gap-3 items-end">
        <el-input  v-model="query.order_id" class="w-52" :placeholder="$t('orders.search_order_id')" clearable />
        <el-select  v-model="query.status" class="w-40" :placeholder="$t('orders.search_status')" clearable>
          <el-option :label="$t('orders.status_0')" :value="0" />
          <el-option :label="$t('orders.status_1')" :value="1" />
          <el-option :label="$t('orders.status_2')" :value="2" />
        </el-select>
        <el-date-picker class="w-22"  v-model="createdRange"  type="datetimerange" range-separator="-" :start-placeholder="$t('orders.order_time_start')"
         :end-placeholder="$t('orders.order_time_end')" />
         <div class="flex-1"></div>
        <el-button type="primary" @click="reload"><el-icon class="mr-1"><Search/></el-icon>{{ $t('common.search') }}</el-button>
        <el-button @click="reset"><el-icon class="mr-1"><Refresh/></el-icon>{{ $t('common.reset') }}</el-button>
      </div>
    </el-card>

    <el-card shadow="never">
      <el-table :data="list" v-loading="loading" stripe size="large">
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column prop="order_id" :label="$t('orders.order_id')" min-width="160" />
        <el-table-column prop="pay_method_name" :label="$t('orders.pay_method')" min-width="140" />
        <el-table-column :label="$t('orders.original_price')" width="120"><template #default="{ row }">{{ row.original_price }}</template></el-table-column>
        <el-table-column :label="$t('orders.final_price')" width="120"><template #default="{ row }">{{ row.final_price }}</template></el-table-column>
        <el-table-column :label="$t('orders.status')" width="120"><template #default="{ row }"><el-tag :type="row.status === 1 ? 'success' : (row.status === 2 ? 'info' : 'warning')">{{ row.status === 1 ? $t('orders.status_1') : (row.status === 2 ? $t('orders.status_2') : $t('orders.status_0')) }}</el-tag></template></el-table-column>
        <el-table-column prop="created_at" :label="$t('orders.created')" min-width="180" />
        <el-table-column prop="updated_at" :label="$t('orders.updated')" min-width="180" />
      </el-table>
      <div class="flex justify-end mt-4">
        <el-pagination background layout="total, sizes, prev, pager, next, jumper" :page-sizes="[10,20,50,100]" :page-size="pageSize" :current-page="page" :total="total" @current-change="handlePageChange" @size-change="handleSizeChange" />
      </div>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { fetchOrders } from '@/apis'
import type { OrderInfo, ListOrdersParams } from '@/types/orders'

const loading = ref(false)
const list = ref<OrderInfo[]>([])
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)

const query = reactive<ListOrdersParams>({ order_id: '', status: undefined })
const createdRange = ref<[Date, Date] | undefined>()

const reload = async () => {
  loading.value = true
  try {
    const params: any = { page: page.value, page_size: pageSize.value, ...query }
    if (createdRange.value && createdRange.value.length === 2) {
      params.created_start = createdRange.value[0].toISOString()
      params.created_end = createdRange.value[1].toISOString()
    }
    const data = await fetchOrders(params)
    list.value = data.list
    total.value = data.total
  } finally { loading.value = false }
}

const handlePageChange = async (p: number) => { page.value = p; await reload() }
const handleSizeChange = async (s: number) => { pageSize.value = s; page.value = 1; await reload() }
const reset = async () => { query.order_id = ''; query.status = undefined; createdRange.value = undefined; page.value = 1; await reload() }

onMounted(reload)
</script>