<script setup lang="ts">

const set_backgrounds = (top: string) => {
  console.log('Setting backgrounds...', top)
}

</script>

<template>
  <div>
    <Header/>
    <div class="main-content">
      <slot @set_backgrounds="set_backgrounds"/>
    </div>
    <Footer/>
  </div>
</template>

<style scoped lang="stylus">

.main-content
  background-color var(--surface-ground)
  flex-grow 1

:deep(main)
  padding-top 48px

</style>
