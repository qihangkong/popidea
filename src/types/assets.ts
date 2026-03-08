// ==================== Character Appearances ====================

export interface CharacterAppearance {
  id: string
  character_id: string
  description?: string
  image_url?: string
  is_selected: boolean
  created_at: number
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

export type AssetType = 'character' | 'location' | 'other'

// ==================== AI Design ====================

export interface AIDesignResult {
  success: boolean
  prompt?: string
  error?: string
}
