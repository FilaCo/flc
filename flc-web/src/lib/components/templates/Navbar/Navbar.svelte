<script lang="ts">
  import type Section from './Section'
  import Logo from '$lib/assets/logo.svg'

  export let sections: Section[]
  export let mode: 'scroll' | 'a'

  const scrollIntoView = ({ target }) => {
    const el = document.querySelector(target.getAttribute('href'))

    if (el === null) {
      return
    }

    el.scrollIntoView({
      behavior: 'smooth',
    })
  }
</script>

<nav class="navbar">
  <div class="row">
    <div class="col-md-1">
      <img class="navbar__logo" src={Logo} alt="FLC Logo" />
    </div>
    <div class="col-md-8" />
    <div class="col-md-3">
      <ul class="navbar__sections">
        {#each sections as section}
          <li>
            {#if mode === 'scroll'}
              <a class="navbar__section" href={section.id} on:click|preventDefault={scrollIntoView}
                >{section.name}</a
              >
            {:else}
              <a class="navbar__section" href={section.id}>{section.name}</a>
            {/if}
          </li>
        {/each}
      </ul>
    </div>
  </div>
</nav>

<style>
  .navbar {
    background: var(--secondary-color);
  }
  .navbar__sections {
    list-style-type: none;
  }
  .navbar__section {
    text-decoration: none;
  }
</style>
