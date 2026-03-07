import { invoke } from '@tauri-apps/api/core'

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
}

export class ApiClient {
  static async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    try {
      const result = await invoke<T>(command, args)
      return result
    } catch (error) {
      console.error(`API Error [${command}]:`, error)
      throw error
    }
  }
}
