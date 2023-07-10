import { type Writable, writable } from 'svelte/store'
import type { Theme, ThemeCollection } from '$lib/containers/theming/utils/theme'
import { persistedStorage } from '$lib/utils'

export const themeKey: Writable<string> = persistedStorage<string>('theme')
export const theme: Writable<Theme> = writable()
export const themes: Writable<ThemeCollection> = writable()
