<script setup>
import { ref } from 'vue';
import { day2_backend } from 'declarations/day2_backend/index';
import Blog from './components/Blog.vue';

let greeting = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const name = target.querySelector('#name').value;
  const numer = target.querySelector('#numer').value;
  await day2_backend.greet(name, Number(numer)).then((response) => {
    greeting.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo"/>
    <br/>
    <br/>
    <form action="#" @submit="handleSubmit">
      <label for="name">Enter your name: &nbsp;</label>
      <input id="name" alt="Name" type="text"/>
      <input id="numer" alt="Numer" type="number"/>
      <button type="submit">Click Me!</button>
    </form>
    <section id="greeting">{{ greeting }}</section>
    <Blog/>
  </main>
</template>