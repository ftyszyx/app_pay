import type { ListParamsReq } from "./api";
export interface PolicyInfo {
  subject: string;
  object: string;
  action: string;
}
export type AddPolicyReq = { subject: string; object: string; action: string };
export type RemovePolicyReq = { subject: string; object: string; action: string };
export interface RoleLinkInfo {
  user: string;
  role: string;
}
export type AddRoleReq = { user: string; role: string };
export type RemoveRoleReq = { user: string; role: string };
export type PermissionCheckReq = { user_id: number; resource: string; action: string };
export type ListPoliciesParams = {} & ListParamsReq;
