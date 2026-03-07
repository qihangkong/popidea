import { useState, useEffect } from 'react'
import { TasksApi } from '../api'
import type { Task } from '../types'

export function TaskMonitor() {
  const [tasks, setTasks] = useState<Task[]>([])
  const [loading, setLoading] = useState(true)
  const [autoRefresh, setAutoRefresh] = useState(true)

  useEffect(() => {
    loadTasks()
    
    if (autoRefresh) {
      const interval = setInterval(loadTasks, 2000)
      return () => clearInterval(interval)
    }
  }, [autoRefresh])

  async function loadTasks() {
    try {
      setLoading(true)
      const data = await TasksApi.getTasks()
      setTasks(data)
    } catch (error) {
      console.error('Failed to load tasks:', error)
    } finally {
      setLoading(false)
    }
  }

  function getStatusColor(status: Task['status']): string {
    switch (status) {
      case 'queued':
        return 'bg-yellow-100 text-yellow-800'
      case 'processing':
        return 'bg-blue-100 text-blue-800'
      case 'completed':
        return 'bg-green-100 text-green-800'
      case 'failed':
        return 'bg-red-100 text-red-800'
      default:
        return 'bg-gray-100 text-gray-800'
    }
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString()
  }

  return (
    <div className="container mx-auto p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold">Task Monitor</h1>
        <div className="flex items-center gap-4">
          <label className="flex items-center gap-2">
            <input
              type="checkbox"
              checked={autoRefresh}
              onChange={(e) => setAutoRefresh(e.target.checked)}
              className="w-4 h-4"
            />
            <span>Auto Refresh</span>
          </label>
          <button
            onClick={loadTasks}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
          >
            Refresh
          </button>
        </div>
      </div>

      {loading && tasks.length === 0 ? (
        <p className="text-gray-500">Loading tasks...</p>
      ) : tasks.length === 0 ? (
        <p className="text-gray-500">No tasks yet.</p>
      ) : (
        <div className="space-y-4">
          {tasks.map((task) => (
            <div
              key={task.id}
              className="bg-white rounded-lg shadow p-4 border border-gray-200"
            >
              <div className="flex justify-between items-start mb-3">
                <div className="flex-1">
                  <div className="flex items-center gap-3 mb-2">
                    <h3 className="text-lg font-semibold">{task.type}</h3>
                    <span className={`px-2 py-1 rounded text-xs font-medium ${getStatusColor(task.status)}`}>
                      {task.status}
                    </span>
                  </div>
                  <p className="text-sm text-gray-600">
                    Project: {task.project}
                    {task.episodeId && ` | Episode: ${task.episodeId}`}
                  </p>
                  {task.targetType && task.targetId && (
                    <p className="text-sm text-gray-600">
                      Target: {task.targetType} ({task.targetId})
                    </p>
                  )}
                </div>
                <div className="text-right text-sm text-gray-500">
                  <p>Created: {formatDate(task.createdAt)}</p>
                  {task.startedAt && <p>Started: {formatDate(task.startedAt)}</p>}
                  {task.finishedAt && <p>Finished: {formatDate(task.finishedAt)}</p>}
                </div>
              </div>

              {task.progress > 0 && (
                <div className="mb-3">
                  <div className="flex justify-between text-sm text-gray-600 mb-1">
                    <span>Progress</span>
                    <span>{task.progress}%</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-500 h-2 rounded-full transition-all"
                      style={{ width: `${task.progress}%` }}
                    />
                  </div>
                </div>
              )}

              {task.payload && (
                <details className="mb-2">
                  <summary className="cursor-pointer text-sm text-gray-600 hover:text-gray-800">
                    Payload
                  </summary>
                  <pre className="mt-2 p-2 bg-gray-50 rounded text-xs overflow-auto">
                    {JSON.stringify(task.payload, null, 2)}
                  </pre>
                </details>
              )}

              {task.result && (
                <details className="mb-2">
                  <summary className="cursor-pointer text-sm text-gray-600 hover:text-gray-800">
                    Result
                  </summary>
                  <pre className="mt-2 p-2 bg-gray-50 rounded text-xs overflow-auto">
                    {JSON.stringify(task.result, null, 2)}
                  </pre>
                </details>
              )}

              {task.errorMessage && (
                <div className="mt-2 p-2 bg-red-50 border border-red-200 rounded">
                  <p className="text-sm text-red-800 font-medium">Error:</p>
                  <p className="text-sm text-red-600">{task.errorMessage}</p>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
