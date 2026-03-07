import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { ProjectsApi } from '../api'
import type { Project } from '../types'

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
    <div className="container">
      <div className="flex justify-between items-center mb-6">
        <h1>PopIdea</h1>
        <Link to="/tasks" className="text-blue-500 hover:text-blue-700">
          Task Monitor
        </Link>
      </div>
      <p>Welcome to PopIdea - Your creative content creation platform</p>

      <div className="project-form">
        <input
          type="text"
          value={newProjectName}
          onChange={(e) => setNewProjectName(e.target.value)}
          placeholder="Enter project name..."
        />
        <button onClick={handleCreateProject}>Create Project</button>
      </div>

      {loading ? (
        <p>Loading projects...</p>
      ) : (
        <div className="projects-list">
          <h2>Projects ({projects.length})</h2>
          {projects.length === 0 ? (
            <p>No projects yet. Create your first project above!</p>
          ) : (
            <ul>
              {projects.map((project) => (
                <li key={project.id}>
                  <div className="project-item">
                    <div>
                      <strong>{project.name}</strong>
                      {project.description && <p>{project.description}</p>}
                    </div>
                    <button onClick={() => handleDeleteProject(project.id)}>
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
