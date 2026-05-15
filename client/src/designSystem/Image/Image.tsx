import React, { type ComponentPropsWithoutRef, type JSX } from 'react'

import { cls } from '../../util/classNames'
import './Image.css'

type IFallbackProps = {
  alt: string
  className?: string
  decoration?: 'rounded' | 'circle'
  height?: number
  width?: number
}

export function Fallback(props: IFallbackProps) {
  const { alt, className, decoration, width = 32, height = 32 } = props
  const maxLength = 200
  const fontSize = Math.min(maxLength, Number(width), Number(height)) / 2
  const styles = { width, height, fontSize }
  const classes = cls('Fallback', decoration, className)

  return <div className={classes} style={styles}>{alt[0]}</div>
}

type ImageProps = ComponentPropsWithoutRef<'img'> & {
  alt: string
  decoration?: 'rounded' | 'circle'
  fallback?: JSX.Element
  height?: number
  src: string
  width?: number
}

export default function Image(props: ImageProps) {
  const { alt, className, decoration, fallback = <Fallback {...props} />, src, ...other } = props
  const [error, setError] = React.useState(false)
  const handleError = () => setError(() => true)
  const classes = cls('Image', decoration, className)

  return error
    ? fallback
    : <img {...other} className={classes} src={src} alt={alt} onError={handleError} />
}

