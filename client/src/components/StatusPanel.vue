<template>
  <div class="bg-white dark:bg-gray-800 rounded-xl shadow-lg overflow-hidden">
    <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
      <h2 class="text-xl font-semibold text-gray-900 dark:text-white">Current Status</h2>
    </div>
    <div class="p-6 space-y-6">
      <!-- Shelly -->
      <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-4">
        <div class="flex items-center justify-between">
          <span class="text-gray-600 dark:text-gray-300">Shelly Power</span>
          <span class="text-2xl font-bold text-gray-900 dark:text-white">
            {{ status.shelly_power_watts?.toFixed(2) || '?' }} W
          </span>
        </div>
        <div class="mt-2 text-sm text-gray-500 dark:text-gray-400">
          Max: {{ config.max_watts_braiins }} W | Start: {{ config.start_watts_braiins }} W
        </div>
      </div>

      <!-- Miners -->
      <div>
        <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-3">Miners</h3>
        <div v-if="!status.miners?.length" class="text-gray-500 dark:text-gray-400 text-center py-4">
          No miners configured.
        </div>
        <div class="space-y-3">
          <div v-for="miner in status.miners" :key="miner.ip" 
               class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-4 flex flex-col sm:flex-row sm:items-center sm:justify-between gap-3">
            <div>
              <div class="font-mono text-gray-900 dark:text-white">{{ miner.ip }}</div>
              <div class="text-sm text-gray-500 dark:text-gray-400">{{ miner.username }}</div>
            </div>
            <div class="flex items-center gap-4">
              <span :class="['px-3 py-1 rounded-full text-sm font-medium', 
                   miner.mining ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200' : 
                                  'bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200']">
                {{ miner.mining ? '⛏️ Mining' : '⏸ Paused' }}
              </span>
              <button
                v-if="!miner.mining"
                @click="$emit('resume', miner.ip)"
                :disabled="loadingMiners[miner.ip]"
                class="px-3 py-1 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50 transition"
              >
                Resume
              </button>
              <button
                v-else
                @click="$emit('pause', miner.ip)"
                :disabled="loadingMiners[miner.ip]"
                class="px-3 py-1 bg-red-500 text-white rounded-lg hover:bg-red-600 disabled:opacity-50 transition"
              >
                Pause
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  status: Object,
  config: Object,
  loadingMiners: Object
});
defineEmits(['pause', 'resume']);
</script>