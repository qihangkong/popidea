import { invoke } from '@tauri-apps/api/core'

// ==================== Character Appearances ====================

export interface CharacterAppearance {
  id: string
  character_id: string
  description?: string
  image_url?: string
  is_selected: boolean
  created_at: number
}

export async function getCharacterAppearances(characterId: string): Promise<CharacterAppearance[]> {
  return await invoke('get_character_appearances', { characterId })
}

export async function createCharacterAppearance(
  characterId: string,
  description?: string,
  imageUrl?: string
): Promise<CharacterAppearance> {
  return await invoke('create_character_appearance', {
    characterId,
    description,
    imageUrl,
  })
}

export async function updateCharacterAppearance(
  id: string,
  description?: string,
  imageUrl?: string,
  isSelected?: boolean
): Promise<CharacterAppearance | null> {
  return await invoke('update_character_appearance', {
    id,
    description,
    imageUrl,
    isSelected,
  })
}

export async function deleteCharacterAppearance(id: string): Promise<boolean> {
  return await invoke('delete_character_appearance', { id })
}

export async function selectCharacterAppearance(
  characterId: string,
  appearanceId: string
): Promise<void> {
  return await invoke('select_character_appearance', {
    characterId,
    appearanceId,
  })
}

// ==================== Asset Folders ====================

export interface AssetFolder {
  id: string
  project_id: string
  name: string
  parent_id?: string
  created_at: number
  updated_at: number
}

export async function getAssetFolders(
  projectId: string,
  parentId?: string
): Promise<AssetFolder[]> {
  return await invoke('get_asset_folders', {
    projectId,
    parentId,
  })
}

export async function createAssetFolder(
  projectId: string,
  name: string,
  parentId?: string
): Promise<AssetFolder> {
  return await invoke('create_asset_folder', {
    projectId,
    name,
    parentId,
  })
}

export async function updateAssetFolder(
  id: string,
  name?: string,
  parentId?: string
): Promise<AssetFolder | null> {
  return await invoke('update_asset_folder', {
    id,
    name,
    parentId,
  })
}

export async function deleteAssetFolder(id: string): Promise<boolean> {
  return await invoke('delete_asset_folder', { id })
}

// ==================== Assets ====================

export interface Asset {
  id: string
  project_id: string
  folder_id?: string
  asset_type: string
  name?: string
  description?: string
  image_url?: string
  metadata?: string
  labels?: string
  created_at: number
  updated_at: number
}

export async function getAssets(
  projectId: string,
  folderId?: string,
  assetType?: string
): Promise<Asset[]> {
  return await invoke('get_assets', {
    projectId,
    folderId,
    assetType,
  })
}

export async function createAsset(
  data: Omit<Asset, 'id' | 'created_at' | 'updated_at'>
): Promise<Asset> {
  return await invoke('create_asset', data)
}

export async function updateAsset(
  id: string,
  updates: Partial<Pick<Asset, 'name' | 'description' | 'image_url' | 'metadata' | 'labels'>>
): Promise<Asset | null> {
  return await invoke('update_asset', { id, ...updates })
}

export async function deleteAsset(id: string): Promise<boolean> {
  return await invoke('delete_asset', { id })
}
