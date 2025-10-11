import request from "@/utils/request";
import type {  PagingResponse } from "@/types";
import type { ListRolesParams, RoleInfo, CreateRoleReq, UpdateRoleReq } from "@/types";

export async function fetchRoles(params: ListRolesParams): Promise<PagingResponse<RoleInfo>> {
  return (await request.get("/admin/roles/list", { params })).data;
}
export async function createRole(payload: CreateRoleReq): Promise<RoleInfo> {
  return (await request.post("/admin/roles", payload)).data;
}
export async function updateRole(id: number, payload: UpdateRoleReq): Promise<RoleInfo> {
  return (await request.put(`/admin/roles/${id}`, payload)).data;
}
export async function deleteRole(id: number): Promise<void> {
  return (await request.delete(`/admin/roles/${id}`)).data;
}
