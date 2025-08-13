import { defineStore } from 'pinia'
import request from '@/utils/request'
import { createOssClient } from '@/utils/oss'

type StsCredentialsResp = { access_key_id: string; access_key_secret: string; security_token: string; expiration: string }

export const useOssStore = defineStore('oss', () => {
  let cachedClient: any = null as any
  let creds: StsCredentialsResp | null = null

  function isExpiredSoon(expiration?: string | null) {
    if (!expiration) return true
    const expireAt = new Date(expiration).getTime()
    const now = Date.now()
    return expireAt - now < 60 * 1000
  }

  async function fetchSts(): Promise<StsCredentialsResp> {
    const resp = await request.get('/admin/storage/oss/sts') as any
    return resp.data as StsCredentialsResp
  }

  async function getClient() {
    if (!creds || isExpiredSoon(creds.expiration)) {
      creds = await fetchSts()
      cachedClient = null
    }
    if (!cachedClient) {
      cachedClient = createOssClient({
        region: import.meta.env.VITE_OSS_REGION,
        bucket: import.meta.env.VITE_OSS_BUCKET,
        accessKeyId: creds!.access_key_id,
        accessKeySecret: creds!.access_key_secret,
        stsToken: creds!.security_token,
        secure: true,
        endpoint: import.meta.env.VITE_OSS_ENDPOINT,
      }) as any
    }
    return cachedClient
  }

  return { getClient }
})


