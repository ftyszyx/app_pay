import type { ListParamsReq } from "./api";

export interface ProductModel {
  id: number;
  name: string;
  price: number;
  app_id: number;
  product_id: string;
  add_valid_days: number;
  image_url?: string | null;
  tags?: string[] ;
  status: number;
  created_at?: string | null;
  updated_at?: string | null;
  remark?: string | null;
  deleted_at?: string | null;
}

export type ListProductsParams = { id?: number; product_id?: string; name?: string; status?: number } & ListParamsReq;

export interface AddProductReq {
  name: string;
  price: number;
  app_id: number;
  product_id: string;
  add_valid_days: number;
  image_url?: string | null;
  tags?: string[] ;
  status: number;
  remark?: string | null;
}

export interface UpdateProductReq {
  name?: string;
  price?: number;
  app_id?: number;
  product_id?: string;
  add_valid_days?: number;
  image_url?: string | null;
  tags?: string[] ;
  status?: number;
  remark?: string | null;
}
