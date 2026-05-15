import Anchor from '../../designSystem/Anchor/Anchor'
import Title from '../../designSystem/Title/Title'
import './Home.css'

export default function Home() {
  return (
    <main className="Home">
      <Title>
        Hi <span role="img" aria-label="hi">👋</span> friend!
      </Title>
      <p>
        This app will be better if you choose a user.
        Try <Anchor href="/belchior">belchior</Anchor>
      </p>
    </main>
  )
}
