import request from "@/utils/request";
import type { PolicyInfo, AddPolicyReq, RemovePolicyReq, RoleLinkInfo, AddRoleReq, RemoveRoleReq, PermissionCheckReq, ApiResponse } from "@/types";

export async function fetchPolicies(): Promise<ApiResponse<PolicyInfo[]>> {
  return request.get("/admin/permissions/policies");
}
export async function addPolicy(payload: AddPolicyReq): Promise<boolean> {
  return request.post("/admin/permissions/policies", payload);
}
export async function removePolicy(payload: RemovePolicyReq): Promise<boolean> {
  return request.delete("/admin/permissions/policies", { data: payload } as any);
}

export async function fetchRoleLinks(): Promise<ApiResponse<RoleLinkInfo[]>> {
  return request.get("/admin/permissions/roles");
}
export async function addRoleForUser(payload: AddRoleReq): Promise<boolean> {
  return request.post("/admin/permissions/roles", payload);
}
export async function removeRoleForUser(payload: RemoveRoleReq): Promise<boolean> {
  return request.delete("/admin/permissions/roles", { data: payload } as any);
}

export async function checkPermission(payload: PermissionCheckReq): Promise<boolean> {
  return request.post("/admin/permissions/check", payload);
}
export async function reloadPolicies(): Promise<string> {
  return request.post("/admin/permissions/reload", {});
}
