import { type PropsWithChildren } from 'react'
import { cls } from '../../util/classNames'
import './Label.css'

type LabelProps = PropsWithChildren & {
  className?: string
}

export default function Label(props: LabelProps) {
  const { children, className = '', ...typographyProps } = props
  const classes = cls('Label', className)

  return (
    <p className={classes} {...typographyProps}>{children}</p>
  )
}

