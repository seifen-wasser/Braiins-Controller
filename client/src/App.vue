<template>
  <div class="min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
    <!-- Navbar -->
    <nav class="bg-white dark:bg-gray-800 shadow-md">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex justify-between h-16 items-center">
          <div class="flex items-center">
            <h1 class="text-xl font-semibold text-gray-900 dark:text-white">Miner Dashboard</h1>
          </div>
          <div class="flex items-center space-x-4">
            <button @click="toggleTheme" class="p-2 rounded-lg bg-gray-200 dark:bg-gray-700 text-gray-800 dark:text-gray-200">
              {{ isDark ? '🌙' : '☀️' }}
            </button>
            <button @click="refresh" class="px-3 py-1 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition">
              Refresh
            </button>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <!-- Status Panel -->
        <StatusPanel 
          :status="status" 
          :config="config" 
          @pause="pauseMiner" 
          @resume="resumeMiner" 
          :loadingMiners="loadingMiners"
        />
        
        <!-- Config Editor -->
        <ConfigEditor :config="config" @save="saveConfig" />
      </div>

      <!-- Error message -->
      <div v-if="error" class="mt-8 p-4 bg-red-100 dark:bg-red-900 border border-red-400 dark:border-red-700 rounded-lg text-red-700 dark:text-red-200">
        {{ error }}
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, watch } from 'vue';
import axios from 'axios';
import StatusPanel from './components/StatusPanel.vue';
import ConfigEditor from './components/ConfigEditor.vue';

const API_BASE = import.meta.env.VITE_API_BASE_URL || '/api';

const config = ref({ braiins: { devices: [] }, shelly: { shelly_ip: '' }, max_watts_braiins: 0, start_watts_braiins: 0 });
const status = ref({ shelly_power_watts: 0, miners: [] });
const error = ref('');
const loadingMiners = ref({});
let intervalId = null;

// Dark mode
const isDark = ref(localStorage.getItem('theme') === 'dark');
watch(isDark, (val) => {
  const root = document.documentElement;
  if (val) {
    root.classList.add('dark');
    localStorage.setItem('theme', 'dark');
  } else {
    root.classList.remove('dark');
    localStorage.setItem('theme', 'light');
  }
}, { immediate: true });
function toggleTheme() { isDark.value = !isDark.value; }

async function fetchConfig() {
  try {
    const res = await axios.get(`${API_BASE}/config/get`);
    config.value = res.data;
  } catch (err) {
    error.value = 'Failed to load config: ' + err.message;
  }
}

async function fetchStatus() {
  try {
    const res = await axios.get(`${API_BASE}/status`);
    status.value = res.data;
  } catch (err) {
    error.value = 'Failed to load status: ' + err.message;
  }
}

async function saveConfig(newConfig) {
  try {
    await axios.post(`${API_BASE}/config/set`, newConfig);
    await fetchConfig();
    error.value = '';
  } catch (err) {
    error.value = 'Failed to save config: ' + err.message;
  }
}

async function pauseMiner(ip) {
  loadingMiners.value[ip] = true;
  try {
    await axios.post(`${API_BASE}/miners/${ip}/pause`);
    await fetchStatus();
  } catch (err) {
    error.value = `Failed to pause ${ip}: ${err.message}`;
  } finally {
    delete loadingMiners.value[ip];
  }
}

async function resumeMiner(ip) {
  loadingMiners.value[ip] = true;
  try {
    await axios.post(`${API_BASE}/miners/${ip}/resume`);
    await fetchStatus();
  } catch (err) {
    error.value = `Failed to resume ${ip}: ${err.message}`;
  } finally {
    delete loadingMiners.value[ip];
  }
}

async function refresh() {
  await Promise.all([fetchConfig(), fetchStatus()]);
}

onMounted(async () => {
  await refresh();
  intervalId = setInterval(fetchStatus, 5000);
});

onUnmounted(() => {
  if (intervalId) clearInterval(intervalId);
});
</script>