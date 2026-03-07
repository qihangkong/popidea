import { invoke } from '@tauri-apps/api/core'

export interface Project {
  id: string
  name: string
  description?: string
  created_at: string
  updated_at: string
  settings?: any
}

export interface Episode {
  id: string
  project_id: string
  name: string
  content?: string
  created_at: string
  updated_at: string
}

export const api = {
  projects: {
    list: async (): Promise<Project[]> => {
      return await invoke<Project[]>('list_projects')
    },
    create: async (name: string, description?: string): Promise<Project> => {
      return await invoke<Project>('create_project', { name, description })
    },
    get: async (id: string): Promise<Project> => {
      return await invoke<Project>('get_project', { id })
    },
    update: async (id: string, name?: string, description?: string): Promise<Project> => {
      return await invoke<Project>('update_project', { id, name, description })
    },
    delete: async (id: string): Promise<void> => {
      return await invoke('delete_project', { id })
    }
  },
  episodes: {
    list: async (projectId: string): Promise<Episode[]> => {
      return await invoke<Episode[]>('list_episodes', { projectId })
    },
    create: async (projectId: string, name: string, content?: string): Promise<Episode> => {
      return await invoke<Episode>('create_episode', { projectId, name, content })
    },
    get: async (id: string): Promise<Episode> => {
      return await invoke<Episode>('get_episode', { id })
    },
    update: async (id: string, name?: string, content?: string): Promise<Episode> => {
      return await invoke<Episode>('update_episode', { id, name, content })
    },
    delete: async (id: string): Promise<void> => {
      return await invoke('delete_episode', { id })
    }
  }
}
