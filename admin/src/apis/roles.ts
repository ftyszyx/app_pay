import request from "@/utils/request";
import type { PagingResponse } from "@/types";
import type { ListRolesParams, RoleInfo, CreateRoleReq, UpdateRoleReq } from "@/types";

export async function fetchRoles(params: ListRolesParams): Promise<PagingResponse<RoleInfo>> {
  return request.get("/admin/roles/list", { params });
}
export async function createRole(payload: CreateRoleReq): Promise<RoleInfo> {
  return request.post("/admin/roles", payload);
}
export async function updateRole(id: number, payload: UpdateRoleReq): Promise<RoleInfo> {
  return request.put(`/admin/roles/${id}`, payload);
}
export async function deleteRole(id: number): Promise<void> {
  return request.delete(`/admin/roles/${id}`);
}
