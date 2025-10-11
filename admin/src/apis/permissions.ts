import request from "@/utils/request";
import type { PolicyInfo, AddPolicyReq, RemovePolicyReq, RoleLinkInfo, AddRoleReq, RemoveRoleReq, PermissionCheckReq } from "@/types";

export async function fetchPolicies(): Promise<PolicyInfo[]> {
  return (await request.get("/admin/permissions/policies")).data;
}
export async function addPolicy(payload: AddPolicyReq): Promise<boolean> {
  return (await request.post("/admin/permissions/policies", payload)).data;
}
export async function removePolicy(payload: RemovePolicyReq): Promise<boolean> {
  return (await request.delete("/admin/permissions/policies", { data: payload } as any)).data;
}

export async function fetchRoleLinks(): Promise<RoleLinkInfo[]> {
  return (await request.get("/admin/permissions/roles")).data;
}
export async function addRoleForUser(payload: AddRoleReq): Promise<boolean> {
  return (await request.post("/admin/permissions/roles", payload)).data;
}
export async function removeRoleForUser(payload: RemoveRoleReq): Promise<boolean> {
  return (await request.delete("/admin/permissions/roles", { data: payload } as any)).data;
}

export async function checkPermission(payload: PermissionCheckReq): Promise<boolean> {
  return (await request.post("/admin/permissions/check", payload)).data;
}
export async function reloadPolicies(): Promise<string> {
  return (await request.post("/admin/permissions/reload", {})).data;
}
