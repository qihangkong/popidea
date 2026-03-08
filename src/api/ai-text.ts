import { invoke } from '@tauri-apps/api/core'
import type { Task } from '../types'

export interface AnalyzeOptions {
  provider?: string
  apiKey?: string
  model?: string
}

// ==================== Novel Analysis ====================

/**
 * 分析小说内容，提取故事结构、角色、场景等信息
 */
export async function analyzeNovel(
  projectId: string,
  episodeId: string | undefined,
  content: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('analyze_novel', {
    projectId,
    episodeId,
    content,
    ...options,
  })
}

// ==================== Global Analysis ====================

/**
 * 分析全局内容，提取角色、场景等全局信息
 */
export async function analyzeGlobal(
  projectId: string,
  episodeId: string | undefined,
  content: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('analyze_global', {
    projectId,
    episodeId,
    content,
    ...options,
  })
}

// ==================== Story to Script ====================

/**
 * 将小说/故事转换为剧本格式
 */
export async function convertStoryToScript(
  projectId: string,
  episodeId: string | undefined,
  content: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('convert_story_to_script', {
    projectId,
    episodeId,
    content,
    ...options,
  })
}

// ==================== Script to Storyboard ====================

/**
 * 将剧本转换为分镜格式
 */
export async function convertScriptToStoryboard(
  projectId: string,
  episodeId: string | undefined,
  content: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('convert_script_to_storyboard', {
    projectId,
    episodeId,
    content,
    ...options,
  })
}

// ==================== Asset Design ====================

/**
 * AI 设计角色描述
 */
export async function aiDesignCharacter(
  projectId: string,
  userInstruction: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('ai_design_character', {
    projectId,
    userInstruction,
    ...options,
  })
}

/**
 * AI 设计场景描述
 */
export async function aiDesignLocation(
  projectId: string,
  userInstruction: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('ai_design_location', {
    projectId,
    userInstruction,
    ...options,
  })
}

// ==================== Asset Modification ====================

/**
 * AI 修改角色外貌
 */
export async function aiModifyAppearance(
  projectId: string,
  characterId: string,
  currentDescription: string,
  modificationInstruction: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('ai_modify_appearance', {
    projectId,
    characterId,
    currentDescription,
    modificationInstruction,
    ...options,
  })
}

/**
 * AI 修改场景
 */
export async function aiModifyLocation(
  projectId: string,
  locationId: string,
  currentDescription: string,
  modificationInstruction: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('ai_modify_location', {
    projectId,
    locationId,
    currentDescription,
    modificationInstruction,
    ...options,
  })
}

/**
 * AI 修改镜头提示
 */
export async function aiModifyShotPrompt(
  projectId: string,
  panelId: string,
  currentPrompt: string,
  modificationInstruction: string,
  options?: AnalyzeOptions
): Promise<Task> {
  return await invoke('ai_modify_shot_prompt', {
    projectId,
    panelId,
    currentPrompt,
    modificationInstruction,
    ...options,
  })
}
