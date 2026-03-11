import { useState } from 'react'
import { Layers, Folder, Settings as SettingsIcon, Lightbulb, Brain, Image, Video, Mic, Key, Globe, Cpu } from 'lucide-react'
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

function App() {
  const [activeItem, setActiveItem] = useState('资产')
  const [activeAssetType, setActiveAssetType] = useState('角色')
  
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
                        <span className="toggle-text">启用图像模型</span>
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
                    <div className="config-toggle">
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
        ) : (
          <>
            <h1>欢迎使用</h1>
            <p>这是一个示例页面，主内容区域可以显示任何内容。</p>
            <p>左侧是导航栏，包含资产、项目和设置选项。</p>
            <p>当前选中：{activeItem}</p>
          </>
        )}
      </main>
    </div>
  )
}

export default App
