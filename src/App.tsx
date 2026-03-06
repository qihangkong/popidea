import { useState } from 'react'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="min-h-screen bg-gray-100">
      <div className="container mx-auto px-4 py-8">
        <h1 className="text-4xl font-bold text-gray-800 mb-4">
          NoIdea AI Video Studio
        </h1>
        <p className="text-gray-600 mb-8">
          Transform your novels into animated videos with AI
        </p>
        
        <div className="bg-white rounded-lg shadow-md p-6">
          <h2 className="text-2xl font-semibold text-gray-700 mb-4">
            Getting Started
          </h2>
          <p className="text-gray-600 mb-4">
            Welcome to NoIdea! This is a local-first AI video creation tool.
          </p>
          <button
            onClick={() => setCount(count + 1)}
            className="bg-blue-500 hover:bg-blue-600 text-white font-bold py-2 px-4 rounded"
          >
            Clicked {count} times
          </button>
        </div>
      </div>
    </div>
  )
}

export default App
