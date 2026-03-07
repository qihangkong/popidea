import { ApiClient } from './client'
import type { Episode } from '../types'

export class EpisodesApi {
  static async getEpisodes(projectId: string): Promise<Episode[]> {
    return ApiClient.invoke<Episode[]>('get_episodes', { projectId })
  }

  static async createEpisode(
    projectId: string,
    name: string,
    content?: string
  ): Promise<Episode> {
    return ApiClient.invoke<Episode>('create_episode', { projectId, name, content })
  }

  static async importEpisode(
    projectId: string,
    name: string,
    content: string
  ): Promise<Episode> {
    return ApiClient.invoke<Episode>('import_episode', { projectId, name, content })
  }

  static async getEpisode(id: string): Promise<Episode | null> {
    return ApiClient.invoke<Episode | null>('get_episode', { id })
  }

  static async updateEpisode(
    id: string,
    name?: string,
    content?: string
  ): Promise<Episode | null> {
    return ApiClient.invoke<Episode | null>('update_episode', { id, name, content })
  }

  static async deleteEpisode(id: string): Promise<boolean> {
    return ApiClient.invoke<boolean>('delete_episode', { id })
  }
}
