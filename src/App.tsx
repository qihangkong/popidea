import { useState } from 'react'
import { Layers, Folder, Settings as SettingsIcon, Lightbulb, Brain, Image, Video, Mic, Key, Globe, Cpu, Plus, MoreHorizontal, Users, X } from 'lucide-react'
import './App.css'

interface ModelConfigCard {
  id: string
  name: string
  type: 'text' | 'image' | 'video' | 'voice' | 'comfyui'
  modelName: string
  apiUrl: string
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

  const [modelConfigs, setModelConfigs] = useState<ModelConfigCard[]>([])

  const assetTypes = ['角色', '场景', '道具', '分镜头', '分镜视频', '成片']

  const handleAddModel = () => {
    const newConfig: ModelConfigCard = {
      id: Date.now().toString(),
      name: '',
      type: 'text',
      modelName: '',
      apiUrl: '',
      apiKey: '',
      enabled: true
    }
    setModelConfigs([...modelConfigs, newConfig])
  }

  const handleDeleteModel = (id: string) => {
    setModelConfigs(modelConfigs.filter(config => config.id !== id))
  }

  const handleModelConfigChange = (id: string, field: keyof ModelConfigCard, value: string | boolean) => {
    setModelConfigs(modelConfigs.map(config =>
      config.id === id ? { ...config, [field]: value } : config
    ))
  }

  const getTypeInfo = (type: string) => {
    switch (type) {
      case 'text': return { icon: Brain, label: '文本模型', color: '#667eea' }
      case 'image': return { icon: Image, label: '图像模型', color: '#f59e0b' }
      case 'video': return { icon: Video, label: '视频模型', color: '#10b981' }
      case 'voice': return { icon: Mic, label: '语音模型', color: '#ef4444' }
      case 'comfyui': return { icon: Cpu, label: 'ComfyUI', color: '#8b5cf6' }
      default: return { icon: Brain, label: '模型', color: '#667eea' }
    }
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

  const ModelCard = ({ config }: { config: ModelConfigCard }) => {
    const typeInfo = getTypeInfo(config.type)
    const Icon = typeInfo.icon

    return (
      <div className="settings-section">
        <div className="section-header">
          <div className="section-icon" style={{ color: typeInfo.color }}>
            <Icon className="w-5 h-5" />
          </div>
          <div className="flex-1">
            <input
              type="text"
              value={config.name}
              onChange={(e) => handleModelConfigChange(config.id, 'name', e.target.value)}
              className="model-name-input"
              placeholder={typeInfo.label + '配置'}
              autoComplete="off"
            />
            <p className="section-description">{typeInfo.label}</p>
          </div>
          <button
            onClick={() => handleDeleteModel(config.id)}
            className="delete-model-btn"
            type="button"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="config-block">
          <div className="config-toggle">
            <label className="toggle-label">
              <input
                type="checkbox"
                checked={config.enabled}
                onChange={(e) => handleModelConfigChange(config.id, 'enabled', e.target.checked)}
                className="toggle-input"
              />
              <span className="toggle-slider"></span>
              <span className="toggle-text">启用</span>
            </label>
          </div>

          {config.enabled && (
            <div className="config-fields">
              <div className="field-group">
                <label>模型类型</label>
                <select
                  value={config.type}
                  onChange={(e) => handleModelConfigChange(config.id, 'type', e.target.value)}
                  className="config-input"
                >
                  <option value="text">文本模型</option>
                  <option value="image">图像模型</option>
                  <option value="video">视频模型</option>
                  <option value="voice">语音模型</option>
                  <option value="comfyui">ComfyUI</option>
                </select>
              </div>
              <div className="field-group">
                <label>模型名称</label>
                <input
                  type="text"
                  value={config.modelName}
                  onChange={(e) => handleModelConfigChange(config.id, 'modelName', e.target.value)}
                  className="config-input"
                  placeholder="例如：gpt-4"
                  autoComplete="off"
                />
              </div>
              <div className="field-group">
                <label>API URL</label>
                <div className="input-wrapper">
                  <Globe className="input-icon" />
                  <input
                    type="text"
                    value={config.apiUrl}
                    onChange={(e) => handleModelConfigChange(config.id, 'apiUrl', e.target.value)}
                    className="config-input"
                    placeholder="https://api.openai.com/v1"
                    autoComplete="off"
                  />
                </div>
              </div>
              <div className="field-group">
                <label>API Key</label>
                <div className="input-wrapper">
                  <Key className="input-icon" />
                  <input
                    type="password"
                    value={config.apiKey}
                    onChange={(e) => handleModelConfigChange(config.id, 'apiKey', e.target.value)}
                    className="config-input"
                    placeholder="sk-..."
                    autoComplete="off"
                  />
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    )
  }

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
                <div>
                  <h1>设置</h1>
                  <p className="settings-description">配置您的模型和 API 设置</p>
                </div>
                <button onClick={handleAddModel} className="settings-add-btn">
                  <Plus className="w-4 h-4" />
                  添加
                </button>
              </div>

              <div className="settings-content">
                {modelConfigs.length === 0 ? (
                  <div className="empty-models-state">
                    <Brain className="empty-icon" />
                    <p>暂无模型配置</p>
                    <p className="text-sm text-gray-400">点击右上角"添加"按钮来配置您的模型</p>
                  </div>
                ) : (
                  modelConfigs.map((config) => (
                    <ModelCard key={config.id} config={config} />
                  ))
                )}
              </div>
            </div>
          </>
        ) : null}
      </main>
    </div>
  )
}

export default App
