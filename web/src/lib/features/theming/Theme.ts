import type Color from 'color'
import type { Writable } from 'svelte/store'

export interface ThemeCollection {
  default: Theme
  themes: Theme[]
}

export interface Theme {
  name: string
  palette: Palette
}

export interface Palette {
  primary: Color
  secondary: Color
}

export interface ThemeContext {
  currentTheme: Writable<Theme>
  availableThemes: ThemeCollection
  setCurrentTheme: (themeName: string) => void
}
