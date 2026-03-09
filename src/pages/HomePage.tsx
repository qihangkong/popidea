import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { ProjectsApi } from '../api'
import type { Project } from '../types'
import { PageContent, Card } from '../components/layout'

export function HomePage() {
  const [projects, setProjects] = useState<Project[]>([])
  const [loading, setLoading] = useState(true)
  const [newProjectName, setNewProjectName] = useState('')

  useEffect(() => {
    loadProjects()
  }, [])

  async function loadProjects() {
    try {
      setLoading(true)
      const data = await ProjectsApi.getProjects()
      setProjects(data)
    } catch (error) {
      console.error('Failed to load projects:', error)
    } finally {
      setLoading(false)
    }
  }

  async function handleCreateProject() {
    if (!newProjectName.trim()) return

    try {
      await ProjectsApi.createProject(newProjectName)
      setNewProjectName('')
      await loadProjects()
    } catch (error) {
      console.error('Failed to create project:', error)
    }
  }

  async function handleDeleteProject(id: string) {
    try {
      await ProjectsApi.deleteProject(id)
      await loadProjects()
    } catch (error) {
      console.error('Failed to delete project:', error)
    }
  }

  return (
    <PageContent
      title="项目管理"
      description="创建和管理你的创意项目"
      icon="📁"
    >
      {/* Create Project Form */}
      <Card title="创建新项目" icon="✨">
        <div className="flex gap-3 items-center">
          <input
            type="text"
            value={newProjectName}
            onChange={(e) => setNewProjectName(e.target.value)}
            placeholder="输入项目名称..."
            className="flex-1"
            onKeyDown={(e) => e.key === 'Enter' && handleCreateProject()}
          />
          <button onClick={handleCreateProject} disabled={!newProjectName.trim()}>
            创建项目
          </button>
        </div>
      </Card>

      {/* Projects List */}
      <Card title={`项目列表 (${projects.length})`}>
        {loading ? (
          <div className="loading text-center py-12">加载中...</div>
        ) : projects.length === 0 ? (
          <div className="text-center py-12 text-slate-400">
            <div className="text-6xl mb-4">🎬</div>
            <p className="text-lg">暂无项目</p>
            <p className="text-sm mt-2">在上方面创建你的第一个项目吧！</p>
          </div>
        ) : (
          <ul className="space-y-3">
            {projects.map((project) => (
              <li key={project.id}>
                <div className="project-item group">
                  <div className="flex items-center gap-3 flex-1">
                    <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-white font-bold text-lg flex-shrink-0">
                      {project.name.charAt(0).toUpperCase()}
                    </div>
                    <div className="flex-1 min-w-0">
                      <Link
                        to={`/project/${project.id}`}
                        className="text-slate-100 font-semibold hover:text-indigo-400 transition-colors"
                      >
                        {project.name}
                      </Link>
                      {project.description && (
                        <p className="text-slate-400 text-sm mt-1">{project.description}</p>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={() => handleDeleteProject(project.id)}
                    className="danger opacity-0 group-hover:opacity-100 transition-opacity flex-shrink-0"
                  >
                    删除
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </Card>
    </PageContent>
  )
}
