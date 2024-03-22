<script setup lang="ts">

</script>

<template>
  <footer>
    <div>
      <p>Test 1</p>
    </div>
    <div>
      <p>Test 2</p>
    </div>
    <div>
      <p>Test 3</p>
    </div>
  </footer>
</template>

<style scoped lang="stylus">
footer
  display flex
  justify-content space-between

  padding 1rem 1rem
  @media screen and (min-width: 600px)
    padding 1rem 3rem
  @media screen and (min-width: 800px)
    padding 1rem 5rem

  background-color var(--primary-100)
</style>
