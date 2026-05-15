import type { PropsWithChildren } from 'react'
import { cls } from '../../util/classNames'
import './Container.css'

type ContainerProps = PropsWithChildren & {
  className?: string,
  maxWidth?: 'xl' | 'xs' | 'sm' | 'md' | 'lg',
}
export default function Container(props: ContainerProps) {
  const { children, className, maxWidth } = props

  const classes = cls(className, 'Container', maxWidth)

  return (
    <div className={classes}>{children}</div>
  )
}
