<script lang="ts">
  import presets from '$lib/utils/theming/presets'
  import {
    themeKey as themeKeyStore,
    themes as themesStore,
    theme as themeStore
  } from '$lib/utils/theming/stores'
  import type { ThemeCollection, ThemeContext } from '$lib/utils/theming/theme'
  import createCssVariables from '$lib/utils/theming/createCssVariables'
  import { onMount, setContext } from 'svelte'

  export let themes: ThemeCollection = presets
  export let themeKey: string | null = null

  themesStore.set(themes)
  const [[fallbackThemeKey, fallbackTheme]] = themes

  if (!themes.has($themeKeyStore)) {
    console.log('wtf')
    console.log($themeKeyStore)
    console.log(themes)
    themeKeyStore.set(fallbackThemeKey)
  }

  $: themeStore.set(themes.get($themeKeyStore) ?? fallbackTheme)
  $: cssVariables = createCssVariables($themeStore)

  const context: ThemeContext = {
    themeKey: themeKeyStore,
    themes: themesStore,
    setTheme: (tKey: string) => {
      if (!themes.has(tKey)) return

      themeKeyStore.set(tKey)
    }
  }

  setContext('theme', context)

  onMount(() => {
    if (themeKey && themes.has(themeKey)) {
      themeKeyStore.set(themeKey)
    } else if (!themes.has($themeKeyStore)) {
      themeKeyStore.set(fallbackThemeKey)
    }
  })
</script>

<div class={`theme-provider ${$$props.class ?? ''}`.trim()} style={cssVariables}>
  <slot />
</div>

<style>
  .theme-provider {
    display: contents;
  }
</style>
