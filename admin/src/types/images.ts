import type { ListParamsReq } from "./api";

export interface ImageModel {
  id: number;
  name: string;
  object_key: string;
  url: string;
  path: string;
  tags?: string[] | null;
  status: number;
  remark?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  deleted_at?: string | null;
}

export type ListImagesParams = { id?: number; name?: string; object_key?: string; url?: string; path?: string; status?: number } & ListParamsReq;

export interface AddImageReq {
  name: string;
  object_key: string;
  url: string;
  path: string;
  tags?: string[] | null;
  status: number;
  remark?: string | null;
}

export interface UpdateImageReq {
  name?: string;
  object_key?: string;
  url?: string;
  path?: string;
  tags?: string[] | null;
  status?: number;
  remark?: string | null;
}


