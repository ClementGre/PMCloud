<script setup lang="ts">

</script>

<template>
  <div>
    <Header/>
    <main>
      <section>
        <NuxtPage/>
      </section>
    </main>
    <Footer/>
  </div>
</template>

<style lang="stylus" src="@/assets/css/default.styl"/>

<style scoped lang="stylus">
main
  background-color var(--primary-50)
  display flex
  justify-content center
  align-items stretch

  @media screen and (min-width: 400px)
    align-items center

  > section
    background-color white;
    padding 10px
    padding-top 40px
    flex-basis 100%

    @media screen and (min-width: 400px)
      border-radius 10px
      margin: 20px 0;
      padding 20px
      height fit-content
      flex-basis 350px

</style>
