import { invoke } from '@tauri-apps/api/core'
import type { Task } from '../types'

interface TaskRaw {
  id: string
  type: string
  status: string
  project: string
  episodeId?: string
  targetType?: string
  targetId?: string
  payload?: string
  result?: string
  progress: number
  errorMessage?: string
  createdAt: number
  startedAt?: number
  finishedAt?: number
}

function parseTask(task: TaskRaw): Task {
  return {
    id: task.id,
    type: task.type,
    status: task.status as Task['status'],
    project: task.project,
    episodeId: task.episodeId,
    targetType: task.targetType,
    targetId: task.targetId,
    payload: task.payload ? JSON.parse(task.payload) : undefined,
    result: task.result ? JSON.parse(task.result) : undefined,
    progress: task.progress,
    errorMessage: task.errorMessage,
    createdAt: task.createdAt,
    startedAt: task.startedAt,
    finishedAt: task.finishedAt
  }
}

export class TasksApi {
  static async getTasks(): Promise<Task[]> {
    const tasks = await invoke<TaskRaw[]>('get_tasks')
    return tasks.map(parseTask)
  }

  static async getTask(taskId: string): Promise<Task | null> {
    const task = await invoke<TaskRaw | null>('get_task', { taskId })
    if (!task) return null
    return parseTask(task)
  }

  static async createTask(params: {
    taskType: string
    project: string
    episodeId?: string
    targetType?: string
    targetId?: string
    payload?: Record<string, unknown>
  }): Promise<Task> {
    const task = await invoke<TaskRaw>('create_task', {
      taskType: params.taskType,
      project: params.project,
      episodeId: params.episodeId,
      targetType: params.targetType,
      targetId: params.targetId,
      payload: params.payload ? JSON.stringify(params.payload) : undefined
    })
    return parseTask(task)
  }
}
