import { ApiClient } from './client'
import type { Storyboard, Panel } from '../types'

export class StoryboardsApi {
  static async getStoryboards(episodeId: string): Promise<Storyboard[]> {
    return ApiClient.invoke<Storyboard[]>('get_storyboards', { episodeId })
  }

  static async createStoryboard(episodeId: string, name: string): Promise<Storyboard> {
    return ApiClient.invoke<Storyboard>('create_storyboard', { episodeId, name })
  }

  static async getPanels(storyboardId: string): Promise<Panel[]> {
    return ApiClient.invoke<Panel[]>('get_panels', { storyboardId })
  }

  static async createPanel(storyboardId: string, panelNumber: number): Promise<Panel> {
    return ApiClient.invoke<Panel>('create_panel', { storyboardId, panelNumber })
  }

  static async updatePanel(
    id: string,
    updates: Partial<Panel>
  ): Promise<Panel | null> {
    return ApiClient.invoke<Panel | null>('update_panel', {
      id,
      shotType: updates.shotType,
      cameraMove: updates.cameraMove,
      description: updates.description,
      location: updates.location,
      characters: updates.characters ? JSON.stringify(updates.characters) : undefined,
      srtStart: updates.srtStart,
      srtEnd: updates.srtEnd,
      duration: updates.duration,
      videoPrompt: updates.videoPrompt,
      imageUrl: updates.imageUrl,
      videoUrl: updates.videoUrl,
    })
  }

  static async deletePanel(id: string): Promise<boolean> {
    return ApiClient.invoke<boolean>('delete_panel', { id })
  }
}
