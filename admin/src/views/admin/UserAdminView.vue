<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { ElMessage, ElMessageBox, type FormInstance, type FormRules } from 'element-plus'
import { formatTime } from '@/utils'
import { fetchUsers, createUser, updateUser, deleteUser } from '@/apis/users'
import { useI18n } from 'vue-i18n'
import type { UserInfo } from '@/types/user'
const rows = ref<UserInfo[]>([])
const selectedIds = ref<number[]>([])
const page = ref(1)
const pageSize = ref(20)
const total = ref(0)
const { t } = useI18n()

const query = reactive({ username: '' as string | undefined })

async function reload() { const data = await fetchUsers({ page: page.value, page_size: pageSize.value, username: query.username || undefined }); rows.value = data.list; total.value = data.total }
function resetFilters() { query.username = ''; page.value = 1; reload() }
function onSelChange(arr: UserInfo[]) { selectedIds.value = arr.map(it => it.id) }
function handlePageChange(p: number) { page.value = p; reload() }
function handleSizeChange(s: number) { pageSize.value = s; page.value = 1; reload() }

const dialog = reactive({ visible: false, mode: 'create' as 'create' | 'edit', editingId: undefined as number | undefined })
const formRef = ref<FormInstance>()
const form = reactive<{ username: string; password?: string }>({ username: '', password: '' })
const rules = reactive<FormRules>({ username: [{ required: true, message: 'Username required' }] })

function openCreate() { dialog.mode = 'create'; dialog.editingId = undefined; form.username = ''; form.password = ''; dialog.visible = true }
function openEdit(row: UserInfo) { dialog.mode = 'edit'; dialog.editingId = row.id; form.username = row.username; form.password = ''; dialog.visible = true }

async function submit() {
  const valid = await formRef.value?.validate(); if (!valid) { ElMessage.error(t('common.please_check_form') as string); return }
  if (dialog.mode === 'create') { await createUser({ username: form.username, password: form.password || '' }); ElMessage.success(t('common.created') as string) }
  else if (dialog.editingId != null) { await updateUser(dialog.editingId, { username: form.username }); ElMessage.success(t('common.save') as string) }
  dialog.visible = false; await reload()
}

async function del(id: number) { await ElMessageBox.confirm(t('common.delete_confirm', { name: rows.value.find(it => it.id === id)?.username || '' }), t('common.confirm'), { type: 'warning' }); await deleteUser(id); ElMessage.success(t('common.deleted') as string); reload() }

onMounted(reload)
</script>

<template>
  <div class="space-y-4">
    <el-card shadow="hover">
      <div class="flex items-center gap-2">
        <el-input v-model="query.username" :placeholder="$t('common.search_by_name')" clearable class="w-64" />
        <el-button type="primary" @click="reload">{{ $t('common.search') }}</el-button>
        <el-button @click="resetFilters">{{ $t('common.reset') }}</el-button>
        <el-button type="success" @click="openCreate">{{ $t('common.new') }}</el-button>
      </div>
    </el-card>

    <el-card shadow="never">
      <el-table :data="rows" stripe size="large" style="width: 100%" @selection-change="onSelChange">
        <el-table-column type="selection" width="50" />
        <el-table-column prop="id" label="ID" width="80" />
        <el-table-column :label="$t('auth.username')" min-width="160">
          <template #default="{ row }">
            <div class="flex items-center gap-2">
              <span class="text-gray-800 break-all">{{ row.username }}</span>
            </div>
          </template>
        </el-table-column>
        <el-table-column :label="$t('orders.created')" min-width="180">
          <template #default="{ row }">{{ formatTime(row.created_at) }}</template>
        </el-table-column>
        <el-table-column :label="$t('common.actions')" width="200" fixed="right">
          <template #default="{ row }">
            <el-button size="small" @click="openEdit(row)">{{ $t('common.edit') }}</el-button>
            <el-button size="small" type="danger" @click="del(row.id)">{{ $t('common.delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="flex justify-end mt-4">
        <el-pagination background layout="total, sizes, prev, pager, next, jumper" :page-sizes="[10, 20, 50, 100]"
          :page-size="pageSize" :current-page="page" :total="total" @current-change="handlePageChange"
          @size-change="handleSizeChange" />
      </div>
    </el-card>

    <el-dialog v-model="dialog.visible" :title="dialog.mode === 'create' ? $t('common.create') : $t('common.edit')"
      width="520px">
      <el-form label-width="140px" ref="formRef" :model="form" :rules="rules">
        <el-form-item :label="$t('auth.username')" prop="username"><el-input v-model="form.username" /></el-form-item>
        <el-form-item v-if="dialog.mode === 'create'" :label="$t('auth.password')" prop="password"><el-input
            v-model="form.password" type="password" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="dialog.visible = false">{{ $t('common.cancel') }}</el-button>
        <el-button type="primary" @click="submit">{{ $t('common.confirm') }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped></style>