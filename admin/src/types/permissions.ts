import type { ListParamsReq } from "./api";
export interface PolicyInfo {
  subject: string;
  object: string;
  action: string;
}
export type AddPolicyReq = { subject: string; object: string; action: string };
export type RemovePolicyReq = { subject: string; object: string; action: string };
export interface RoleLinkInfo {
  user_id: string;
  user: string;
  role: string;
}
export type AddRoleReq = { user_id: number; role_id: number };
export type RemoveRoleReq = { user_id: number; role_id: number };
export type PermissionCheckReq = { user_id: number; resource: string; action: string };
export type ListPoliciesParams = {} & ListParamsReq;

export enum  ActionType {
  READ = "read",
  CREATE = "create",
  UPDATE = "update",
  DELETE = "delete",
}