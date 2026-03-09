import type { ReactNode } from 'react'

interface CardProps {
  children: ReactNode
  title?: string
  icon?: string
  className?: string
}

export function Card({ children, title, icon, className = '' }: CardProps) {
  return (
    <div className={`bg-slate-800/50 backdrop-blur-sm rounded-xl p-6 border border-slate-700 shadow ${className}`}>
      {title && (
        <h2 className="text-xl font-semibold mb-4 flex items-center gap-2">
          {icon && <span className="text-2xl">{icon}</span>}
          {title}
        </h2>
      )}
      {children}
    </div>
  )
}
