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

  function getStatusColor(status: Task['status']): { bg: string; text: string; icon: string } {
    switch (status) {
      case 'queued':
        return { bg: 'bg-amber-500/10', text: 'text-amber-400', icon: '⏳' }
      case 'processing':
        return { bg: 'bg-blue-500/10', text: 'text-blue-400', icon: '🔄' }
      case 'completed':
        return { bg: 'bg-emerald-500/10', text: 'text-emerald-400', icon: '✅' }
      case 'failed':
        return { bg: 'bg-red-500/10', text: 'text-red-400', icon: '❌' }
      default:
        return { bg: 'bg-slate-500/10', text: 'text-slate-400', icon: '❓' }
    }
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString()
  }

  return (
    <div className="container">
      <div className="mb-8">
        <h1 className="text-4xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">
          Task Monitor
        </h1>
        <p className="text-slate-400 mt-2">Monitor and track your background tasks</p>
      </div>

      <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl p-6 mb-8 border border-slate-700 shadow">
        <div className="flex flex-wrap items-center justify-between gap-4">
          <div className="flex items-center gap-4">
            <label className="flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                checked={autoRefresh}
                onChange={(e) => setAutoRefresh(e.target.checked)}
                className="w-5 h-5 rounded border-slate-600 bg-slate-700 text-indigo-500 focus:ring-indigo-500 focus:ring-offset-0"
              />
              <span className="text-slate-200">Auto Refresh (2s)</span>
            </label>
          </div>
          <div className="flex items-center gap-3">
            <button onClick={loadTasks} className="secondary">
              Refresh
            </button>
            <div className="text-slate-400">
              Total: <span className="text-indigo-400 font-semibold">{tasks.length}</span>
            </div>
          </div>
        </div>
      </div>

      <div className="bg-slate-800/50 backdrop-blur-sm rounded-xl p-6 border border-slate-700 shadow">
        {loading && tasks.length === 0 ? (
          <div className="loading text-center py-12">Loading tasks...</div>
        ) : tasks.length === 0 ? (
          <div className="text-center py-12 text-slate-400">
            <div className="text-6xl mb-4">📋</div>
            <p className="text-lg">No tasks yet</p>
            <p className="text-sm mt-2">Tasks will appear here when you create content</p>
          </div>
        ) : (
          <div className="space-y-4">
            {tasks.map((task) => {
              const statusStyle = getStatusColor(task.status)
              return (
                <div
                  key={task.id}
                  className="bg-slate-900/50 rounded-lg p-5 border border-slate-700 hover:border-slate-600 transition-all"
                >
                  <div className="flex flex-col lg:flex-row lg:justify-between lg:items-start gap-4 mb-4">
                    <div className="flex-1">
                      <div className="flex items-center gap-3 mb-3">
                        <span className="text-2xl">{statusStyle.icon}</span>
                        <h3 className="text-lg font-semibold text-slate-100">{task.type}</h3>
                        <span className={`px-3 py-1 rounded-full text-xs font-semibold ${statusStyle.bg} ${statusStyle.text}`}>
                          {task.status}
                        </span>
                      </div>
                      <div className="space-y-1 text-sm text-slate-400">
                        <p>
                          <span className="text-slate-500">Project:</span>{' '}
                          <span className="text-slate-200">{task.project}</span>
                          {task.episodeId && (
                            <>
                              {' | '}
                              <span className="text-slate-500">Episode:</span>{' '}
                              <span className="text-slate-200">{task.episodeId}</span>
                            </>
                          )}
                        </p>
                        {task.targetType && task.targetId && (
                          <p>
                            <span className="text-slate-500">Target:</span>{' '}
                            <span className="text-slate-200">{task.targetType}</span>{' '}
                            <span className="text-slate-500">({task.targetId})</span>
                          </p>
                        )}
                      </div>
                    </div>
                    <div className="text-sm text-slate-400 min-w-fit">
                      <div className="space-y-1">
                        <p>
                          <span className="text-slate-500">Created:</span>{' '}
                          <span className="text-slate-200">{formatDate(task.createdAt)}</span>
                        </p>
                        {task.startedAt && (
                          <p>
                            <span className="text-slate-500">Started:</span>{' '}
                            <span className="text-slate-200">{formatDate(task.startedAt)}</span>
                          </p>
                        )}
                        {task.finishedAt && (
                          <p>
                            <span className="text-slate-500">Finished:</span>{' '}
                            <span className="text-slate-200">{formatDate(task.finishedAt)}</span>
                          </p>
                        )}
                      </div>
                    </div>
                  </div>

                  {task.progress > 0 && (
                    <div className="mb-4">
                      <div className="flex justify-between text-sm text-slate-400 mb-2">
                        <span>Progress</span>
                        <span className="text-indigo-400 font-semibold">{task.progress}%</span>
                      </div>
                      <div className="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
                        <div
                          className="bg-gradient-to-r from-indigo-500 to-purple-500 h-full rounded-full transition-all duration-300"
                          style={{ width: `${task.progress}%` }}
                        />
                      </div>
                    </div>
                  )}

                  {task.payload && (
                    <details className="mb-3 group">
                      <summary className="cursor-pointer text-sm text-slate-400 hover:text-slate-200 transition-colors flex items-center gap-2">
                        <span>Payload</span>
                        <span className="text-slate-500 group-open:rotate-90 transition-transform">▶</span>
                      </summary>
                      <div className="mt-2 p-3 bg-slate-950/50 rounded border border-slate-700 overflow-auto">
                        <pre className="text-xs text-slate-300 font-mono whitespace-pre-wrap">
                          {JSON.stringify(task.payload, null, 2)}
                        </pre>
                      </div>
                    </details>
                  )}

                  {task.result && (
                    <details className="mb-3 group">
                      <summary className="cursor-pointer text-sm text-slate-400 hover:text-slate-200 transition-colors flex items-center gap-2">
                        <span>Result</span>
                        <span className="text-slate-500 group-open:rotate-90 transition-transform">▶</span>
                      </summary>
                      <div className="mt-2 p-3 bg-slate-950/50 rounded border border-slate-700 overflow-auto">
                        <pre className="text-xs text-slate-300 font-mono whitespace-pre-wrap">
                          {JSON.stringify(task.result, null, 2)}
                        </pre>
                      </div>
                    </details>
                  )}

                  {task.errorMessage && (
                    <div className="mt-2 p-4 bg-red-500/10 border border-red-500/30 rounded">
                      <p className="text-sm text-red-400 font-medium mb-1">Error:</p>
                      <p className="text-sm text-red-300">{task.errorMessage}</p>
                    </div>
                  )}
                </div>
              )
            })}
          </div>
        )}
      </div>
    </div>
  )
}
