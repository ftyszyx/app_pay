export interface ApiResponse<T> {
  code: number;
  message: string;
  success: boolean;
  data: T;
}

export interface PagingResponse<T> {
  list: T[]
  page: number
  total: number
}

export interface ListParamsReq {
  page?: number
  page_size?: number
}