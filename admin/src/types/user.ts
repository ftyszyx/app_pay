import type { ListParamsReq } from "./api"

export interface ChangePasswordPayload {
  old_password: string
  new_password: string
}

export interface UserInfo {
  id: number
  username: string
  balance: number
  invite_count: number
  invite_rebate_total: number
  role_id: number
  role_name: string
  created_at: string
}

export type CreateUserReq = { username: string; password: string; role_id?: number }
export type UpdateUserReq = { username?: string; password?: string; role_id?: number; balance?: number }
export type ListUsersParams = {  username?: string; id?: number }&ListParamsReq
