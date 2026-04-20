<script setup lang="ts">
import { ref } from "vue";

const deviceAddress = ref("1");
const tcpHost = ref("192.168.88.28");
const tcpPort = ref("502");
</script>

<template>
  <main class="container">
    <form class="form" @submit.prevent>
      <label class="field">
        <span class="label">Адрес устройства:</span>
        <input v-model="deviceAddress" type="text" inputmode="numeric" autocomplete="off" />
      </label>
      <label class="field">
        <span class="label">TCP хост:</span>
        <input v-model="tcpHost" type="text" autocomplete="off" autocapitalize="none" />
      </label>
      <label class="field">
        <span class="label">TCP порт:</span>
        <input v-model="tcpPort" type="text" inputmode="numeric" autocomplete="off" />
      </label>
    </form>
  </main>
</template>

<style scoped>
.form {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
  max-width: 22rem;
  margin: 0 auto;
  text-align: left;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.label {
  font-weight: 500;
  font-size: 0.95rem;
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding: 1.25rem 1rem 2rem;
  min-height: 100vh;
  box-sizing: border-box;
}

input {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 0.85em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  width: 100%;
  box-sizing: border-box;
}

input {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  input {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
}
</style>
