import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { api, Project, Episode } from '../api/client'

export default function ProjectDetail() {
  const { id } = useParams<{ id: string }>()
  const navigate = useNavigate()
  const [project, setProject] = useState<Project | null>(null)
  const [episodes, setEpisodes] = useState<Episode[]>([])
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const [showCreateEpisodeModal, setShowCreateEpisodeModal] = useState(false)
  const [newEpisodeName, setNewEpisodeName] = useState('')
  const [newEpisodeContent, setNewEpisodeContent] = useState('')

  const loadProject = async () => {
    if (!id) return

    try {
      setLoading(true)
      setError(null)
      const [projectData, episodesData] = await Promise.all([
        api.projects.get(id),
        api.episodes.list(id)
      ])
      setProject(projectData)
      setEpisodes(episodesData)
    } catch (err) {
      setError(err as string)
    } finally {
      setLoading(false)
    }
  }

  const handleCreateEpisode = async () => {
    if (!id || !newEpisodeName.trim()) return

    try {
      await api.episodes.create(id, newEpisodeName, newEpisodeContent || undefined)
      setNewEpisodeName('')
      setNewEpisodeContent('')
      setShowCreateEpisodeModal(false)
      loadProject()
    } catch (err) {
      setError(err as string)
    }
  }

  const handleDeleteEpisode = async (episodeId: string) => {
    if (!confirm('确定要删除这个剧集吗？')) return

    try {
      await api.episodes.delete(episodeId)
      loadProject()
    } catch (err) {
      setError(err as string)
    }
  }

  useEffect(() => {
    loadProject()
  }, [id])

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-gray-600">加载中...</div>
      </div>
    )
  }

  if (!project) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-red-600">项目未找到</div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-gray-100">
      <div className="container mx-auto px-4 py-8">
        <div className="mb-6">
          <button
            onClick={() => navigate('/')}
            className="text-blue-500 hover:text-blue-600 mb-4 inline-block"
          >
            ← 返回项目列表
          </button>
          <h1 className="text-3xl font-bold text-gray-800">{project.name}</h1>
          {project.description && (
            <p className="text-gray-600 mt-2">{project.description}</p>
          )}
        </div>

        {error && (
          <div className="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
            {error}
          </div>
        )}

        <div className="flex justify-between items-center mb-6">
          <h2 className="text-2xl font-semibold text-gray-700">剧集列表</h2>
          <button
            onClick={() => setShowCreateEpisodeModal(true)}
            className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
          >
            新建剧集
          </button>
        </div>

        {episodes.length === 0 ? (
          <div className="bg-white rounded-lg shadow-md p-8 text-center">
            <p className="text-gray-600 mb-4">还没有剧集，创建一个开始吧！</p>
            <button
              onClick={() => setShowCreateEpisodeModal(true)}
              className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
            >
              创建第一个剧集
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {episodes.map((episode) => (
              <div key={episode.id} className="bg-white rounded-lg shadow-md p-6 hover:shadow-lg transition-shadow">
                <h3 className="text-xl font-semibold text-gray-800 mb-2">{episode.name}</h3>
                {episode.content && (
                  <p className="text-gray-600 mb-4 line-clamp-3">{episode.content}</p>
                )}
                <div className="text-sm text-gray-500 mb-4">
                  创建于: {new Date(episode.created_at).toLocaleDateString()}
                </div>
                <div className="flex gap-2">
                  <button
                    onClick={() => navigate(`/project/${project.id}/episode/${episode.id}`)}
                    className="flex-1 bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
                  >
                    打开
                  </button>
                  <button
                    onClick={() => handleDeleteEpisode(episode.id)}
                    className="bg-red-500 hover:bg-red-600 text-white font-bold py-2 px-4 rounded"
                  >
                    删除
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}

        {showCreateEpisodeModal && (
          <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center">
            <div className="bg-white rounded-lg shadow-xl p-6 w-full max-w-md">
              <h2 className="text-2xl font-bold text-gray-800 mb-4">创建新剧集</h2>
              <div className="mb-4">
                <label className="block text-gray-700 font-bold mb-2">剧集名称</label>
                <input
                  type="text"
                  value={newEpisodeName}
                  onChange={(e) => setNewEpisodeName(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  placeholder="输入剧集名称"
                />
              </div>
              <div className="mb-4">
                <label className="block text-gray-700 font-bold mb-2">内容（可选）</label>
                <textarea
                  value={newEpisodeContent}
                  onChange={(e) => setNewEpisodeContent(e.target.value)}
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                  rows={6}
                  placeholder="输入剧集内容"
                />
              </div>
              <div className="flex gap-2">
                <button
                  onClick={handleCreateEpisode}
                  disabled={!newEpisodeName.trim()}
                  className="flex-1 bg-blue-500 hover:bg-blue-600 disabled:bg-gray-400 text-white font-bold py-2 px-4 rounded"
                >
                  创建
                </button>
                <button
                  onClick={() => setShowCreateEpisodeModal(false)}
                  className="bg-gray-300 hover:bg-gray-400 text-gray-800 font-bold py-2 px-4 rounded"
                >
                  取消
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  )
}
