import { createI18n } from 'vue-i18n'
import enUS from '@/locales/en.json'
import zhCN from '@/locales/zh-cn.json'

export const i18n = createI18n({ legacy: false, locale: (localStorage.getItem('locale') as 'en' | 'zh-cn') || 'en', fallbackLocale: 'en', messages: { en: enUS, 'zh-cn': zhCN } })


