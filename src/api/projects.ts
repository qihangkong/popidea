import { ApiClient } from './client'
import type { Project } from '../types'

export class ProjectsApi {
  static async getProjects(): Promise<Project[]> {
    return ApiClient.invoke<Project[]>('get_projects')
  }

  static async createProject(name: string, description?: string): Promise<Project> {
    return ApiClient.invoke<Project>('create_project', { name, description })
  }

  static async getProject(id: string): Promise<Project | null> {
    return ApiClient.invoke<Project | null>('get_project', { id })
  }

  static async updateProject(
    id: string,
    name?: string,
    description?: string
  ): Promise<Project | null> {
    return ApiClient.invoke<Project | null>('update_project', { id, name, description })
  }

  static async deleteProject(id: string): Promise<boolean> {
    return ApiClient.invoke<boolean>('delete_project', { id })
  }
}
