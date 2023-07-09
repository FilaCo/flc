<script lang="ts">
  import presets from '$lib/features/theming/presets'
  import {
    paletteName as paletteNameStore,
    palettes as palettesStore,
    palette as paletteStore
  } from '$lib/features/theming/utils/stores'
  import { afterUpdate, onMount, setContext } from 'svelte'
  import type {Palette, PaletteCollection} from '$lib/features/theming/utils/palette'

  export let palettes: PaletteCollection = presets
  export let paletteName: string | undefined = undefined

  palettesStore.set(palettes)

  const [[fallbackPaletteName]] = palettes

  if (!palettes.has($paletteNameStore)) {
    paletteNameStore.set(fallbackPaletteName)
  }

  paletteStore.set(palettes[$paletteNameStore])

  const createCssVariables = (palette: Palette) => {

  }

  setContext('theme', {})

  onMount(() => {
    const savedPaletteName = localStorage && localStorage.getItem('palette')

    if (paletteName && palettes[paletteName]) {
      paletteNameStore.set(paletteName)
    } else if (savedPaletteName && palettes[savedPaletteName]) {
      paletteNameStore.set(savedPaletteName)
    } else {
      paletteNameStore.set(fallbackPaletteName)
    }

    localStorage && localStorage.setItem('palette', $paletteNameStore)
  })

  afterUpdate(() => {
    if (localStorage) localStorage.setItem('palette', $paletteNameStore)
  })
</script>

<div class={`theme-provider ${$$props.class ?? ''}`.trim()} bind:style={}>
  <slot />
</div>

<style>
  .theme-provider {
    display: contents;
  }
</style>
