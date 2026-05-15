import { type PropsWithChildren } from 'react'
import { useParams } from 'react-router'

import Footer from '../Footer/Footer'
import Header from '../Header/Header'
import './App.css'

type AppProps = PropsWithChildren

export default function App(props: AppProps) {
  const { children } = props
  const params = useParams()

  return (
    <div className="App">
      <Header login={params?.login} />
      {children}
      <Footer />
    </div>
  )
}

