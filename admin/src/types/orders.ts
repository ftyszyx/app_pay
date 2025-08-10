import type { ListParamsReq } from './api'

export interface OrderInfo {
  id: number
  order_id: string
  user_info?: Record<string, any> | null
  status: number
  pay_method_id: number
  original_price: number
  final_price: number
  remark?: string | null
  created_at: string
  updated_at: string
  created_by: number
  updated_by: number
  pay_method_name?: string | null
  created_by_username?: string | null
  updated_by_username?: string | null
}

export type ListOrdersParams = { order_id?: string; status?: number } & ListParamsReq


