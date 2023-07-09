import type Color from 'color'

interface Theme {
  name: 'light' | 'dark'
  palette: Palette
}

interface Palette {
  primary: PaletteColor
  secondary: PaletteColor
}

interface PaletteColor {
  main: Color
  light?: Color
  dark?: Color
}

export default Theme
