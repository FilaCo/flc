import Color from 'color'
import type { Palette, PaletteCollection } from '$lib/features/theming/utils/palette'

const presets: PaletteCollection = new Map<string, Palette>([
  ['Pikachu Dark', { primary: Color('#000'), secondary: Color('#f00') }],
  ['Pikachu Light', { primary: Color('#ff0'), secondary: Color('#f00') }]
])

export default presets
