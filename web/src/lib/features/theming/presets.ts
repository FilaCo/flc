import Color from 'color'
import type { Theme, ThemeCollection } from '$lib/features/theming/Theme'

const pikachuLight: Theme = {
  name: 'Pikachu Light',
  palette: {
    primary: Color('#ff0'),
    secondary: Color('#f00')
  }
}

const pikachuDark: Theme = {
  name: 'Pikachu Dark',
  palette: {
    primary: Color('#000'),
    secondary: Color('#f00')
  }
}

export const presets: ThemeCollection = {
  default: pikachuLight,
  themes: [pikachuLight, pikachuDark]
}
