export interface Project {
  id: string
  name: string
  description?: string
  createdAt: number
  updatedAt: number
  settings?: Record<string, unknown>
}

export interface Episode {
  id: string
  projectId: string
  name: string
  content?: string
  createdAt: number
  updatedAt: number
}

export interface Storyboard {
  id: string
  episodeId: string
  name: string
  panelCount: number
  createdAt: number
  updatedAt: number
}

export interface Panel {
  id: string
  storyboardId: string
  panelIndex: number
  panelNumber: number
  shotType?: string
  cameraMove?: string
  description?: string
  location?: string
  characters?: string[]
  srtStart?: number
  srtEnd?: number
  duration?: number
  videoPrompt?: string
  imageUrl?: string
  videoUrl?: string
}

export interface GlobalCharacter {
  id: string
  projectId: string
  name: string
  description?: string
  appearances?: string[]
  createdAt: number
  updatedAt: number
}

export interface GlobalLocation {
  id: string
  projectId: string
  name: string
  description?: string
  createdAt: number
  updatedAt: number
}

export interface Task {
  id: string
  type: string
  status: 'queued' | 'processing' | 'completed' | 'failed'
  project: string
  episodeId?: string
  targetType?: string
  targetId?: string
  payload?: Record<string, unknown>
  result?: Record<string, unknown>
  progress: number
  errorMessage?: string
  createdAt: number
  startedAt?: number
  finishedAt?: number
}

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

export interface AssetFolder {
  id: string
  project_id: string
  name: string
  parent_id?: string
  created_at: number
  updated_at: number
}
