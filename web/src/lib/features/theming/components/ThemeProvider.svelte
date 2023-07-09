<script lang="ts">
  import { onMount, setContext } from 'svelte'
  import type { Theme, ThemeContext } from '$lib/features/theming/Theme'
  import type Color from 'color'
  import { presets } from '$lib/features/theming/presets'
  import { writable } from 'svelte/store'
  import { browser } from '$app/environment'

  export let themeCollection = presets

  const initThemeName = browser
    ? window.localStorage.getItem('theme') ?? presets.default.name
    : presets.default.name

  const theme = writable(presets.themes.find((t) => t.name === initThemeName) ?? presets.default)
  theme.subscribe((t) => browser && window.localStorage.setItem('theme', t.name))

  const context: ThemeContext = {
    currentTheme: theme,
    availableThemes: themeCollection,
    setCurrentTheme: (themeName: string) => {
      const newTheme = themeCollection.themes.find((t) => t.name === themeName)
      if (!newTheme) return

      theme.set(newTheme)
      setRootColors($theme)
    }
  }

  setContext('theme', context)

  onMount(() => {
    setRootColors($theme)
  })

  const setRootColors = (theme: Theme) => {
    for (const [prop, value] of Object.entries(theme.palette)) {
      const colorStringName = `--theme-${prop}`
      const color = value as Color
      document.documentElement.style.setProperty(colorStringName, color.hex())
    }
    document.documentElement.style.setProperty('--theme-name', theme.name)
  }
</script>

<slot />
