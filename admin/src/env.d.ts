/// <reference types="vue-i18n" />
export {}

interface ImportMetaEnv {
  readonly VITE_OSS_REGION: string
  readonly VITE_OSS_BUCKET: string
  readonly VITE_OSS_ENDPOINT?: string
  // If using permanent AK (not recommended), or fetch STS from backend and fill at runtime
  readonly VITE_OSS_ACCESS_KEY_ID?: string
  readonly VITE_OSS_ACCESS_KEY_SECRET?: string
}
interface ImportMeta {
  readonly env: ImportMetaEnv
}


