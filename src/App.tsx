import { useState } from 'react'
import { Layers, Folder, Settings as SettingsIcon, Lightbulb, Brain, Key, Globe, Cpu, Plus, MoreHorizontal, Users, X, Image} from 'lucide-react'
import './App.css'

interface ModelConfigCard {
  id: string
  name: string
  platform: 'zhipu' | 'volcengine' | 'comfyui' | 'custom' | 'openai' | 'stability' | 'midjourney'
  apiUrl?: string
  apiKey: string
}

interface Project {
  id: string
  title: string
  thumbnail: string
  chapters: number
  collaborators: number
  updatedAt: string
}

type SettingsTab = 'llm' | 'image'

function App() {
  const [activeItem, setActiveItem] = useState('资产')
  const [activeAssetType, setActiveAssetType] = useState('角色')
  const [projectTab, setProjectTab] = useState('myProjects')
  const [settingsTab, setSettingsTab] = useState<SettingsTab>('llm')

  const [llmConfigs, setLlmConfigs] = useState<ModelConfigCard[]>([])
  const [imageConfigs, setImageConfigs] = useState<ModelConfigCard[]>([])

  const handleAddLLMModel = () => {
    const config: ModelConfigCard = {
      id: Date.now().toString(),
      name: '质谱配置',
      platform: 'zhipu',
      apiUrl: undefined,
      apiKey: ''
    }
    setLlmConfigs([...llmConfigs, config])
  }

  const handleAddImageModel = () => {
    const config: ModelConfigCard = {
      id: Date.now().toString(),
      name: 'ComfyUI配置',
      platform: 'comfyui',
      apiUrl: 'http://127.0.0.1:8188',
      apiKey: ''
    }
    setImageConfigs([...imageConfigs, config])
  }

  const assetTypes = ['角色', '场景', '道具', '分镜头', '分镜视频', '成片']

  const platformConfig = {
    zhipu: { name: '质谱', defaultUrl: 'https://open.bigmodel.cn/api/paas/v4', icon: Brain, color: '#667eea' },
    volcengine: { name: '火山引擎', defaultUrl: 'https://ark.cn-beijing.volces.com/api/v3', icon: Globe, color: '#f59e0b' },
    openai: { name: 'OpenAI', defaultUrl: 'https://api.openai.com/v1', icon: Brain, color: '#10a37f' },
    comfyui: { name: 'ComfyUI', defaultUrl: 'http://127.0.0.1:8188', icon: Cpu, color: '#8b5cf6' },
    stability: { name: 'Stability AI', defaultUrl: 'https://api.stability.ai/v1', icon: Image, color: '#ff6b6b' },
    midjourney: { name: 'Midjourney', defaultUrl: '', icon: Image, color: '#ec4899' },
    custom: { name: '自定义', defaultUrl: '', icon: SettingsIcon, color: '#10b981' }
  }

  const handleDeleteModel = (id: string, type: SettingsTab) => {
    if (type === 'llm') {
      setLlmConfigs(llmConfigs.filter(config => config.id !== id))
    } else {
      setImageConfigs(imageConfigs.filter(config => config.id !== id))
    }
  }

  const handleModelConfigChange = (id: string, field: keyof ModelConfigCard, value: string | boolean, type: SettingsTab) => {
    const updater = (configs: ModelConfigCard[]) =>
      configs.map(config =>
        config.id === id ? { ...config, [field]: value } : config
      )

    if (type === 'llm') {
      setLlmConfigs(updater(llmConfigs))
    } else {
      setImageConfigs(updater(imageConfigs))
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

  const ModelCard = ({ config, type }: { config: ModelConfigCard, type: SettingsTab }) => {
    const platformInfo = platformConfig[config.platform]
    const Icon = platformInfo.icon
    const showApiUrl = config.platform === 'comfyui' || config.platform === 'custom' || config.platform === 'stability'

    return (
      <div className="settings-section">
        <div className="section-header">
          <div className="section-icon" style={{ color: platformInfo.color }}>
            <Icon className="w-5 h-5" />
          </div>
          <div className="flex-1">
            <input
              type="text"
              value={config.name}
              onChange={(e) => handleModelConfigChange(config.id, 'name', e.target.value, type)}
              className="model-name-input"
              placeholder={platformInfo.name + '配置'}
              autoComplete="off"
            />
            <p className="section-description">{platformInfo.name}</p>
          </div>
          <button
            onClick={() => handleDeleteModel(config.id, type)}
            className="delete-model-btn"
            type="button"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        <div className="config-block">
          <div className="config-fields">
            <div className="field-group">
              <label>平台</label>
              <select
                value={config.platform}
                onChange={(e) => handleModelConfigChange(config.id, 'platform', e.target.value as any, type)}
                className="config-input"
              >
                {type === 'llm' ? (
                  <>
                    <option value="zhipu">质谱</option>
                    <option value="volcengine">火山引擎</option>
                    <option value="openai">OpenAI</option>
                    <option value="custom">自定义</option>
                  </>
                ) : (
                  <>
                    <option value="comfyui">ComfyUI</option>
                    <option value="stability">Stability AI</option>
                    <option value="midjourney">Midjourney</option>
                    <option value="custom">自定义</option>
                  </>
                )}
              </select>
            </div>

            {showApiUrl && (
              <div className="field-group">
                <label>API URL</label>
                <div className="input-wrapper">
                  <Globe className="input-icon" />
                  <input
                    type="text"
                    value={config.apiUrl || ''}
                    onChange={(e) => handleModelConfigChange(config.id, 'apiUrl', e.target.value, type)}
                    className="config-input"
                    placeholder={config.platform === 'comfyui' ? 'http://127.0.0.1:8188' : config.platform === 'stability' ? 'https://api.stability.ai/v1' : 'https://api.example.com/v1'}
                    autoComplete="off"
                  />
                </div>
              </div>
            )}

            <div className="field-group">
              <label>API Key</label>
              <div className="input-wrapper">
                <Key className="input-icon" />
                <input
                  type="password"
                  value={config.apiKey}
                  onChange={(e) => handleModelConfigChange(config.id, 'apiKey', e.target.value, type)}
                  className="config-input"
                  placeholder="输入您的 API Key"
                  autoComplete="off"
                />
              </div>
            </div>
          </div>
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
              </div>

              <div className="settings-tabs">
                <button
                  className={`settings-tab ${settingsTab === 'llm' ? 'active' : ''}`}
                  onClick={() => setSettingsTab('llm')}
                >
                  <Brain className="w-4 h-4" />
                  大语言模型设置
                </button>
                <button
                  className={`settings-tab ${settingsTab === 'image' ? 'active' : ''}`}
                  onClick={() => setSettingsTab('image')}
                >
                  <Image className="w-4 h-4" />
                  图片生成设置
                </button>
              </div>

              <div className="settings-content">
                {settingsTab === 'llm' ? (
                  <>
                    {llmConfigs.length === 0 ? (
                      <div className="empty-models-state">
                        <Brain className="empty-icon" />
                        <p>暂无大语言模型配置</p>
                        <p className="text-sm text-gray-400">点击"添加大语言模型"按钮来配置您的模型</p>
                      </div>
                    ) : (
                      llmConfigs.map((config) => (
                        <ModelCard key={config.id} config={config} type="llm" />
                      ))
                    )}
                    <button onClick={handleAddLLMModel} className="settings-add-btn">
                      <Plus className="w-4 h-4" />
                      添加大语言模型
                    </button>
                  </>
                ) : (
                  <>
                    {imageConfigs.length === 0 ? (
                      <div className="empty-models-state">
                        <Image className="empty-icon" />
                        <p>暂无图片生成模型配置</p>
                        <p className="text-sm text-gray-400">点击"添加图片生成模型"按钮来配置您的模型</p>
                      </div>
                    ) : (
                      imageConfigs.map((config) => (
                        <ModelCard key={config.id} config={config} type="image" />
                      ))
                    )}
                    <button onClick={handleAddImageModel} className="settings-add-btn">
                      <Plus className="w-4 h-4" />
                      添加图片生成模型
                    </button>
                  </>
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
