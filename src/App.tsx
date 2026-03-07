import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom'
import { HomePage, ProjectDetail, TaskMonitor } from './pages'

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<HomePage />} />
        <Route path="/project/:projectId" element={<ProjectDetail />} />
        <Route path="/tasks" element={<TaskMonitor />} />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </Router>
  )
}

export default App
