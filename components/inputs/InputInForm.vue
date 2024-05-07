<script setup lang="ts">

const props = defineProps({
  name: String,
  value: String,
  type: String,
  aria: String,
  icon: String,
  small: String,
  small_error: Boolean,
  reset_password: Boolean
})

const target = ref()
const emit = defineEmits(['update:value'])

let id: string = '';
if(props.name){
  id = props.name.toLowerCase() + '-input';
}

</script>

<template>
  <div class="input-in-form">
    <div class="header" v-if="name">
      <label :for="id">{{ name }}</label>
      <label v-if="reset_password" :for="id"><nuxt-link href="/resetpassword">Forgot password?</nuxt-link></label>
    </div>

    <InputText
        :id="id"
        :ref="target"
        :type="type"
        :value="props.value"
        @input="emit('update:value', ($event.target as HTMLInputElement).value)"
        :aria-describedby="props.aria"
        :invalid="small_error && small?.length != 0"
        autocomplete="on"/>

    <small v-if="props.small"
           :style="props.small_error ? 'color: var(--red-700);' : ''">
      {{ props.small }}
    </small>
  </div>
</template>

<style scoped lang="stylus">
.input-in-form
  *
    display block
  .header
    display flex
    justify-content space-between
    margin-bottom 5px

  input
    width 100%

</style>
