import type { ListParamsReq } from "./api";

export interface ResourceModel {
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
}

export type ListResourcesParams = { id?: number; name?: string; object_key?: string; url?: string; path?: string; status?: number } & ListParamsReq;

export interface AddResourceReq {
  name: string;
  object_key: string;
  url: string;
  path: string;
  tags?: string[] | null;
  status: number;
  remark?: string | null;
}

export interface UpdateResourceReq {
  name?: string;
  object_key?: string;
  url?: string;
  path?: string;
  tags?: string[] | null;
  status?: number;
  remark?: string | null;
}


