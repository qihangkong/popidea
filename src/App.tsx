import { useState } from 'react'
import { Layers, Folder, Settings as SettingsIcon, Lightbulb, Brain, Image, Video, Mic, Key, Globe, Cpu, Plus, MoreHorizontal, Users } from 'lucide-react'
import './App.css'

interface ModelConfig {
  modelName: string
  apiUrl: string
  apiKey: string
  enabled: boolean
}

interface ComfyUIConfig {
  url: string
  apiKey: string
  enabled: boolean
}

interface Project {
  id: string
  title: string
  thumbnail: string
  chapters: number
  collaborators: number
  updatedAt: string
}

function App() {
  const [activeItem, setActiveItem] = useState('资产')
  const [activeAssetType, setActiveAssetType] = useState('角色')
  const [projectTab, setProjectTab] = useState('myProjects')

  const [textModel, setTextModel] = useState<ModelConfig>({
    modelName: '',
    apiUrl: '',
    apiKey: '',
    enabled: false
  })

  const [imageModel, setImageModel] = useState<ModelConfig>({
    modelName: '',
    apiUrl: '',
    apiKey: '',
    enabled: false
  })

  const [videoModel, setVideoModel] = useState<ModelConfig>({
    modelName: '',
    apiUrl: '',
    apiKey: '',
    enabled: false
  })

  const [voiceModel, setVoiceModel] = useState<ModelConfig>({
    modelName: '',
    apiUrl: '',
    apiKey: '',
    enabled: false
  })

  const [comfyUI, setComfyUI] = useState<ComfyUIConfig>({
    url: '',
    apiKey: '',
    enabled: false
  })

  const assetTypes = ['角色', '场景', '道具', '分镜头', '分镜视频', '成片']

  const handleModelChange = (setter: (config: ModelConfig) => void, field: keyof ModelConfig, value: string | boolean) => {
    setter((prev) => ({ ...prev, [field]: value }))
  }

  const handleComfyUIChange = (field: keyof ComfyUIConfig, value: string | boolean) => {
    setComfyUI((prev) => ({ ...prev, [field]: value }))
  }

  const myProjects: Project[] = [
    {
      id: '1',
      title: '《别叫我弟弟》',
      thumbnail: 'https://pubsto.fullpeace.cn/aigc/entweb/20251223095315.jpg',
      chapters: 0,
      collaborators: 0,
      updatedAt: '2026-03-11 13:59:49'
    }
  ]

  const participatedProjects: Project[] = []

  const ProjectCard = ({ project }: { project: Project }) => (
    <div className="video-card cursor-pointer">
      <div className="absolute right-2 top-1 z-[3] bg-black bg-opacity-50 rounded-lg px-1 py-[2px] flex items-center justify-center text-xl cursor-pointer hover:bg-opacity-70">
        <MoreHorizontal className="w-5 h-5 text-white" />
      </div>
      <div className="relative flex items-center justify-center overflow-hidden aspect-square">
        <div className="el-image w-full h-full object-contain">
          <img src={project.thumbnail} className="w-full h-full object-cover" alt={project.title} />
        </div>
        <div className="absolute bottom-2 right-2">
          <span className="text-xs px-2 py-1 bg-black/80 rounded text-white backdrop-blur-sm">共{project.chapters}章</span>
        </div>
      </div>
      <div className="p-2">
        <h3 className="text-sm">{project.title}</h3>
        <p className="text-xs opacity-50 mt-1">{project.updatedAt}</p>
        <div className="flex items-center gap-1 mt-2">
          <Users className="w-3 h-3 text-gray-500" />
          <span className="text-gray-400 text-xs">{project.collaborators}人协作</span>
        </div>
      </div>
    </div>
  )

  return (
    <div className="app">
      <aside className="sidebar">
        <div className="sidebar-header">
          <Lightbulb className="logo-icon" />
          <span className="logo-text">PopIdea</span>
        </div>
        <div className="sidebar-divider"></div>
        <div className="sidebar-content">
          <div
            className={`nav-item ${activeItem === '资产' ? 'active' : ''}`}
            onClick={() => setActiveItem('资产')}
            tabIndex={-1}
          >
            <Layers className="nav-icon" />
            <span>资产</span>
          </div>
          <div
            className={`nav-item ${activeItem === '项目' ? 'active' : ''}`}
            onClick={() => setActiveItem('项目')}
            tabIndex={-1}
          >
            <Folder className="nav-icon" />
            <span>项目</span>
          </div>
        </div>
        <div className="sidebar-footer">
          <div
            className={`nav-item settings ${activeItem === '设置' ? 'active' : ''}`}
            onClick={() => setActiveItem('设置')}
            tabIndex={-1}
          >
            <SettingsIcon className="nav-icon" />
            <span>设置</span>
          </div>
        </div>
      </aside>
      <main className="main-content">
        {activeItem === '资产' ? (
          <>
            <div className="content-header">
              <h1>资产</h1>
              <div className="asset-tabs">
                {assetTypes.map((type) => (
                  <button
                    key={type}
                    className={`asset-tab ${activeAssetType === type ? 'active' : ''}`}
                    onClick={() => setActiveAssetType(type)}
                  >
                    {type}
                  </button>
                ))}
              </div>
            </div>
            <div className="content-body">
              <p>当前选中：{activeAssetType}</p>
            </div>
          </>
        ) : activeItem === '项目' ? (
          <>
            <div className="project-page">
              <div className="project-header">
                <div className="project-tabs">
                  <button
                    className={`project-tab ${projectTab === 'myProjects' ? 'active' : ''}`}
                    onClick={() => setProjectTab('myProjects')}
                  >
                    我的项目
                  </button>
                  <button
                    className={`project-tab ${projectTab === 'participated' ? 'active' : ''}`}
                    onClick={() => setProjectTab('participated')}
                  >
                    我参与的项目
                  </button>
                </div>
                <button className="create-project-btn">
                  <Plus className="w-4 h-4" />
                  创建项目
                </button>
              </div>
              <div className="project-grid video-grid py-2">
                {(projectTab === 'myProjects' ? myProjects : participatedProjects).map((project) => (
                  <ProjectCard key={project.id} project={project} />
                ))}
                {(projectTab === 'myProjects' ? myProjects : participatedProjects).length === 0 && (
                  <div className="empty-state">
                    <p className="text-gray-400 text-sm">暂无项目</p>
                  </div>
                )}
              </div>
            </div>
          </>
        ) : activeItem === '设置' ? (
          <>
            <div className="settings-page">
              <div className="settings-header">
                <h1>设置</h1>
                <p className="settings-description">配置您的模型和 API 设置</p>
              </div>

              <div className="settings-content">
                <div className="settings-section">
                  <div className="section-header">
                    <div className="section-icon">
                      <Brain className="w-5 h-5" />
                    </div>
                    <div>
                      <h2>文本模型设置</h2>
                      <p className="section-description">配置用于文本生成的 AI 模型</p>
                    </div>
                  </div>

                  <div className="config-block">
                    <div className="config-toggle">
                      <label className="toggle-label">
                        <input
                          type="checkbox"
                          checked={textModel.enabled}
                          onChange={(e) => handleModelChange(setTextModel, 'enabled', e.target.checked)}
                          className="toggle-input"
                        />
                        <span className="toggle-slider"></span>
                        <span className="toggle-text">启用文本模型</span>
                      </label>
                    </div>

                    {textModel.enabled && (
                      <div className="config-fields">
                        <div className="field-group">
                          <label>模型名称</label>
                          <input
                            type="text"
                            value={textModel.modelName}
                            onChange={(e) => handleModelChange(setTextModel, 'modelName', e.target.value)}
                            className="config-input"
                            placeholder="例如：gpt-4"
                          />
                        </div>
                        <div className="field-group">
                          <label>API URL</label>
                          <div className="input-wrapper">
                            <Globe className="input-icon" />
                            <input
                              type="text"
                              value={textModel.apiUrl}
                              onChange={(e) => handleModelChange(setTextModel, 'apiUrl', e.target.value)}
                              className="config-input"
                              placeholder="https://api.openai.com/v1"
                            />
                          </div>
                        </div>
                        <div className="field-group">
                          <label>API Key</label>
                          <div className="input-wrapper">
                            <Key className="input-icon" />
                            <input
                              type="password"
                              value={textModel.apiKey}
                              onChange={(e) => handleModelChange(setTextModel, 'apiKey', e.target.value)}
                              className="config-input"
                              placeholder="sk-..."
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                <div className="settings-section">
                  <div className="section-header">
                    <div className="section-icon">
                      <Image className="w-5 h-5" />
                    </div>
                    <div>
                      <h2>图像模型设置</h2>
                      <p className="section-description">配置用于图像生成的 AI 模型</p>
                    </div>
                  </div>

                  <div className="config-block">
                    <div className="config-toggle">
                      <label className="toggle-label">
                        <input
                          type="checkbox"
                          checked={imageModel.enabled}
                          onChange={(e) => handleModelChange(setImageModel, 'enabled', e.target.checked)}
                          className="toggle-input"
                        />
                        <span className="toggle-slider"></span>
                        <span className="toggle-text">启用图像图像模型</span>
                      </label>
                    </div>

                    {imageModel.enabled && (
                      <div className="config-fields">
                        <div className="field-group">
                          <label>模型名称</label>
                          <input
                            type="text"
                            value={imageModel.modelName}
                            onChange={(e) => handleModelChange(setImageModel, 'modelName', e.target.value)}
                            className="config-input"
                            placeholder="例如：dall-e-3"
                          />
                        </div>
                        <div className="field-group">
                          <label>API URL</label>
                          <div className="input-wrapper">
                            <Globe className="input-icon" />
                            <input
                              type="text"
                              value={imageModel.apiUrl}
                              onChange={(e) => handleModelChange(setImageModel, 'apiUrl', e.target.value)}
                              className="config-input"
                              placeholder="https://api.openai.com/v1"
                            />
                          </div>
                        </div>
                        <div className="field-group">
                          <label>API Key</label>
                          <div className="input-wrapper">
                            <Key className="input-icon" />
                            <input
                              type="password"
                              value={imageModel.apiKey}
                              onChange={(e) => handleModelChange(setImageModel, 'apiKey', e.target.value)}
                              className="config-input"
                              placeholder="sk-..."
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                <div className="settings-section">
                  <div className="section-header">
                    <div className="section-icon">
                      <Video className="w-5 h-5" />
                    </div>
                    <div>
                      <h2>视频模型设置</h2>
                      <p className="section-description">配置用于视频生成的 AI 模型</p>
                    </div>
                  </div>

                  <div className="config-block">
                    <div className="config-toggle">
                      <label className="toggle-label">
                        <input
                          type="checkbox"
                          checked={videoModel.enabled}
                          onChange={(e) => handleModelChange(setVideoModel, 'enabled', e.target.checked)}
                          className="toggle-input"
                        />
                        <span className="toggle-slider"></span>
                        <span className="toggle-text">启用视频模型</span>
                      </label>
                    </div>

                    {videoModel.enabled && (
                      <div className="config-fields">
                        <div className="field-group">
                          <label>模型名称</label>
                          <input
                            type="text"
                            value={videoModel.modelName}
                            onChange={(e) => handleModelChange(setVideoModel, 'modelName', e.target.value)}
                            className="config-input"
                            placeholder="例如：sora"
                          />
                        </div>
                        <div className="field-group">
                          <label>API URL</label>
                          <div className="input-wrapper">
                            <Globe className="input-icon" />
                            <input
                              type="text"
                              value={videoModel.apiUrl}
                              onChange={(e) => handleModelChange(setVideoModel, 'apiUrl', e.target.value)}
                              className="config-input"
                              placeholder="https://api.openai.com/v1"
                            />
                          </div>
                        </div>
                        <div className="field-group">
                          <label>API Key</label>
                          <div className="input-wrapper">
                            <Key className="input-icon" />
                            <input
                              type="password"
                              value={videoModel.apiKey}
                              onChange={(e) => handleModelChange(setVideoModel, 'apiKey', e.target.value)}
                              className="config-input"
                              placeholder="sk-..."
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                <div className="settings-section">
                  <div className="section-header">
                    <div className="section-icon">
                      <Mic className="w-5 h-5" />
                    </div>
                    <div>
                      <h2>语音模型设置</h2>
                      <p className="section-description">配置用于语音合成的 AI 模型</p>
                    </div>
                  </div>

                  <div className="config-block">
                    <div className="=">
                      <label className="toggle-label">
                        <input
                          type="checkbox"
                          checked={voiceModel.enabled}
                          onChange={(e) => handleModelChange(setVoiceModel, 'enabled', e.target.checked)}
                          className="toggle-input"
                        />
                        <span className="toggle-slider"></span>
                        <span className="toggle-text">启用语音模型</span>
                      </label>
                    </div>

                    {voiceModel.enabled && (
                      <div className="config-fields">
                        <div className="field-group">
                          <label>模型名称</label>
                          <input
                            type="text"
                            value={voiceModel.modelName}
                            onChange={(e) => handleModelChange(setVoiceModel, 'modelName', e.target.value)}
                            className="config-input"
                            placeholder="例如：tts-1"
                          />
                        </div>
                        <div className="field-group">
                          <label>API URL</label>
                          <div className="input-wrapper">
                            <Globe className="input-icon" />
                            <input
                              type="text"
                              value={voiceModel.apiUrl}
                              onChange={(e) => handleModelChange(setVoiceModel, 'apiUrl', e.target.value)}
                              className="config-input"
                              placeholder="https://api.openai.com/v1"
                            />
                          </div>
                        </div>
                        <div className="field-group">
                          <label>API Key</label>
                          <div className="input-wrapper">
                            <Key className="input-icon" />
                            <input
                              type="password"
                              value={voiceModel.apiKey}
                              onChange={(e) => handleModelChange(setVoiceModel, 'apiKey', e.target.value)}
                              className="config-input"
                              placeholder="sk-..."
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>

                <div className="settings-section">
                  <div className="section-header">
                    <div className="section-icon">
                      <Cpu className="w-5 h-5" />
                    </div>
                    <div>
                      <h2>ComfyUI 配置</h2>
                      <p className="section-description">配置 ComfyUI 图像生成服务</p>
                    </div>
                  </div>

                  <div className="config-block">
                    <div className="config-toggle">
                      <label className="toggle-label">
                        <input
                          type="checkbox"
                          checked={comfyUI.enabled}
                          onChange={(e) => handleComfyUIChange('enabled', e.target.checked)}
                          className="toggle-input"
                        />
                        <span className="toggle-slider"></span>
                        <span className="toggle-text">启用 ComfyUI</span>
                      </label>
                    </div>

                    {comfyUI.enabled && (
                      <div className="config-fields">
                        <div className="field-group">
                          <label>ComfyUI URL</label>
                          <div className="input-wrapper">
                            <Globe className="input-icon" />
                            <input
                              type="text"
                              value={comfyUI.url}
                              onChange={(e) => handleComfyUIChange('url', e.target.value)}
                              className="config-input"
                              placeholder="http://localhost:8188"
                            />
                          </div>
                        </div>
                        <div className="field-group">
                          <label>API Key</label>
                          <div className="input-wrapper">
                            <Key className="input-icon" />
                            <input
                              type="password"
                              value={comfyUI.apiKey}
                              onChange={(e) => handleComfyUIChange('apiKey', e.target.value)}
                              className="config-input"
                              placeholder="可选"
                            />
                          </div>
                        </div>
                      </div>
                    )}
                  </div>
                </div>
              </div>
            </div>
          </>
        ) : null}
      </main>
    </div>
  )
}

export default App
