import { type PropsWithChildren } from 'react'
import './Language.css'

type LanguageProps = PropsWithChildren & {
  color: string
}

export default function Language(props: LanguageProps) {
  const { children, color } = props
  return (
    <span className="Language">
      <span className="circle" style={{ backgroundColor: color }} />
      <span>{children}</span>
    </span>
  )
}
