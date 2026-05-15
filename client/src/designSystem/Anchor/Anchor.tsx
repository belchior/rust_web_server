import { type ComponentPropsWithoutRef } from 'react'
import { cls } from '../../util/classNames'
import { Link } from 'react-router'
import './Anchor.css'

type AnchorProps = ComponentPropsWithoutRef<'a'> & {
  decoration?: 'primary' | 'secondary' | 'contained' | 'button'
  external?: boolean
  href: string
}

export default function Anchor(props: AnchorProps) {
  const { children, className = '', href, external = false, decoration = 'primary', ...other } = props
  const classes = cls('Anchor', decoration, className)

  if (external) {
    return <a href={href} className={classes} {...other}>{children}</a>
  }

  return (
    <Link to={href} className={classes} {...other}>
      {children}
    </Link>
  )
};

