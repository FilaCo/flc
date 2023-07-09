<script lang="ts">
  import { getContext } from 'svelte'
  import type { ThemeContext } from '$lib/features/theming/Theme'

  const { currentTheme, availableThemes, setCurrentTheme } = getContext<ThemeContext>('theme')

  let selected = $currentTheme.name
</script>

<select
  class={`theme-picker ${$$props.class ?? ''}`.trim()}
  bind:value={selected}
  on:change={() => setCurrentTheme(selected)}
>
  {#each availableThemes.themes as theme}
    <option value={theme.name}>{theme.name}</option>
  {/each}
</select>
