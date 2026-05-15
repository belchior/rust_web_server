import React, { type PropsWithChildren } from 'react'

type IProps = PropsWithChildren & {
  fallback?: React.ReactNode
}
type IState = {
  hasError: boolean
}

function ErrorView() {
  return (
    <div>
      <h1>Something went wrong.</h1>
      <p>Try refresh the page</p>
    </div>
  )
}

export default class ErrorBoundary extends React.Component<IProps, IState> {
  constructor(props: IProps) {
    super(props)
    this.state = { hasError: false }
  }

  static getDerivedStateFromError() {
    return { hasError: true }
  }

  render() {
    if (this.state.hasError === false) return this.props.children
    if (this.props.fallback) return this.props.fallback
    return <ErrorView />
  }
}
