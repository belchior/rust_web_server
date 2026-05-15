import Anchor from '../Anchor/Anchor'
import Container from '../Container/Container'
import GithubIcon from '../Icon/Github'
import './Header.css'

type HeaderProps = {
  login?: string
}
export default function Header(props: HeaderProps) {
  const { login } = props
  return (
    <Container className="Header">
      <Anchor href="/" title="Go to home" ><GithubIcon /></Anchor>
      {login && <Anchor href={`/${login}`} decoration="button">{login}</Anchor>}
    </Container>
  )
};

