import request from '@/utils/request'
import type { ApiResponse, PagingResponse } from '@/types'
import type { ProductModel, ListProductsParams, AddProductReq, UpdateProductReq } from '@/types/products'

export const fetchProducts = async (params: ListProductsParams = {}) => {
  const response = await request.get('/admin/products/list', { params }) as ApiResponse<PagingResponse<ProductModel>>
  return response.data
}

export const createProduct = async (payload: AddProductReq) => {
  const response = await request.post('/admin/products', payload) as ApiResponse<ProductModel>
  return response.data
}

export const updateProduct = async (id: number, payload: UpdateProductReq) => {
  const response = await request.put(`/admin/products/${id}`, payload) as ApiResponse<ProductModel>
  return response.data
}

export const deleteProduct = async (id: number) => {
  const response = await request.delete(`/admin/products/${id}`) as ApiResponse<void>
  return response.data
}


