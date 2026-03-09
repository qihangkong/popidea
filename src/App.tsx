import { useState } from 'react'
import { Layers, Folder, Settings as SettingsIcon } from 'lucide-react'
import './App.css'

function App() {
  const [activeItem, setActiveItem] = useState('资产')

  return (
    <div className="app">
      <aside className="sidebar">
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
        <h1>欢迎使用</h1>
        <p>这是一个示例页面，主内容区域可以显示任何内容。</p>
        <p>左侧是导航栏，包含资产、项目和设置选项。</p>
        <p>当前选中：{activeItem}</p>
      </main>
    </div>
  )
}

export default App
