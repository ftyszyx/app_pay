import OSS from 'ali-oss'

export type OssConfig = {
  region: string
  bucket: string
  accessKeyId: string
  accessKeySecret: string
  stsToken?: string
  secure?: boolean
  cname?: boolean
  endpoint?: string
}

export function createOssClient(cfg: OssConfig) {
  return new OSS({
    region: cfg.region,
    bucket: cfg.bucket,
    accessKeyId: cfg.accessKeyId,
    accessKeySecret: cfg.accessKeySecret,
    stsToken: cfg.stsToken,
    secure: cfg.secure ?? true,
    cname: cfg.cname,
    endpoint: cfg.endpoint,
  })
}

export async function uploadFile(client: OSS, file: File, key: string) {
  const result = await client.put(key, file)
  return result.url as string
}

export function genObjectKey(prefix: string, file: File) {
  const ts = Date.now()
  const ext = file.name.includes('.') ? file.name.substring(file.name.lastIndexOf('.')) : ''
  return `${prefix.replace(/\/$/, '')}/${ts}-${Math.random().toString(36).slice(2)}${ext}`
}


