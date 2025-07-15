import axios, { type AxiosResponse, type InternalAxiosRequestConfig } from 'axios'
import { consoleError, consoleLog } from './log'
import { ElMessage } from 'element-plus'

console.log("request",import.meta.env.VITE_BASE_URL||'/api')
const instance = axios.create({
  baseURL: import.meta.env.VITE_BASE_URL||'/api',
})

instance.interceptors.request.use(
  (config: InternalAxiosRequestConfig) => {
    consoleLog(`[request] ${config.method} ${config.baseURL}${config.url} \nData: ${JSON.stringify(config.data)} \nParams: ${JSON.stringify(config.params)}`)
    const token = localStorage.getItem('token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    return config
  },
  (error: any) => {
    consoleError(`[req error] ${error.config?.method} ${error.config?.baseURL}${error.config?.url}`)
    ElMessage.error(`${error.code}:${error.message}`)
    return Promise.reject(error)
  }
)

instance.interceptors.response.use(
  (response: AxiosResponse) => {
        consoleLog(`[response] ${response.config.method} ${response.config.baseURL}${response.config.url} \nData: ${JSON.stringify(response.data)}`)
        const data = response.data.data
    if(data.code==0){
      return data
    }else{
      const message = data.message
      ElMessage.error(message)
      consoleError(message)
      return Promise.reject(message)
    }

  },
  (error: any) => {
    consoleError(`[error] ${error.config?.method} ${error.config?.baseURL}${error.config?.url}`)
    ElMessage.error(`${error.code}:${error.message}`)
    return Promise.reject(error)
  }
)

export default instance 