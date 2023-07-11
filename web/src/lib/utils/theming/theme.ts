import type Color from 'color'
import type { Writable } from 'svelte/store'

export type ThemeCollection = Map<string, Theme>

export interface Theme {
  name: string
  colors: { primary: ThemeColor; secondary: ThemeColor }
}

export interface ThemeColor {
  main: Color
  light: Color
  dark: Color
  contractText: Color
}

export interface ThemeContext {
  themeKey: Writable<string>
  themes: Writable<ThemeCollection>
  setTheme: (tKey: string) => void
}
