<script setup lang="ts">

const props = defineProps({
  name: String,
  value: String,
  type: String,
  aria: String,
  icon: String,
  small: String,
  small_error: Boolean
})

const emit = defineEmits(['update:value'])

const model = defineModel()


</script>

<template>
  <div class="input-in-form">
    <label v-if="name" for="username">{{ name }}</label>
    <InputText :type="type"
               :value="props.value"
               @input="emit('update:value', $event.target.value)"
               :aria-describedby="props.aria"/>
    <small v-if="props.small"
           :style="props.small_error ? 'color: var(--red-700);' : ''">
      {{ props.small }}
    </small>
  </div>
</template>

<style scoped lang="stylus">
div
  *
    display block
  label
    margin-bottom 5px
  input
    width 100%
</style>
