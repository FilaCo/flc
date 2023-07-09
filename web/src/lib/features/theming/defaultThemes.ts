import type Theme from '$lib/features/theming/theme'
import Color from 'color'

const defaultThemes: Theme[] = [
  {
    name: 'light',
    palette: {
      primary: {
        main: Color('#ff0')
      },
      secondary: {
        main: Color('#f00')
      }
    }
  },
  {
    name: 'dark',
    palette: {
      primary: {
        main: Color('#ff0')
      },
      secondary: {
        main: Color('#f00')
      }
    }
  }
]
