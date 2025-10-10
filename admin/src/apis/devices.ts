import request from '@/utils/request'
import type { PagingResponse } from '@/types'
import type { ListDevicesParams, DeviceInfo } from '@/types/app_devices'


export async function fetchDevices(params: ListDevicesParams): Promise<PagingResponse<DeviceInfo>> {
  return request.get('/admin/devices/list', { params })
}
