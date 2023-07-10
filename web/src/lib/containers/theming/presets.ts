import Color from 'color'
import type { Theme, ThemeCollection } from '$lib/containers/theming/utils/theme'

const presets: ThemeCollection = new Map<string, Theme>([
  [
    'pikachu.dark',
    {
      name: 'Pikachu Dark',
      colors: {
        primary: {
          main: Color('#333'),
          light: Color('#ccc'),
          dark: Color('#000'),
          contractText: Color('#fff')
        },
        secondary: {
          main: Color('#f00'),
          light: Color('#f69'),
          dark: Color('#c00'),
          contractText: Color('#fff')
        }
      }
    }
  ],
  [
    'pikachu.light',
    {
      name: 'Pikachu Light',
      colors: {
        primary: {
          main: Color('#ff0'),
          light: Color('#ff9'),
          dark: Color('#fc0'),
          contractText: Color('#fff')
        },
        secondary: {
          main: Color('#f00'),
          light: Color('#f69'),
          dark: Color('#c00'),
          contractText: Color('#fff')
        }
      }
    }
  ]
])

export default presets
