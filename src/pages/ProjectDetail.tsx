import { useState, useEffect } from 'react'
import { useParams } from 'react-router-dom'
import { EpisodesApi } from '../api'
import type { Episode } from '../types'

export function ProjectDetail() {
  const { projectId } = useParams<{ projectId: string }>()
  const [episodes, setEpisodes] = useState<Episode[]>([])
  const [loading, setLoading] = useState(true)
  const [newEpisodeName, setNewEpisodeName] = useState('')

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
      await loadEpisodes()
    } catch (error) {
      console.error('Failed to delete episode:', error)
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
      </div>

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
                    <div>
                      <strong>{episode.name}</strong>
                      {episode.content && <p>{episode.content}</p>}
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
    </div>
  )
}
