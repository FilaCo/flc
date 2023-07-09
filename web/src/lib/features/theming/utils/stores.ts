import { type Writable, writable } from 'svelte/store'
import type { Palette, PaletteCollection } from '$lib/features/theming/utils/palette'

export const paletteName: Writable<string> = writable()
export const palette: Writable<Palette> = writable()
export const palettes: Writable<PaletteCollection> = writable()
