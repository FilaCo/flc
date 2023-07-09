import type Color from 'color'

export type PaletteCollection = Map<string, Palette>

export interface Palette {
  primary: Color
  secondary: Color
}
