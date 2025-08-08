import { defineStore } from 'pinia'
import { ref } from 'vue'
import zhCn from 'element-plus/dist/locale/zh-cn.mjs'
import en from 'element-plus/dist/locale/en.mjs'

export type SupportedLocale = 'en' | 'zh-cn'

export const useLocaleStore = defineStore('locale', () => {
    const current = ref<SupportedLocale>((localStorage.getItem('locale') as SupportedLocale) || 'en')

    const setLocale = (loc: SupportedLocale) => {
        current.value = loc
        localStorage.setItem('locale', loc)
    }

    const elLocale = () => (current.value === 'zh-cn' ? zhCn : en)

    return { current, setLocale, elLocale }
})


