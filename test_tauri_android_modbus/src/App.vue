<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

const deviceAddress = ref("1");
const tcpHost = ref("192.168.88.28");
const tcpPort = ref("502");

const uptimeLabel = "uptime";
const uptimeValue = ref("—");
const uptimeError = ref("");

let timer: ReturnType<typeof setInterval> | null = null;
let busy = false;

async function pollUptimeOnce() {
  if (busy) return;
  busy = true;
  try {
    const unit = Number.parseInt(deviceAddress.value, 10);
    const port = Number.parseInt(tcpPort.value, 10);
    if (Number.isNaN(unit) || unit < 0 || unit > 255) {
      uptimeError.value = "Адрес устройства: число 0…255";
      uptimeValue.value = "—";
      return;
    }
    if (Number.isNaN(port) || port < 1 || port > 65535) {
      uptimeError.value = "TCP порт: число 1…65535";
      uptimeValue.value = "—";
      return;
    }

    const value = await invoke<string>("read_uptime_register", {
      params: {
        deviceAddress: unit,
        tcpHost: tcpHost.value.trim(),
        tcpPort: port,
      },
    });
    uptimeValue.value = value;
    uptimeError.value = "";
  } catch (e) {
    uptimeValue.value = "—";
    uptimeError.value = e instanceof Error ? e.message : String(e);
  } finally {
    busy = false;
  }
}

onMounted(() => {
  void pollUptimeOnce();
  timer = setInterval(() => void pollUptimeOnce(), 1000);
});

onUnmounted(() => {
  if (timer != null) {
    clearInterval(timer);
    timer = null;
  }
});
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

    <section class="uptime-section" aria-live="polite">
      <h2 class="uptime-heading">{{ uptimeLabel }}</h2>
      <p class="uptime-value">{{ uptimeValue }}</p>
      <p v-if="uptimeError" class="uptime-error">{{ uptimeError }}</p>
    </section>
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

.uptime-section {
  margin-top: 2rem;
  text-align: center;
  width: 100%;
  max-width: 24rem;
  margin-left: auto;
  margin-right: auto;
}

.uptime-heading {
  margin: 0 0 0.5rem;
  font-size: 1.1rem;
  font-weight: 600;
  text-transform: lowercase;
  letter-spacing: 0.02em;
}

.uptime-value {
  margin: 0;
  font-size: clamp(2.25rem, 10vw, 3.5rem);
  font-weight: 700;
  line-height: 1.1;
  font-variant-numeric: tabular-nums;
  word-break: break-all;
}

.uptime-error {
  margin: 0.75rem 0 0;
  font-size: 0.9rem;
  color: #b00020;
  line-height: 1.35;
}

@media (prefers-color-scheme: dark) {
  .uptime-error {
    color: #ff8a8a;
  }
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
