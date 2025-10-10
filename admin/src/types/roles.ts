import type { ListParamsReq } from "./api";

export interface RoleInfo {
  id: number;
  name: string;
  remark?: string | null;
  created_at: string;
}
export type CreateRoleReq = { name: string; remark?: string | null };
export type UpdateRoleReq = { name?: string; remark?: string | null };
export type ListRolesParams = { id?: number; name?: string } & ListParamsReq;
