import type {Ref} from "vue";

export function useEmailValidator(email: Ref<string>, message: Ref<string> = ref('')) {

  watch(email, (newVal) => {
    if (newVal) {
      if (!validateEmail(newVal)) {
        message.value = 'Invalid email !'
        return
      }
    }
    message.value = ''
  })

  return message
}


export function validateEmail(email: string): boolean {
  return new RegExp(/^[^\s@]+@[^\s@]+\.[^\s@]+$/).test(email)
}
