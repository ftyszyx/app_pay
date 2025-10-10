<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { ElMessage,  type FormInstance, type FormRules } from "element-plus";
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
import type { PolicyInfo, RoleLinkInfo } from "@/types";
const { t } = useI18n();

const policies = ref<PolicyInfo[]>([]);
const roleLinks = ref<RoleLinkInfo[]>([]);

async function reloadAll() {
  policies.value = (await fetchPolicies()).data ;
  roleLinks.value = (await fetchRoleLinks()).data;
}

const policyDialog = reactive({ visible: false, mode: "create" as "create" | "remove" });
const policyFormRef = ref<FormInstance>();
const policyForm = reactive<{ subject: string; object: string; action: string }>({ subject: "", object: "", action: "" });
const policyRules = reactive<FormRules>({ subject: [{ required: true }], object: [{ required: true }], action: [{ required: true }] });

async function submitPolicy() {
  const valid = await policyFormRef.value?.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form") as string);
    return;
  }
  if (policyDialog.mode === "create") {
    await addPolicy(policyForm);
    ElMessage.success(t("common.created") as string);
  } else {
    await removePolicy(policyForm as any);
    ElMessage.success(t("common.deleted") as string);
  }
  policyDialog.visible = false;
  await reloadAll();
}

const roleDialog = reactive({ visible: false, mode: "create" as "create" | "remove" });
const roleFormRef = ref<FormInstance>();
const roleForm = reactive<{ user: string; role: string }>({ user: "", role: "" });
const roleRules = reactive<FormRules>({ user: [{ required: true }], role: [{ required: true }] });

async function submitRoleLink() {
  const valid = await roleFormRef.value?.validate();
  if (!valid) {
    ElMessage.error(t("common.please_check_form") as string);
    return;
  }
  if (roleDialog.mode === "create") {
    await addRoleForUser(roleForm);
    ElMessage.success(t("common.created") as string);
  } else {
    await removeRoleForUser(roleForm as any);
    ElMessage.success(t("common.deleted") as string);
  }
  roleDialog.visible = false;
  await reloadAll();
}

const checkDialog = reactive({ visible: false });
const checkFormRef = ref<FormInstance>();
const checkForm = reactive<{ user_id: number | null; resource: string; action: string }>({ user_id: null, resource: "", action: "" });
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
        <el-button
          type="primary"
          @click="
            () => {
              policyDialog.mode = 'create';
              policyDialog.visible = true;
            }
          "
          >Add Policy</el-button
        >
        <el-button
          @click="
            () => {
              policyDialog.mode = 'remove';
              policyDialog.visible = true;
            }
          "
          >Remove Policy</el-button
        >
        <el-button
          type="success"
          @click="
            () => {
              roleDialog.mode = 'create';
              roleDialog.visible = true;
            }
          "
          >Add Role For User</el-button
        >
        <el-button
          @click="
            () => {
              roleDialog.mode = 'remove';
              roleDialog.visible = true;
            }
          "
          >Remove Role For User</el-button
        >
        <el-button
          type="warning"
          @click="
            () => {
              checkDialog.visible = true;
            }
          "
          >Check Permission</el-button
        >
        <el-button type="info" @click="doReloadPolicies">Reload Policies</el-button>
      </div>
    </el-card>

    <el-card shadow="never" header="Policies">
      <el-table :data="policies" stripe size="large" style="width: 100%">
        <el-table-column label="Subject" prop="subject" />
        <el-table-column label="Object" prop="object" />
        <el-table-column label="Action" prop="action" />
      </el-table>
    </el-card>

    <el-card shadow="never" header="Role Links">
      <el-table :data="roleLinks" stripe size="large" style="width: 100%">
        <el-table-column label="User" prop="user" />
        <el-table-column label="Role" prop="role" />
      </el-table>
    </el-card>

    <el-dialog v-model="policyDialog.visible" :title="policyDialog.mode === 'create' ? 'Add Policy' : 'Remove Policy'" width="560px">
      <el-form ref="policyFormRef" :model="policyForm" :rules="policyRules" label-width="120px">
        <el-form-item label="Subject" prop="subject"><el-input v-model="policyForm.subject" /></el-form-item>
        <el-form-item label="Object" prop="object"><el-input v-model="policyForm.object" /></el-form-item>
        <el-form-item label="Action" prop="action"><el-input v-model="policyForm.action" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="policyDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitPolicy">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="roleDialog.visible" :title="roleDialog.mode === 'create' ? 'Add Role For User' : 'Remove Role For User'" width="520px">
      <el-form ref="roleFormRef" :model="roleForm" :rules="roleRules" label-width="140px">
        <el-form-item label="User" prop="user"><el-input v-model="roleForm.user" /></el-form-item>
        <el-form-item label="Role" prop="role"><el-input v-model="roleForm.role" /></el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="roleDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitRoleLink">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>

    <el-dialog v-model="checkDialog.visible" title="Check Permission" width="520px">
      <el-form ref="checkFormRef" :model="checkForm" :rules="checkRules" label-width="140px">
        <el-form-item label="User ID" prop="user_id"><el-input-number v-model="checkForm.user_id" :min="1" /></el-form-item>
        <el-form-item label="Resource" prop="resource"><el-input v-model="checkForm.resource" /></el-form-item>
        <el-form-item label="Action" prop="action"><el-input v-model="checkForm.action" /></el-form-item>
      </el-form>
      <div v-if="checkResult" class="mt-2 text-sm text-gray-600">Result: {{ checkResult }}</div>
      <template #footer>
        <el-button @click="checkDialog.visible = false">{{ $t("common.cancel") }}</el-button>
        <el-button type="primary" @click="submitCheck">{{ $t("common.confirm") }}</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<style scoped></style>
