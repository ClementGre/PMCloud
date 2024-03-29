<script setup lang="ts">
definePageMeta({
  layout: 'noscroll'
})

import { ref } from 'vue'

const email = ref('')
const email_small = ref('')

// check valid email on change
watch(email, (newVal) => {
  if (newVal) {
    if (!new RegExp(/^[^\s@]+@[^\s@]+\.[^\s@]+$/).test(newVal)) {
      email_small.value = 'Invalid email'
      return
    }
  }
  email_small.value = ''
})

</script>

<template>
  <div class="root">
    <h1>Sign in to PMCloud</h1>
    <InputInForm name="Email" value="" type="email" aria="Email" v-model:value="email" :small="email_small" small_error/>
    <InputInForm name="Password" value="" type="password" aria="Password" />
    <br/>
    <p><nuxt-link to="/signup">Sign up</nuxt-link> | <nuxt-link to="/reset-password">Reset password</nuxt-link></p>
  </div>
</template>

<style scoped lang="stylus">
.root
  display flex
  flex-direction column
  align-items center
  gap 10px

  h1
    font-size 1.5em
    width 100%
    text-align center
    margin-bottom 1em

  .input-in-form
    width 100%
    max-width 250px

</style>
