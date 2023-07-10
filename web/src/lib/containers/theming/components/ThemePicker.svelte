<script lang="ts">
  import { getContext } from 'svelte'
  import type { ThemeContext } from '$lib/containers/theming/utils/theme'

  const { themeKey, themes, setTheme } = getContext<ThemeContext>('theme')

  $: selected = $themeKey
</script>

<select
  class={`theme-picker ${$$props.class ?? ''}`.trim()}
  bind:value={selected}
  on:change={() => setTheme(selected)}
>
  {#each $themes as [key, palette]}
    <option value={key}>{palette.name}</option>
  {/each}
</select>

<style>
  .theme-picker {
  }
</style>
