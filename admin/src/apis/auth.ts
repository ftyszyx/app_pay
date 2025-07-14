import request from "@/utils/request"
import type { ApiResponse, AuthPayload, AuthResponse, RegisterPayload } from "@/types"


export const sentLogin = async (payload: AuthPayload)=> {
    const response = await request.post('/login', payload) as ApiResponse<AuthResponse>
    return response.data
}

export const sentRegister = async (payload: RegisterPayload) => {
    const response = await request.post('/register', payload) as ApiResponse<AuthResponse>
    return response.data
}

export const sentLogout = async () => {
    const response = await request.post('/logout') as ApiResponse<AuthResponse>
    return response.data
}

export const sentGetUserInfo = async () => {
    const response = await request.get('/user/info') as ApiResponse<AuthResponse>
    return response.data
}