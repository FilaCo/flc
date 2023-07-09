<script lang="ts">
  import ThemeProvider from '$lib/features/theming/components/ThemeProvider.svelte'
  import AppBar from '$lib/features/appbar/components/AppBar.svelte'
  import Footer from '$lib/features/footer/components/Footer.svelte'
  import Grid from '$lib/features/grid/components/Grid.svelte'
  import ThemePicker from '$lib/features/theming/components/ThemePicker.svelte'
</script>

<ThemeProvider>
  <Grid>
    <AppBar>
      <ThemePicker />
    </AppBar>
    <slot />
    <!--    TODO: use i18n-->
    <Footer>Powered by FLC</Footer>
  </Grid>
</ThemeProvider>

<style>
</style>
