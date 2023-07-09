<script lang="ts">
  import { getContext } from 'svelte'
  import type { ThemeContext } from '$lib/features/theming/utils/palette'

  const { theme, themes, setTheme } = getContext<ThemeContext>('theme')

  let selected = $theme.name
</script>

<select
  class={`theme-picker ${$$props.class ?? ''}`.trim()}
  bind:value={selected}
  on:change={() => setTheme(selected)}
>
  {#each themes.themes as theme}
    <option value={theme.name}>{theme.name}</option>
  {/each}
</select>
