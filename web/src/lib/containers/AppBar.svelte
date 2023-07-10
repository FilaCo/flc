<script lang="ts">
  export let color: 'primary' | 'secondary' = 'primary'
</script>

<div class={`app-bar ${color} ${$$props.class ?? ''}`.trim()}>
  <slot />
</div>

<style>
  .app-bar {
  }
  .primary {
    background: var(--theme-color-primary-main);
    color: var(--theme-color-primary-contrastText);
  }
  .secondary {
    background: var(--theme-color-secondary-main);
    color: var(--theme-color-secondary-contrastText);
  }
</style>
