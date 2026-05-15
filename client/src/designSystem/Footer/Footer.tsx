import Anchor from '../Anchor/Anchor'
import Container from '../Container/Container'
import './Footer.css'

export default function Footer() {
  return (
    <div className="Footer">
      <Container maxWidth="xl">
        <p className="text">
          This app is part of the project{' '}
          <Anchor href="https://github.com/belchior/rust_web_server" external>
            Rust Web Server
          </Anchor>{' '}
          made by{' '}
          <Anchor href="https://github.com/belchior" external>
            Belchior Oliveira
          </Anchor>
        </p>
      </Container>
    </div>
  )
}
