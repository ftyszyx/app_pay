import request from '@/utils/request'
import type { ApiResponse, PagingResponse } from '@/types'
import type { ResourceModel, ListResourcesParams, AddResourceReq, UpdateResourceReq } from '@/types'

export const fetchResources = async (params: ListResourcesParams = {}) => {
  const response = await request.get('/admin/resources/list', { params }) as ApiResponse<PagingResponse<ResourceModel>>
  return response.data
}

export const createResource = async (payload: AddResourceReq) => {
  const response = await request.post('/admin/resources', payload) as ApiResponse<ResourceModel>
  return response.data
}

export const updateResource = async (id: number, payload: UpdateResourceReq) => {
  const response = await request.put(`/admin/resources/${id}`, payload) as ApiResponse<ResourceModel>
  return response.data
}

export const deleteResource = async (id: number) => {
  const response = await request.delete(`/admin/resources/${id}`) as ApiResponse<void>
  return response.data
}


