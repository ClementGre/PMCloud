<script setup lang="ts">
import {validateEmail} from "~/composables/validators";

definePageMeta({
  layout: 'noscroll',
})

const email = ref('')
const email_small = ref('')
const password = ref('')

const onSubmit = () => {
  let validated: boolean = validateEmail(email.value)
  if (!validated) {
    email_small.value = 'Email is invalid';
  }else{
    email_small.value = '';
    useUserStore().signIn(email.value, password.value)
  }
}
</script>

<template>
  <main>
    <h1>Sign in to PMCloud</h1>
    <form>
      <InputInForm name="Email" type="email" aria="Email" v-model:value="email" :small="email_small"
                   small_error/>
      <InputInForm name="Password" reset_password value="" type="password" aria="Password" v-model:value="password"/>
      <Button label="Sign in" icon="pi pi-sign-in" @click="onSubmit"/>
    </form>
    <p>Already have an account?
      <nuxt-link to="/signup">Sign up</nuxt-link>
    </p>
  </main>
</template>

<style scoped lang="stylus">
main

  form
    display flex
    flex-direction column
    align-items center
    gap 10px

    .input-in-form
      width 100%
      max-width 250px

  button
    width 100%
    max-width 150px
    margin 1em 0

</style>
