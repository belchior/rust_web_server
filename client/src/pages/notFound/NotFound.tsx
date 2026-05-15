import Container from '../../designSystem/Container/Container'
import Title from '../../designSystem/Title/Title'
import './NotFound.css'

export default function NotFound() {
  return (
    <main className="NotFound">
      <Container maxWidth="lg">
        <Title>404 - Not found</Title>
        <p>
          This is not the web page you are looking for.
        </p>
      </Container>
    </main>
  )
}

