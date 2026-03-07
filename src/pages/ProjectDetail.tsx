import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
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
      <h1>Project: {projectId}</h1>

      <div className="episode-form">
        <input
          type="text"
          value={newEpisodeName}
          onChange={(e) => setNewEpisodeName(e.target.value)}
          placeholder="Enter episode name..."
        />
        <button onClick={handleCreateEpisode}>Create Episode</button>
        <button onClick={handleShowImport}>Import Script</button>
      </div>

      {showImport && (
        <div className="import-modal">
          <div className="import-modal-content">
            <h2>Import Script</h2>
            <div className="import-form">
              <input
                type="text"
                value={importName}
                onChange={(e) => setImportName(e.target.value)}
                placeholder="Episode name..."
              />
              <input
                type="file"
                accept=".txt,.md"
                onChange={handleFileSelect}
              />
              <textarea
                value={importContent}
                onChange={(e) => setImportContent(e.target.value)}
                placeholder="Or paste your script content here..."
                rows={10}
              />
            </div>
            <div className="import-actions">
              <button onClick={handleImportEpisode}>Import</button>
              <button onClick={handleHideImport}>Cancel</button>
            </div>
          </div>
        </div>
      )}

      {loading ? (
        <p>Loading episodes...</p>
      ) : (
        <div className="episodes-list">
          <h2>Episodes ({episodes.length})</h2>
          {episodes.length === 0 ? (
            <p>No episodes yet. Create your first episode above!</p>
          ) : (
            <ul>
              {episodes.map((episode) => (
                <li key={episode.id}>
                  <div className="episode-item">
                    <div onClick={() => handleSelectEpisode(episode)}>
                      <strong>{episode.name}</strong>
                      {episode.content && <p>{episode.content.substring(0, 100)}...</p>}
                    </div>
                    <button onClick={() => handleDeleteEpisode(episode.id)}>
                      Delete
                    </button>
                  </div>
                </li>
              ))}
            </ul>
          )}
        </div>
      )}

      {selectedEpisode && (
        <div className="episode-editor">
          <h2>Editing: {selectedEpisode.name}</h2>
          <textarea
            value={selectedEpisode.content || ''}
            onChange={(e) => {
              const updated = { ...selectedEpisode, content: e.target.value }
              setSelectedEpisode(updated)
            }}
            rows={20}
            className="editor-textarea"
          />
          <div className="editor-actions">
            <button onClick={() => setSelectedEpisode(null)}>Close</button>
          </div>
        </div>
      )}
    </div>
  )
}
