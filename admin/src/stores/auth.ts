import { defineStore } from 'pinia'
import type { AuthPayload } from '@/types'
import { ref } from 'vue'
import { sentLogin } from '@/apis'

export const useAuthStore = defineStore('auth', () => {
  const token = ref(localStorage.getItem('token'))
  const isAuthenticated = ref(!!token.value)

  function setToken(newToken: string) {
    token.value = newToken
    isAuthenticated.value = true
    localStorage.setItem('token', newToken)
  }

  function clearToken() {
    token.value = null
    isAuthenticated.value = false
    localStorage.removeItem('token')
  }

  async function login(payload: AuthPayload) {
    const response = await sentLogin(payload)
    if(!response) throw new Error('Login failed: no token received')
    setToken(response.token)
  }

  function logout() {
    clearToken()
  }

  return { token, isAuthenticated, login, logout }
}) 