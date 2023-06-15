<script lang="ts">
  import { SvelteComponent } from 'svelte'

  export let children: SvelteComponent
  export let variant
</script>

<button class={$$props.class}>{children}</button>
