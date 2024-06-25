<template>
  <div>
    <button @click="refresh">refresh</button>
    siema blog!
    {{ items }}
    <input v-model="text" type="text">
    <button @click="addItem">dodaj</button>
  </div>
</template>

<script setup>
import { day2_backend } from 'declarations/day2_backend/index';
import {ref} from "vue";

const items = ref([])
const text = ref("")

async function refresh() {
  items.value = await day2_backend.read_items();
}
async function addItem() {
  await day2_backend.add_item(text.value);
}
</script>