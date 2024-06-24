<script setup>
import { ref } from 'vue';
let greeting = ref('');
let counter = ref(0);

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  greeting.value = `Hello ${name}`;
}

const decrease = () => {
  counter.value--;
};

const increase = () => {
  counter.value++;
};
</script>

<template>
  <main>
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>

    <section>
      <button @click="decrease">-</button>
      {{ counter }}
      <button @click="increase">+</button>
    </section>

  </main>
</template>