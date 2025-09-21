<template>
  <div class="space-y-4">
    <el-card shadow="hover">
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">Registration Codes</h2>
        <div class="flex items-center gap-2">
          <el-input v-model="query.code" placeholder="Code" clearable class="w-56" />
          <el-select v-model.number="query.app_id" placeholder="App" clearable class="w-48">
            <el-option v-for="opt in appOptions" :key="opt.id" :label="opt.name" :value="opt.id" />
          </el-select>
          <el-select v-model="query.code_type" placeholder="Type" clearable class="w-36">
            <el-option label="Time" :value="0" />
            <el-option label="Count" :value="1" />
          </el-select>
          <el-button type="primary" @click="reload">Search</el-button>
          <el-button @click="resetFilters">Reset</el-button>
          <el-button type="success" @click="openBatchDialog">Batch Create</el-button>
          <el-button type="danger" :disabled="!selectedIds.length" @click="batchDelete">Batch Delete</el-button>
          <el-button @click="exportCsv">Export CSV</el-button>
        </div>
      </div>
    </el-card>

    <el-card shadow="never">
      <el-table :data="rows" stripe size="large" style="width: 100%" @selection-change="onSelChange">
        <el-table-column type="selection" width="50" />
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column label="Code" min-width="200">
          <template #default="{ row }">
            <div class="flex items-center gap-2">
              <span class="text-gray-800 break-all">{{ row.code }}</span>
              <el-button size="small" @click="copy(row.code)">Copy</el-button>
            </div>
          </template>
        </el-table-column>
        <el-table-column label="App" min-width="160">
          <template #default="{ row }">
            <span>{{ row.app_name || row.app_id }}</span>
          </template>
        </el-table-column>
        <el-table-column label="Type" width="100">
          <template #default="{ row }">{{ row.code_type === 1 ? 'Count' : 'Time' }}</template>
        </el-table-column>
        <el-table-column label="Total/Days" width="120">
          <template #default="{ row }">{{ row.code_type === 1 ? (row.total_count ?? '-') : row.valid_days }}</template>
        </el-table-column>
        <el-table-column prop="use_count" label="Used" width="100" />
        <el-table-column prop="status" label="Status" width="100" />
        <el-table-column prop="created_at" label="Created" min-width="180" />
        <el-table-column label="Actions" width="120" fixed="right">
          <template #default="{ row }">
            <el-button size="small" type="danger" @click="del(row.id)">Delete</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="flex justify-end mt-4">
        <el-pagination
          background
          layout="total, sizes, prev, pager, next, jumper"
          :page-sizes="[10, 20, 50, 100]"
          :page-size="pageSize"
          :current-page="page"
          :total="total"
          @current-change="handlePageChange"
          @size-change="handleSizeChange"
        />
      </div>
    </el-card>

    <el-dialog v-model="batch.visible" title="Batch Create" width="520px">
      <el-form label-width="140px">
        <el-form-item label="App">
          <el-select v-model.number="batch.app_id" placeholder="Select App" class="w-full">
            <el-option v-for="opt in appOptions" :key="opt.id" :label="opt.name" :value="opt.id" />
          </el-select>
        </el-form-item>
        <el-form-item label="Quantity"><el-input-number v-model.number="batch.quantity" :min="1" /></el-form-item>
        <el-form-item label="Type">
          <el-radio-group v-model="batch.code_type">
            <el-radio :label="0">Time</el-radio>
            <el-radio :label="1">Count</el-radio>
          </el-radio-group>
        </el-form-item>
        <el-form-item v-if="batch.code_type===0" label="Valid Days"><el-input-number v-model.number="batch.valid_days" :min="1" /></el-form-item>
        <el-form-item v-else label="Total Count"><el-input-number v-model.number="batch.total_count" :min="1" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="batch.visible=false">Cancel</el-button>
        <el-button type="primary" @click="submitBatch">Confirm</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { fetchRegCodes, deleteRegCode, batchCreateRegCodes } from '@/apis/reg_codes'
import { fetchApps } from '@/apis/apps'
import type { RegCodeModel, ListRegCodesParams, BatchCreateRegCodesReq } from '@/types/reg_codes'
import { ElMessage, ElMessageBox } from 'element-plus'

const rows = ref<RegCodeModel[]>([])
const appOptions = ref<{id:number,name:string}[]>([])
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const selectedIds = ref<number[]>([])

const query = reactive<ListRegCodesParams>({ code: '', app_id: undefined, code_type: undefined })

async function reload(){
  const data = await fetchRegCodes({ ...query, page: page.value, page_size: pageSize.value })
  rows.value = data.list
  total.value = data.total
}
function resetFilters(){ query.code=''; query.app_id = undefined; query.code_type=undefined; page.value=1; reload() }
function onSelChange(arr: RegCodeModel[]){ selectedIds.value = arr.map(it=>it.id) }
function handlePageChange(p:number){ page.value=p; reload() }
function handleSizeChange(s:number){ pageSize.value=s; page.value=1; reload() }

async function del(id:number){
  await ElMessageBox.confirm('Delete this code?','Confirm',{type:'warning'})
  await deleteRegCode(id)
  ElMessage.success('Deleted')
  reload()
}
async function batchDelete(){
  await ElMessageBox.confirm('Delete selected codes?','Confirm',{type:'warning'})
  for(const id of selectedIds.value){ await deleteRegCode(id) }
  ElMessage.success('Deleted')
  reload()
}
function exportCsv(){
  const headers = ['id','code','app_id','app_name','code_type','valid_days','total_count','use_count','status','created_at']
  const lines = rows.value.map(r=>[
    r.id,r.code,r.app_id,(r.app_name||''),r.code_type,r.valid_days,(r.total_count??''),r.use_count,r.status,r.created_at
  ].join(','))
  const csv = [headers.join(','), ...lines].join('\n')
  const blob = new Blob([csv],{type:'text/csv;charset=utf-8;'})
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a'); a.href=url; a.download='reg_codes.csv'; a.click(); URL.revokeObjectURL(url)
}

const batch = reactive<BatchCreateRegCodesReq & {visible:boolean}>({ visible:false, app_id:0, quantity:10, code_type:0, valid_days:7, total_count:1 })
function openBatchDialog(){ if((!batch.app_id || batch.app_id===0) && appOptions.value.length){ batch.app_id = appOptions.value[0].id } batch.visible=true }
async function submitBatch(){
  await batchCreateRegCodes(batch)
  batch.visible=false
  ElMessage.success('Created')
  reload()
}

async function copy(text:string){ try{ await navigator.clipboard.writeText(text); ElMessage.success('Copied') }catch{ ElMessage.error('Copy failed') } }

onMounted(async ()=>{ await loadApps(); await reload() })

async function loadApps(){
  const data = await fetchApps({ page:1, page_size:1000 })
  appOptions.value = data.list.map((a:any)=>({ id:a.id, name: a.name }))
  if((!batch.app_id || batch.app_id===0) && appOptions.value.length){ batch.app_id = appOptions.value[0].id }
}
</script>

<style scoped></style>


