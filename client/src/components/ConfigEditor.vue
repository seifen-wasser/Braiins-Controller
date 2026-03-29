<template>
  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden">
    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white">Configuration</h2>
    </div>
    <form @submit.prevent="submit" class="p-6 space-y-6">
      <!-- Shelly -->
      <div>
        <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Shelly IP</label>
        <input v-model="editableConfig.shelly.shelly_ip" type="text"
               class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
      </div>
      <div class="grid grid-cols-2 gap-4">
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Max Watts (pause above)</label>
          <input v-model.number="editableConfig.max_watts_braiins" type="number" step="10"
                 class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
        </div>
        <div>
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">Start Watts (resume below)</label>
          <input v-model.number="editableConfig.start_watts_braiins" type="number" step="10"
                 class="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg shadow-sm focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
        </div>
      </div>

      <!-- Miners -->
      <div>
        <div class="flex justify-between items-center mb-2">
          <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Miners</label>
          <button type="button" @click="addMiner" class="text-sm text-blue-600 hover:text-blue-700 dark:text-blue-400">
            + Add Miner
          </button>
        </div>
        <div class="space-y-3">
          <div v-for="(miner, idx) in editableConfig.braiins.devices" :key="idx"
               class="border border-gray-200 dark:border-gray-700 rounded-lg p-3">
            <div class="grid grid-cols-1 sm:grid-cols-3 gap-3">
              <input v-model="miner.braiins_ip" placeholder="IP" 
                     class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
              <input v-model="miner.braiins_username" placeholder="Username"
                     class="px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
              <div class="flex gap-2">
                <input v-model="miner.braiins_password" placeholder="Password" type="password"
                       class="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white">
                <button type="button" @click="removeMiner(idx)" class="px-2 bg-red-100 text-red-600 rounded-lg hover:bg-red-200 dark:bg-red-900 dark:text-red-300">
                  ✖
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Actions -->
      <div class="flex justify-end gap-3 pt-4">
        <button type="button" @click="reset" :disabled="saving"
                class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50">
          Reset
        </button>
        <button type="submit" :disabled="saving"
                class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 disabled:opacity-50 transition">
          Save Configuration
        </button>
      </div>
    </form>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';

const props = defineProps({
  config: Object
});
const emit = defineEmits(['save']);

const editableConfig = ref(JSON.parse(JSON.stringify(props.config)));
const saving = ref(false);

watch(() => props.config, (newVal) => {
  editableConfig.value = JSON.parse(JSON.stringify(newVal));
}, { deep: true });

function addMiner() {
  editableConfig.value.braiins.devices.push({
    braiins_ip: '',
    braiins_username: 'root',
    braiins_password: ''
  });
}

function removeMiner(idx) {
  editableConfig.value.braiins.devices.splice(idx, 1);
}

function reset() {
  editableConfig.value = JSON.parse(JSON.stringify(props.config));
}

async function submit() {
  saving.value = true;
  await emit('save', editableConfig.value);
  saving.value = false;
}
</script>