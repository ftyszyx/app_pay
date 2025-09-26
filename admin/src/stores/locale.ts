import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
import en from 'element-plus/es/locale/lang/en'
import { i18n } from '@/utils/i18n'

export type SupportedLocale = 'en' | 'zh-cn'

export const useLocaleStore = defineStore('locale', () => {
    const current = ref<SupportedLocale>((localStorage.getItem('locale') as SupportedLocale) || 'zh-cn')

    const setLocale = (loc: SupportedLocale) => { current.value = loc }

    watch(current, (loc) => {
        localStorage.setItem('locale', loc)
        if (i18n.global && 'locale' in i18n.global) { ;(i18n.global as any).locale.value = loc }
    }, { immediate: true })

    const elLocale = () => (current.value === 'zh-cn' ? zhCn : en)

    return { current, setLocale, elLocale }
})


