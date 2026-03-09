import type { ReactNode } from 'react'

interface PageContentProps {
  children: ReactNode
  title?: string
  description?: string
  icon?: string
}

export function PageContent({ children, title, description, icon }: PageContentProps) {
  return (
    <div className="p-6 max-w-7xl mx-auto">
      {(title || description) && (
        <div className="mb-8">
          {icon && (
            <div className="text-4xl mb-2">{icon}</div>
          )}
          {title && (
            <h1 className="text-4xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">
              {title}
            </h1>
          )}
          {description && (
            <p className="text-slate-400 mt-2">{description}</p>
          )}
        </div>
      )}
      {children}
    </div>
  )
}
