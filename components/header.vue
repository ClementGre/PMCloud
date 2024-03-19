<script setup lang="ts">

</script>

<template>
  <header>
    <nav>
      <ul>
        <li>
          <nuxt-link to="/">Home</nuxt-link>
        </li>
        <li>
          <nuxt-link to="/about">About</nuxt-link>
        </li>
      </ul>

    </nav>
  </header>
</template>

<style scoped lang="stylus">
header
  background-color #f0f0f0
  padding 1rem
  nav
    ul
      list-style none
      padding 0
      margin 0
      li
        display inline
        margin-right 1rem
        a
          text-decoration none
          color #333
          &hover
            color #666
</style>
