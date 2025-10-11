<script setup lang="ts">
import { ref, reactive, onMounted, computed } from "vue";
import { ElMessage, type FormInstance, type FormRules } from "element-plus";
import { useI18n } from "vue-i18n";
import {
  fetchPolicies,
  addPolicy,
  removePolicy,
  fetchRoleLinks,
  addRoleForUser,
  removeRoleForUser,
  checkPermission,
  reloadPolicies,
} from "@/apis/permissions";
import type { PolicyInfo, RoleLinkInfo, UserInfo, RoleInfo } from "@/types";
import { ActionType } from "@/types/permissions";
import { fetchUsers } from "@/apis/users";
import { fetchRoles } from "@/apis/roles";
const { t } = useI18n();

const policies = ref<PolicyInfo[]>([]);
const roleLinks = ref<RoleLinkInfo[]>([]);
const users = ref<UserInfo[]>([]);
const roles = ref<RoleInfo[]>([]);
const policyPage = ref(1); const policyPageSize = ref(10); const displayPolicies = computed(() => { const start = (policyPage.value - 1) * policyPageSize.value; return policies.value.slice(start, start + policyPageSize.value) });
const rolePage = ref(1); const rolePageSize = ref(10); const displayRoleLinks = computed(() => { const start = (rolePage.value - 1) * rolePageSize.value; return roleLinks.value.slice(start, start + rolePageSize.value) });

async function reloadAll() {
  policies.value = (await fetchPolicies());
  roleLinks.value = (await fetchRoleLinks());
  const [usersResp, rolesResp] = await Promise.all([
    fetchUsers({ page: 1, page_size: 1000 }),
    fetchRoles({ page: 1, page_size: 1000 })
  ]);
  users.value = usersResp.list;
  roles.value = rolesResp.list;
}

const policyDialog = reactive({ visible: false, mode: "create" as "create" | "remove" });
const policyFormRef = ref<FormInstance>();
const policyForm = reactive<{ subject: string; object: string; action: ActionType | "" }>({ subject: "", object: "", action: "" });
const policyRules = reactive<FormRules>({ subject: [{ required: true }], object: [{ required: true }], action: [{ required: true }] });
const policyEditingOriginal = ref<PolicyInfo | null>(null);
function toActionType(s: string): ActionType | "" { switch (s) { case ActionType.READ: return ActionType.READ; case ActionType.CREATE: return ActionType.CREATE; case ActionType.UPDATE: return ActionType.UPDATE; case ActionType.DELETE: return ActionType.DELETE; default: return "" } }

async function submitPolicy() {
  const valid = await policyFormRef.value?.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form") as string);
    return;
  }
  const payload = { subject: policyForm.subject, object: policyForm.object, action: policyForm.action as string } as any;
  if (policyDialog.mode === "create") { if (policyEditingOriginal.value) { await removePolicy(policyEditingOriginal.value as any); await addPolicy(payload); policyEditingOriginal.value = null; ElMessage.success(t("common.save") as string) } else { await addPolicy(payload); ElMessage.success(t("common.created") as string) } } else { await removePolicy(payload); ElMessage.success(t("common.deleted") as string) }
  policyDialog.visible = false;
  await reloadAll();
}

const roleDialog = reactive({ visible: false, mode: "create" as "create" | "remove" });
const roleFormRef = ref<FormInstance>();
const roleForm = reactive<{ user_id: number | undefined; role_id: number | undefined }>({ user_id: undefined, role_id: undefined });
const roleRules = reactive<FormRules>({ user_id: [{ required: true }], role_id: [{ required: true }] });

const roleEditingOriginal = ref<RoleLinkInfo | null>(null);

async function submitRoleLink() {
  const valid = await roleFormRef.value?.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form") as string);
    return;
  }
  if (roleDialog.mode === "create") { if (roleEditingOriginal.value) { await removeRoleForUser(roleEditingOriginal.value as any); await addRoleForUser(roleForm as any); roleEditingOriginal.value = null; ElMessage.success(t("common.save") as string) } else { await addRoleForUser(roleForm as any); ElMessage.success(t("common.created") as string) } } else { await removeRoleForUser(roleForm as any); ElMessage.success(t("common.deleted") as string) }
  roleDialog.visible = false;
  await reloadAll();
}
function openEditPolicy(row: PolicyInfo) {
  policyDialog.mode = "create";
  policyEditingOriginal.value = { ...row };
  policyForm.subject = row.subject;
  policyForm.object = row.object;
  policyForm.action = toActionType(row.action); policyDialog.visible = true
}
function removePolicyRow(row: PolicyInfo) {
  removePolicy(row as any).then(() => { ElMessage.success(t("common.deleted") as string); reloadAll() })
}
function openEditRoleLink(row: RoleLinkInfo) {
  roleDialog.mode = "create";
  roleEditingOriginal.value = { ...row };
  const u = users.value.find(u => u.username === row.user);
  const r = roles.value.find(r => r.name === row.role);
  roleForm.user_id = u?.id;
  roleForm.role_id = r?.id;
  roleDialog.visible = true
}
function removeRoleLinkRow(row: RoleLinkInfo) {
  removeRoleForUser({ user_id: row.user_id, role_id: row.role } as any)
    .then(() => { ElMessage.success(t("common.deleted") as string); reloadAll() })
}

const checkDialog = reactive({ visible: false });
const checkFormRef = ref<FormInstance>();
const checkForm = reactive<{ user_id: number | undefined; resource: string; action: string }>({ user_id: undefined, resource: "", action: "" });
const checkRules = reactive<FormRules>({ user_id: [{ required: true }], resource: [{ required: true }], action: [{ required: true }] });
const checkResult = ref<string>("");
async function submitCheck() {
  const valid = await checkFormRef.value?.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form") as string);
    return;
  }
  const ok = await checkPermission({ user_id: checkForm.user_id!, resource: checkForm.resource, action: checkForm.action });
  checkResult.value = ok ? "allowed" : "denied";
}

async function doReloadPolicies() {
  const msg = await reloadPolicies();
  ElMessage.success(msg as any);
  await reloadAll();
}

onMounted(reloadAll);
</script>

<template>
  <div class="space-y-6">
    <el-card shadow="hover">
      <div class="flex items-center gap-2">
        <el-button type="primary" @click="() => { policyDialog.mode = 'create'; policyDialog.visible = true; }">{{
          $t('permissions.policy_add') }}</el-button>
        <el-button type="success" @click="() => { roleDialog.mode = 'create'; roleDialog.visible = true; }">{{
          $t('permissions.add_role_for_user') }}</el-button>
        <el-button type="warning" @click="() => { checkDialog.visible = true; }">{{ $t('permissions.check_permission')
          }}</el-button>
        <el-button type="info" @click="doReloadPolicies">{{ $t('permissions.reload_policies') }}</el-button>
      </div>
    </el-card>

    <el-card shadow="never" :header="$t('permissions.policies')">
      <el-table :data="displayPolicies" stripe size="large" style="width: 100%">
        <el-table-column :label="$t('permissions.subject')" prop="subject" />
        <el-table-column :label="$t('permissions.object')" prop="object" />
        <el-table-column :label="$t('permissions.action')" prop="action" />
        <el-table-column :label="$t('common.actions')" width="200">
          <template #default="{ row }">
            <el-button size="small" @click="openEditPolicy(row)">{{ $t('common.edit') }}</el-button>
            <el-button size="small" type="danger" @click="removePolicyRow(row)">{{ $t('common.delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="flex justify-end mt-4">
        <el-pagination background layout="total, sizes, prev, pager, next, jumper" :page-sizes="[10, 20, 50, 100]"
          :page-size="policyPageSize" :current-page="policyPage" :total="policies.length"
          @current-change="(p: number) => { policyPage = p }" @size-change="(s: number) => { policyPageSize = s; policyPage = 1 }" />
      </div>
    </el-card>

    <el-card shadow="never" :header="$t('permissions.role_links')">
      <el-table :data="displayRoleLinks" stripe size="large" style="width: 100%">
        <el-table-column :label="$t('permissions.user')" prop="user" />
        <el-table-column :label="$t('permissions.role')" prop="role" />
        <el-table-column :label="$t('common.actions')" width="200">
          <template #default="{ row }">
            <el-button size="small" @click="openEditRoleLink(row)">{{ $t('common.edit') }}</el-button>
            <el-button size="small" type="danger" @click="removeRoleLinkRow(row)">{{ $t('common.delete') }}</el-button>
          </template>
        </el-table-column>
      </el-table>
      <div class="flex justify-end mt-4">
        <el-pagination background layout="total, sizes, prev, pager, next, jumper" :page-sizes="[10, 20, 50, 100]"
          :page-size="rolePageSize" :current-page="rolePage" :total="roleLinks.length"
          @current-change="(p: number) => { rolePage = p }" @size-change="(s: number) => { rolePageSize = s; rolePage = 1 }" />
      </div>
    </el-card>

    <el-dialog v-model="policyDialog.visible"
      :title="policyDialog.mode === 'create' ? $t('permissions.add_policy') : $t('permissions.remove_policy')"
      width="560px">
      <el-form ref="policyFormRef" :model="policyForm" :rules="policyRules" label-width="120px">
        <el-form-item :label="$t('permissions.subject')" prop="subject">
          <el-select v-model="policyForm.subject" filterable class="w-full">
            <el-option v-for="r in roles" :key="r.id" :label="r.name" :value="r.name" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('permissions.object')" prop="object"><el-input
            v-model="policyForm.object" /></el-form-item>
        <el-form-item :label="$t('permissions.action')" prop="action">
          <el-select v-model="policyForm.action" class="w-full">
            <el-option v-for="action in Object.values(ActionType)" :key="action" :label="action" :value="action" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="policyDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitPolicy">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="roleDialog.visible"
      :title="roleDialog.mode === 'create' ? $t('permissions.add_role_for_user') : $t('permissions.remove_role_for_user')"
      width="520px">
      <el-form ref="roleFormRef" :model="roleForm" :rules="roleRules" label-width="140px">
        <el-form-item :label="$t('permissions.user')" prop="user">
          <el-select v-model="roleForm.user_id" filterable class="w-full">
            <el-option v-for="u in users" :key="u.id" :label="u.username" :value="u.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('permissions.role')" prop="role">
          <el-select v-model="roleForm.role_id" filterable class="w-full">
            <el-option v-for="r in roles" :key="r.id" :label="r.name" :value="r.id" />
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="roleDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitRoleLink">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="checkDialog.visible" :title="$t('permissions.check_permission')" width="520px">
      <el-form ref="checkFormRef" :model="checkForm" :rules="checkRules" label-width="140px">
        <el-form-item :label="$t('permissions.user_id')" prop="user_id">
          <el-select v-model="checkForm.user_id" filterable class="w-full">
            <el-option v-for="u in users" :key="u.id" :label="u.username" :value="u.id" />
          </el-select>
        </el-form-item>
        <el-form-item :label="$t('permissions.resource')" prop="resource"><el-input
            v-model="checkForm.resource" /></el-form-item>
        <el-form-item :label="$t('permissions.action')" prop="action">
          <el-select v-model="checkForm.action" class="w-full">
            <el-option v-for="action in Object.values(ActionType)" :key="action" :label="action" :value="action" />
          </el-select>
        </el-form-item>
      </el-form>
      <div v-if="checkResult" class="mt-2 text-sm text-gray-600">{{ $t('permissions.result') }}: {{ checkResult }}</div>
      <template #footer>
        <el-button @click="checkDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitCheck">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped></style>
