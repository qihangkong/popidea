import { useState, useEffect } from 'react'
import { useParams, Link } from 'react-router-dom'
import { EpisodesApi } from '../api'
import type { Episode } from '../types'

export function ProjectDetail() {
  const { projectId } = useParams<{ projectId: string }>()
  const [episodes, setEpisodes] = useState<Episode[]>([])
  const [loading, setLoading] = useState(true)
  const [newEpisodeName, setNewEpisodeName] = useState('')
  const [selectedEpisode, setSelectedEpisode] = useState<Episode | null>(null)
  const [showImport, setShowImport] = useState(false)
  const [importName, setImportName] = useState('')
  const [importContent, setImportContent] = useState('')

  useEffect(() => {
    if (projectId) {
      loadEpisodes()
    }
  }, [projectId])

  async function loadEpisodes() {
    if (!projectId) return

    try {
      setLoading(true)
      const data = await EpisodesApi.getEpisodes(projectId)
      setEpisodes(data)
    } catch (error) {
      console.error('Failed to load episodes:', error)
    } finally {
      setLoading(false)
    }
  }

  async function handleCreateEpisode() {
    if (!projectId || !newEpisodeName.trim()) return

    try {
      await EpisodesApi.createEpisode(projectId, newEpisodeName)
      setNewEpisodeName('')
      await loadEpisodes()
    } catch (error) {
      console.error('Failed to create episode:', error)
    }
  }

  async function handleDeleteEpisode(id: string) {
    try {
      await EpisodesApi.deleteEpisode(id)
      if (selectedEpisode?.id === id) {
        setSelectedEpisode(null)
      }
      await loadEpisodes()
    } catch (error) {
      console.error('Failed to delete episode:', error)
    }
  }

  function handleSelectEpisode(episode: Episode) {
    setSelectedEpisode(episode)
  }

  function handleShowImport() {
    setShowImport(true)
    setImportName('')
    setImportContent('')
  }

  function handleHideImport() {
    setShowImport(false)
  }

  async function handleImportEpisode() {
    if (!projectId || !importName.trim() || !importContent.trim()) return

    try {
      const episode = await EpisodesApi.importEpisode(projectId, importName, importContent)
      setShowImport(false)
      setImportName('')
      setImportContent('')
      await loadEpisodes()
      handleSelectEpisode(episode)
    } catch (error) {
      console.error('Failed to import episode:', error)
    }
  }

  function handleFileSelect(event: React.ChangeEvent<HTMLInputElement>) {
    const file = event.target.files?.[0]
    if (file) {
      const reader = new FileReader()
      reader.onload = (e) => {
        const content = e.target?.result as string
        setImportContent(content)
        if (!importName) {
          setImportName(file.name.replace(/\.[^/.]+$/, ''))
        }
      }
      reader.readAsText(file)
    }
  }

  return (
    <div className="container">
      {/* Header */}
      <div className="mb-6">
        <Link
          to="/"
          className="inline-flex items-center gap-2 text-slate-400 hover:text-slate-200 transition-colors mb-4"
        >
          <span>←</span>
          <span>Back to Projects</span>
        </Link>
        <h1 className="text-3xl font-bold">Project: {projectId}</h1>
      </div>

      {/* Create Episode Form */}
      <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl p-6 mb-6 border border-slate-700 shadow">
        <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
          <span className="text-2xl">🎬</span>
          Create New Episode
        </h2>
        <div className="flex gap-3 items-center flex-wrap">
          <input
            type="text"
            value={newEpisodeName}
            onChange={(e) => setNewEpisodeName(e.target.value)}
            placeholder="Enter episode name..."
            className="flex-1 min-w-[200px]"
            onKeyDown={(e) => e.key === 'Enter' && handleCreateEpisode()}
          />
          <button onClick={handleCreateEpisode} disabled={!newEpisodeName.trim()}>
            Create Episode
          </button>
          <button onClick={handleShowImport} className="secondary">
            Import Script
          </button>
        </div>
      </div>

      {/* Import Modal */}
      {showImport && (
        <div className="import-modal" onClick={handleHideImport}>
          <div className="import-modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-2xl font-bold">Import Script</h2>
              <button onClick={handleHideImport} className="secondary p-2">
                ✕
              </button>
            </div>
            <div className="import-form">
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Episode Name
                </label>
                <input
                  type="text"
                  value={importName}
                  onChange={(e) => setImportName(e.target.value)}
                  placeholder="Episode name..."
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Upload File
                </label>
                <input
                  type="file"
                  accept=".txt,.md"
                  onChange={handleFileSelect}
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-slate-300 mb-2">
                  Script Content
                </label>
                <textarea
                  value={importContent}
                  onChange={(e) => setImportContent(e.target.value)}
                  placeholder="Or paste your script content here..."
                  rows={10}
                />
              </div>
            </div>
            <div className="import-actions">
              <button onClick={handleImportEpisode} disabled={!importName.trim() || !importContent.trim()}>
                Import
              </button>
              <button onClick={handleHideImport} className="secondary">
                Cancel
              </button>
            </div>
          </div>
        </div>
      )}

      {/* Episodes List */}
      <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl p-6 mb-6 border border-slate-700 shadow">
        <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
          <span className="text-2xl">📝</span>
          Episodes ({episodes.length})
        </h2>

        {loading ? (
          <div className="loading text-center py-12">
            Loading episodes...
          </div>
        ) : episodes.length === 0 ? (
          <div className="text-center py-12 text-slate-400">
            <div className="text-6xl mb-4">🎭</div>
            <p className="text-lg">No episodes yet</p>
            <p className="text-sm mt-2">Create your first episode above to get started!</p>
          </div>
        ) : (
          <ul className="space-y-3">
            {episodes.map((episode) => (
              <li key={episode.id}>
                <div className="episode-item group">
                  <div
                    onClick={() => handleSelectEpisode(episode)}
                    className="flex items-center gap-3 flex-1 cursor-pointer"
                  >
                    <div className="w-10 h-10 rounded-lg bg-gradient-to-br from-emerald-500 to-teal-500 flex items-center justify-center text-white font-bold text-lg">
                      {episode.name.charAt(0).toUpperCase()}
                    </div>
                    <div className="flex-1">
                      <strong className="text-slate-100 group-hover:text-emerald-400 transition-colors">
                        {episode.name}
                      </strong>
                      {episode.content && (
                        <p className="text-slate-400 text-sm mt-1">
                          {episode.content.substring(0, 100)}...
                        </p>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={() => handleDeleteEpisode(episode.id)}
                    className="danger opacity-0 group-hover:opacity-100 transition-opacity"
                  >
                    Delete
                  </button>
                </div>
              </li>
            ))}
          </ul>
        )}
      </div>

      {/* Episode Editor */}
      {selectedEpisode && (
        <div className="episode-editor">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-xl font-semibold flex items-center gap-2">
              <span className="text-2xl">✏️</span>
              Editing: {selectedEpisode.name}
            </h2>
            <button onClick={() => setSelectedEpisode(null)} className="secondary">
              Close
            </button>
          </div>
          <textarea
            value={selectedEpisode.content || ''}
            onChange={(e) => {
              const updated = { ...selectedEpisode, content: e.target.value }
              setSelectedEpisode(updated)
            }}
            rows={20}
            className="editor-textarea"
            placeholder="Start writing your script..."
          />
        </div>
      )}
    </div>
  )
}
