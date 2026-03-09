import { useState, useEffect } from 'react'
import { ProjectsApi } from '../api'
import type { Project } from '../types'
import { PageContent, Card } from '../components/layout'

export function AssetsPage() {
  const [projects, setProjects] = useState<Project[]>([])
  const [loading, setLoading] = useState(true)

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

  return (
    <PageContent
      title="资产管理"
      description="管理你的项目资源文件"
      icon="🎨"
    >
      <Card title={`可用项目 (${projects.length})`}>
        {loading ? (
          <div className="loading text-center py-12">加载中...</div>
        ) : projects.length === 0 ? (
          <div className="text-center py-12 text-slate-400">
            <div className="text-6xl mb-4">📁</div>
            <p className="text-lg">暂无项目</p>
            <p className="text-sm mt-2">请先创建项目以管理资产</p>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {projects.map((project) => (
              <div
                key={project.id}
                className="p-4 bg-slate-900/50 rounded-lg border border-slate-700 hover:border-indigo-500/50 transition-all"
              >
                <div className="flex items-center gap-3 mb-3">
                  <div className="w-12 h-12 rounded-lg bg-gradient-to-br from-indigo-500 to-purple-500 flex items-center justify-center text-white font-bold text-xl flex-shrink-0">
                    {project.name.charAt(0).toUpperCase()}
                  </div>
                  <div className="flex-1 min-w-0">
                    <h3 className="text-lg font-semibold text-slate-100 truncate">
                      {project.name}
                    </h3>
                  </div>
                </div>
                <p className="text-sm text-slate-400">
                  点击查看项目资产
                </p>
              </div>
            ))}
          </div>
        )}
      </Card>
    </PageContent>
  )
}
