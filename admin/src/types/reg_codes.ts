import type { ListParamsReq } from './api'

export interface RegCodeModel {
  id: number
  code: string
  app_id: number
  bind_device_info?: any | null
  valid_days: number
  max_devices: number
  status: number
  binding_time?: string | null
  code_type: number
  expire_time?: string | null
  total_count?: number | null
  use_count: number
  device_id?: string | null
  created_at: string
  updated_at: string
  app_name?: string | null
}

export type ListRegCodesParams = {
  id?: number
  code?: string
  app_id?: number
  status?: number
  code_type?: number
} & ListParamsReq

export interface BatchCreateRegCodesReq {
  app_id: number
  quantity: number
  code_type: number // 0 time, 1 count
  valid_days?: number | null
  total_count?: number | null
}

