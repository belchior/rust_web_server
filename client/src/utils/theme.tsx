import { createTheme } from '@material-ui/core/styles';


export const theme = createTheme({
  palette: {
    action: {
      disabled: 'rgba(255, 255, 255, 0.5)',
    },
    background: {
      default: 'rgb(36, 41, 46)',
    },
    divider: 'rgb(85, 85, 85)',
    primary: {
      main: '#42a5f5',
    },
    text: {
      primary: 'rgb(255, 255, 255)',
    },
  },
  shape: {
    borderRadius: 3,
  },
  overrides: {
    MuiTypography: {
      h1: {
        fontSize: '2rem',
        fontWeight: 400,
        lineHeight: '3rem'
      },
      h2: {
        fontSize: '1.4rem',
        fontWeight: 400,
        lineHeight: '1.8rem'
      },
    },
  }
});
