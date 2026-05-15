import type { PropsWithChildren } from 'react'

type TitleProps = PropsWithChildren & {
  className?: string
  component?: string
  variant?: 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6'
}

export default function Title(props: TitleProps) {
  const { children, variant = 'h1', ...typographyProps } = props
  const Header = variant

  return (
    <Header {...typographyProps}>
      {children}
    </Header>
  )
}

