

import request from '@/utils/request'
import type {  PagingResponse } from '@/types'
import type { ListUsersParams, UserInfo, CreateUserReq, UpdateUserReq } from '@/types/user'

export async function fetchUsers(params: ListUsersParams): Promise<PagingResponse<UserInfo>> {
  return request.get('/admin/users/list', { params })
}

export async function createUser(payload: CreateUserReq): Promise<UserInfo> {
  return request.post('/admin/users', payload)
}

export async function updateUser(id: number, payload: UpdateUserReq): Promise<UserInfo> {
  return request.put(`/admin/users/${id}`, payload)
}

export async function deleteUser(id: number): Promise<void> {
  return request.delete(`/admin/users/${id}`)
}
