import request from '@/utils/request'
import type { ApiResponse, PagingResponse } from '@/types'
import type { ImageModel, ListImagesParams, AddImageReq, UpdateImageReq } from '@/types/images'

export const fetchImages = async (params: ListImagesParams = {}) => {
  const response = await request.get('/admin/images/list', { params }) as ApiResponse<PagingResponse<ImageModel>>
  return response.data
}

export const createImage = async (payload: AddImageReq) => {
  const response = await request.post('/admin/images', payload) as ApiResponse<ImageModel>
  return response.data
}

export const updateImage = async (id: number, payload: UpdateImageReq) => {
  const response = await request.put(`/admin/images/${id}`, payload) as ApiResponse<ImageModel>
  return response.data
}

export const deleteImage = async (id: number) => {
  const response = await request.delete(`/admin/images/${id}`) as ApiResponse<void>
  return response.data
}


