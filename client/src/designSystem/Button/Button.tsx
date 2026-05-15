import type { ComponentPropsWithoutRef } from 'react'
import { cls } from '../../util/classNames'
import './Button.css'

type ButtonProps = ComponentPropsWithoutRef<'button'>

export default function Button(props: ButtonProps) {
  const { children, className, ...other } = props

  const classes = cls('Button', className)

  return (
    <button type="button" className={classes} {...other}>{children}</button>
  )
}
