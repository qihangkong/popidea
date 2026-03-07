import { useState } from 'react'

interface TextEditorProps {
  value: string
  onChange: (value: string) => void
  placeholder?: string
}

export function TextEditor({ value, onChange, placeholder }: TextEditorProps) {
  const [localValue, setLocalValue] = useState(value)

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    const newValue = e.target.value
    setLocalValue(newValue)
    onChange(newValue)
  }

  return (
    <div className="text-editor">
      <textarea
        value={localValue}
        onChange={handleChange}
        placeholder={placeholder || 'Enter your content here...'}
        className="text-editor-textarea"
        rows={20}
      />
    </div>
  )
}
