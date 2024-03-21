<script setup lang="ts">

</script>

<template>
  <header>
    <div class="left">
      <div class="icon">
        <img src="/favicon.ico" alt="PMCloud icon">
      </div>
      <h2>
        PMCloud
      </h2>
    </div>
    <div class="right">
      <nav>
        <ul>
          <li class="current">
            <nuxt-link to="/"><span class="pi pi-home"/>Home</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/about"><span class="pi pi-info-circle"/>About</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/about"><span class="pi pi-dollar"/>Pricing</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/signin"><span class="pi pi-sign-in"/>Sign in</nuxt-link>
          </li>
          <li>
            <nuxt-link to="/singup"><span class="pi pi-user-plus"/>Sign up</nuxt-link>
          </li>
        </ul>
      </nav>
    </div>
  </header>
</template>

<style scoped lang="stylus">
header
  -webkit-backdrop-filter: blur(8px);
  backdrop-filter: blur(8px);
  background-color: hsla(174, 53%, 90%, 0.7);
  border-bottom: 1px solid hsl(174, 53%, 90%);

  position fixed
  min-height 4rem
  width 100vw

  display flex
  justify-content space-between
  align-items stretch
  flex-wrap wrap

  padding 0 1rem
  @media screen and (min-width: 600px)
    padding 0 3rem
  @media screen and (min-width: 800px)
    padding 0 5rem

.left, .right
  display flex
  align-items center

.left
  gap .5rem

nav
  height 100%

  ul
    list-style none
    height 100%
    padding 0
    margin 0
    display flex
    flex-wrap wrap
    gap .7rem


    li
      height 100%
      display block
      position relative
      &.current a
        border-color hsl(174, 90%, 40%)
      a
        position relative
        top 50%
        transform translateY(-50%)
        display flex
        align-items center
        white-space nowrap
        gap .5rem

        font-size 1.1rem;
        vertical-align center
        padding .4rem .7rem
        text-decoration none
        color var(--text-color)

        border-radius .5rem
        background-color var(--surface-ground)
        border 1px solid var(--surface-border)

        &:hover
          background-color var(--surface-hover)
</style>
