import type { Component } from 'vue'

export interface AdminMenuItem {
    label: string
    icon?: string | Component
    path?: string
    index?: string
    disabled?: boolean
    children?: AdminMenuItem[]
}

export type AdminMenuMode = 'vertical' | 'horizontal'

