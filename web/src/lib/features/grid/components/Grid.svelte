<div class={`grid ${$$props.class ?? ''}`.trim()}>
  <slot />
</div>

<style>
  .grid {
    display: grid;
  }
</style>
