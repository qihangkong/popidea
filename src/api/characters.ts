import { ApiClient } from './client'
import type { GlobalCharacter, GlobalLocation } from '../types'

export class CharactersApi {
  static async getGlobalCharacters(projectId: string): Promise<GlobalCharacter[]> {
    return ApiClient.invoke<GlobalCharacter[]>('get_global_characters', { projectId })
  }

  static async createGlobalCharacter(
    projectId: string,
    name: string,
    description?: string
  ): Promise<GlobalCharacter> {
    return ApiClient.invoke<GlobalCharacter>('create_global_character', {
      projectId,
      name,
      description,
    })
  }

  static async updateGlobalCharacter(
    id: string,
    name?: string,
    description?: string
  ): Promise<GlobalCharacter | null> {
    return ApiClient.invoke<GlobalCharacter | null>('update_global_character', {
      id,
      name,
      description,
    })
  }

  static async deleteGlobalCharacter(id: string): Promise<boolean> {
    return ApiClient.invoke<boolean>('delete_global_character', { id })
  }
}

export class LocationsApi {
  static async getGlobalLocations(projectId: string): Promise<GlobalLocation[]> {
    return ApiClient.invoke<GlobalLocation[]>('get_global_locations', { projectId })
  }

  static async createGlobalLocation(
    projectId: string,
    name: string,
    description?: string
  ): Promise<GlobalLocation> {
    return ApiClient.invoke<GlobalLocation>('create_global_location', {
      projectId,
      name,
      description,
    })
  }

  static async updateGlobalLocation(
    id: string,
    name?: string,
    description?: string
  ): Promise<GlobalLocation | null> {
    return ApiClient.invoke<GlobalLocation | null>('update_global_location', {
      id,
      name,
      description,
    })
  }

  static async deleteGlobalLocation(id: string): Promise<boolean> {
    return ApiClient.invoke<boolean>('delete_global_location', { id })
  }
}
