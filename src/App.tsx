import { BrowserRouter as Router, Routes, Route, Navigate, useLocation } from 'react-router-dom'
import { useState } from 'react'
import { Link } from 'react-router-dom'
import { HomePage, ProjectDetail, TaskMonitor, AssetsPage } from './pages'

function Sidebar() {
  const [collapsed, setCollapsed] = useState(false)
  const location = useLocation()

  const isActive = (path: string) => {
    if (path === '/') {
      return location.pathname === '/'
    }
    return location.pathname.startsWith(path)
  }

  const menuItems = [
    { path: '/assets', label: '资产', icon: '🎨' },
    { path: '/', label: '项目', icon: '📁' },
  ]

  return (
    <aside
      className={`bg-slate-900 border-r border-slate-800 z-50 flex flex-col transition-all duration-300 ${
        collapsed ? 'w-16' : 'w-64'
      }`}
      style={{ display: 'flex', flexDirection: 'column', width: collapsed ? '64px' : '256px', flexShrink: 0 }}
    >
      {/* Logo */}
      <div className="p-4 border-b border-slate-800 flex items-center justify-between">
        {!collapsed && (
          <Link
            to="/"
            className="flex items-center gap-2 text-xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent"
          >
            <span className="text-2xl">💡</span>
            <span>PopIdea</span>
          </Link>
        )}
        {collapsed && (
          <Link
            to="/"
            className="flex items-center justify-center w-full"
          >
            <span className="text-2xl">💡</span>
          </Link>
        )}
        <button
          onClick={() => setCollapsed(!collapsed)}
          className="p-2 rounded-lg hover:bg-slate-800 transition-colors text-slate-400 hover:text-slate-200"
        >
          <svg
            className={`w-5 h-5 transition-transform ${collapsed ? 'rotate-180' : ''}`}
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11 19l-7-7 7-7M5 12l7 7-7-7" />
          </svg>
        </button>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-2 space-y-1">
        {menuItems.map((item) => (
          <Link
            key={item.path}
            to={item.path}
            className={`flex items-center gap-3 px-3 py-2 rounded-lg text-sm font-medium transition-all group ${
              isActive(item.path)
                ? 'bg-indigo-500/20 text-indigo-400'
                : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800'
            }`}
            title={collapsed ? item.label : undefined}
          >
            <span className="text-xl">{item.icon}</span>
            {!collapsed && <span>{item.label}</span>}
          </Link>
        ))}
      </nav>

      {/* User Section */}
      <div className="p-2 border-t border-slate-800">
        <div
          className="flex items-center gap-3 px-3 py-2 rounded-lg bg-slate-800/50 hover:bg-slate-800 transition-colors cursor-pointer"
          title={collapsed ? '用户' : undefined}
        >
          <div className="w-8 h-8 rounded-full bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-white font-bold text-sm flex-shrink-0">
            U
          </div>
          {!collapsed && (
            <div className="flex-1 min-w-0">
              <div className="text-sm font-medium text-slate-200">用户</div>
              <div className="text-xs text-slate-400 truncate">user@example.com</div>
            </div>
          )}
        </div>
      </div>
    </aside>
  )
}

function App() {
  return (
    <Router>
      <div className="bg-gradient-to-br from-slate-900 to-slate-800" style={{ display: 'flex', flexDirection: 'row', height: '100vh' }}>
        <Sidebar />
        <main className="flex-1 overflow-auto" style={{ display: 'flex', flexDirection: 'column', flex: '1', overflow: 'auto' }}>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/project/:projectId" element={<ProjectDetail />} />
            <Route path="/tasks" element={<TaskMonitor />} />
            <Route path="/assets" element={<AssetsPage />} />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </main>
      </div>
    </Router>
  )
}

export default App
