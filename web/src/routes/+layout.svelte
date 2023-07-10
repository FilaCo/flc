<script lang="ts">
  import ThemeProvider from '$lib/containers/theming/components/ThemeProvider.svelte'
  import AppBar from '$lib/containers/AppBar.svelte'
  import Footer from '$lib/containers/Footer.svelte'
</script>

<ThemeProvider>
  <AppBar />
  <slot />
  <Footer />
</ThemeProvider>
