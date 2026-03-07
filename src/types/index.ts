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
