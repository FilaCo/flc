<script lang="ts">
  const title = 'FLC Portfolio | Software Engineer'

  const sections = [
    {
      id: 'hom',
    },
  ]
</script>

<svelte:head>
  <title>{title}</title>
</svelte:head>

<nav class="navbar" />

<footer class="footer" />

<style>
  :global(html) {
    font-family: 'Roboto Flex', sans-serif;
  }

  :root {
  }

  .footer {
    position: fixed;
    left: 0;
    bottom: 0;
    text-align: center;
  }
</style>
