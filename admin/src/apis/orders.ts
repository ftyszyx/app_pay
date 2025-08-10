import request from '@/utils/request'
import type { ApiResponse, PagingResponse } from '@/types'
import type { OrderInfo, ListOrdersParams } from '@/types/orders'

export const fetchOrders = async (params: ListOrdersParams = {}) => {
  const res = await request.get('/admin/orders/list', { params }) as ApiResponse<PagingResponse<OrderInfo>>
  return res.data
}


