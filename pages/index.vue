<template>
  <main>
    <p>This is the content of the app!</p>
    <NuxtLink to="/overview">Go to the landing page</NuxtLink>
    <IconField iconPosition="left">
      <InputIcon class="pi pi-search"></InputIcon>
      <InputText placeholder="Search"/>
      <Button label="Check" icon="pi pi-check"/>
    </IconField>
    <Card>
      <template #title>
        Multi Select : {{ selectedCities }}
      </template>
      <template #content>
        <MultiSelect v-model="selectedCities" :options="cities" filter optionLabel="name" placeholder="Select Cities"
                     :maxSelectedLabels="3"/>
      </template>

    </Card>
  </main>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'app'
})

import {ref} from "vue";

const selectedCities = ref();
const cities = ref([
  {name: 'New York', code: 'NY'},
  {name: 'Rome', code: 'RM'},
  {name: 'London', code: 'LDN'},
  {name: 'Istanbul', code: 'IST'},
  {name: 'Paris', code: 'PRS'}
]);
</script>
