<template>
  <div class="dashboard">
    <!-- Welcome Section -->
    <div class="welcome-section">
      <h1 class="welcome-title">Welcome to Brains Controller</h1>
      <p class="welcome-subtitle">AI Mining Management Dashboard</p>
    </div>

    <!-- Loading / Error States -->
    <div v-if="loadingShelly" class="loading">Loading Shelly data...</div>
    <div v-else-if="errorShelly" class="error">Error: {{ errorShelly }}</div>
    <!-- Main Stats Card -->
    <div v-else class="card stats-card">
      <div class="card-header">
        <h3>Shelly EM Status</h3>
        <button class="refresh-btn" @click="fetchShellyData" :disabled="loadingShelly" title="Refresh">
          ⟳
        </button>
      </div>
      <div class="card-content">
        <div class="stats-grid">
          <div class="stat-metric">
            <div class="metric-value">{{  Math.round(shellyData?.data.total_current  ?? 0) }} A</div>
            <div class="metric-label">Total Current</div>
          </div>
          <div class="stat-metric">
            <div class="metric-value">{{  Math.round(shellyData?.data.total_act_power ?? 0) }} W</div>
            <div class="metric-label">Active Power</div>
          </div>
          <div class="stat-metric">
            <div class="metric-value">{{ Math.round(shellyData?.data.total_aprt_power ?? 0) }} VA</div>
            <div class="metric-label">Apparent Power</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import axios from "axios";

// Define expected API response shape
interface ShellyStatus {
  total_current: number;
  total_act_power: number;
  total_aprt_power: number;
}

// Reactive state
const loadingShelly = ref(false);
const errorShelly = ref<string | null>(null);
const shellyData = ref<ShellyStatus | null>(null);
const lastUpdated = ref("");


// Methods
async function fetchShellyData() {
  loadingShelly.value = true;
  errorShelly.value = null;

  try {
    // Use relative path – requires Vite proxy to forward to backend
    //const response = await axios.get("http://127.0.0.1:8080/api/shelly/stus");
    const response = await axios({
      method: "get",
      url: "http://192.168.178.98:8080/api/shelly/status",
      headers: {
        "Content-Type": "application/json",
        "User-Agent": "BrainsController/1.0",
        "Accept-Encoding": "gzip, deflate, br",
        "Connection": "keep-alive",
        "Accept": "*/*",
        
      },
    })
    shellyData.value = response.data;
    lastUpdated.value = new Date().toLocaleTimeString();
    console.log("Data received:", response.data);
  } catch (err: any) {
    errorShelly.value = err.message || "Failed to fetch Shelly data";
    console.error(err);
  } finally {
    loadingShelly.value = false;
  }
}

// Lifecycle
onMounted(() => {
  fetchShellyData();
});
</script>

<style scoped>
.dashboard {
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
}

.welcome-section {
  text-align: center;
  margin-bottom: 3rem;
  background: linear-gradient(
    135deg,
    rgba(150, 0, 255, 0.1) 0%,
    rgba(150, 0, 255, 0.05) 100%
  );
  padding: 2rem;
  border-radius: 20px;
  border: 1px solid rgba(150, 0, 255, 0.2);
}

.welcome-title {
  font-size: 2.5rem;
  background: linear-gradient(135deg, #9600ff 0%, #c500ff 100%);
  -webkit-background-clip: text;
  background-clip: text;
  color: transparent;
  margin-bottom: 0.5rem;
}

.welcome-subtitle {
  color: #a0a0c0;
  font-size: 1.1rem;
  margin-bottom: 2rem;
}

.stats-banner {
  display: flex;
  justify-content: center;
  gap: 3rem;
  margin-top: 1.5rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.stat-value {
  font-size: 2rem;
  font-weight: bold;
  color: #9600ff;
}

.stat-label {
  font-size: 0.9rem;
  color: #a0a0c0;
  margin-top: 0.25rem;
}

.dashboard-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 1.5rem;
}

.card {
  width: 500px;
  height: 150px;
  background: #1a1a2ecc;
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 16px;
  backdrop-filter: blur(10px);
  overflow: hidden;
  transition: all 0.3s ease;
}

.card:hover {
  transform: translateY(-4px);
  box-shadow: 0 10px 30px rgba(150, 0, 255, 0.15);
  border-color: rgba(150, 0, 255, 0.3);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.25rem 1.5rem;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.card-header h3 {
  margin: 0;
  font-size: 1.2rem;
  color: #fff;
}

.card-content {
  padding: 0.6rem;
}

.refresh-btn {
  background: transparent;
  border: 1px solid rgba(150, 0, 255, 0.3);
  color: #9600ff;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
}

.refresh-btn:hover:not(:disabled) {
  background: rgba(150, 0, 255, 0.1);
  transform: rotate(180deg);
}

.refresh-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.status-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.5rem;
}

.status-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-radius: 12px;
  transition: all 0.3s ease;
}

.status-item:hover {
  background: rgba(255, 255, 255, 0.05);
}

.status-icon {
  font-size: 1.5rem;
}

.status-info {
  display: flex;
  flex-direction: column;
}

.status-name {
  font-size: 0.9rem;
  color: #a0a0c0;
}

.status-value {
  font-size: 1rem;
  font-weight: 600;
}

.status-online .status-value {
  color: #10b981;
}

.status-offline .status-value {
  color: #ef4444;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.5rem;
}

.stat-metric {
  text-align: center;
  padding: 1rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 12px;
  transition: all 0.3s ease;
}

.stat-metric:hover {
  background: rgba(150, 0, 255, 0.1);
  transform: translateY(-2px);
}

.metric-value {
  font-size: 1.4rem;
  font-weight: bold;
  color: #fff;
  margin-bottom: 0.5rem;
}

.metric-label {
  font-size: 0.7rem;
  color: #a0a0c0;
}

.metric-trend {
  font-size: 0.8rem;
  margin-top: 0.5rem;
  padding: 0.25rem 0.5rem;
  border-radius: 12px;
  display: inline-block;
}

.trend-up {
  background: rgba(16, 185, 129, 0.2);
  color: #10b981;
}

.trend-down {
  background: rgba(239, 68, 68, 0.2);
  color: #ef4444;
}

.trend-neutral {
  background: rgba(245, 158, 11, 0.2);
  color: #f59e0b;
}

.last-updated {
  font-size: 0.8rem;
  color: #a0a0c0;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.action-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.5rem;
  padding: 1.25rem;
  border: none;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.05);
  color: white;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 0.9rem;
}

.action-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.start-btn:hover:not(:disabled) {
  background: rgba(16, 185, 129, 0.2);
  border: 1px solid rgba(16, 185, 129, 0.4);
}

.stop-btn:hover:not(:disabled) {
  background: rgba(239, 68, 68, 0.2);
  border: 1px solid rgba(239, 68, 68, 0.4);
}

.restart-btn:hover:not(:disabled) {
  background: rgba(245, 158, 11, 0.2);
  border: 1px solid rgba(245, 158, 11, 0.4);
}

.shutdown-btn:hover:not(:disabled) {
  background: rgba(150, 0, 255, 0.2);
  border: 1px solid rgba(150, 0, 255, 0.4);
}

.clear-btn:hover:not(:disabled) {
  background: rgba(156, 163, 175, 0.2);
  border: 1px solid rgba(156, 163, 175, 0.4);
}

.settings-btn:hover:not(:disabled) {
  background: rgba(59, 130, 246, 0.2);
  border: 1px solid rgba(59, 130, 246, 0.4);
}

.action-icon {
  font-size: 1.5rem;
}

.action-text {
  font-weight: 500;
}

.activity-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.activity-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.03);
  transition: all 0.3s ease;
}

.activity-item:hover {
  background: rgba(255, 255, 255, 0.05);
}

.activity-success {
  border-left: 3px solid #10b981;
}

.activity-info {
  border-left: 3px solid #3b82f6;
}

.activity-warning {
  border-left: 3px solid #f59e0b;
}

.activity-error {
  border-left: 3px solid #ef4444;
}

.activity-icon {
  font-size: 1.25rem;
}

.activity-content {
  flex: 1;
}

.activity-message {
  font-size: 0.9rem;
  color: #fff;
  margin-bottom: 0.25rem;
}

.activity-time {
  font-size: 0.75rem;
  color: #a0a0c0;
}

.empty-state {
  text-align: center;
  padding: 2rem;
  color: #a0a0c0;
  font-style: italic;
}

.chart-placeholder {
  padding: 1rem 0;
}

.chart-container {
  height: 200px;
  display: flex;
  align-items: flex-end;
  justify-content: center;
}

.chart-mock {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  justify-content: flex-end;
}

.chart-grid {
  display: flex;
  justify-content: space-around;
  align-items: flex-end;
  height: 80%;
  padding: 0 1rem;
}

.chart-bar {
  width: 40px;
  background: linear-gradient(to top, #9600ff, #c500ff);
  border-radius: 6px 6px 0 0;
  transition: height 0.3s ease;
}

.chart-labels {
  display: flex;
  justify-content: space-around;
  padding: 0.5rem 1rem;
  color: #a0a0c0;
  font-size: 0.8rem;
}

.chart-note {
  text-align: center;
  color: #a0a0c0;
  font-size: 0.8rem;
  margin-top: 1rem;
}

.time-range {
  display: flex;
  gap: 0.5rem;
}

.range-btn {
  padding: 0.25rem 0.75rem;
  background: transparent;
  border: 1px solid rgba(150, 0, 255, 0.3);
  color: #a0a0c0;
  border-radius: 12px;
  font-size: 0.8rem;
  cursor: pointer;
  transition: all 0.3s ease;
}

.range-btn.active {
  background: rgba(150, 0, 255, 0.2);
  color: #9600ff;
  border-color: #9600ff;
}

.range-btn:hover:not(.active) {
  background: rgba(150, 0, 255, 0.1);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 8px;
}

.info-label {
  font-size: 0.9rem;
  color: #a0a0c0;
}

.info-value {
  font-size: 0.9rem;
  color: #fff;
  font-weight: 500;
}

/* Responsive Design */
@media (max-width: 1200px) {
  .dashboard-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .dashboard {
    padding: 1rem;
  }

  .dashboard-grid {
    grid-template-columns: 1fr;
  }

  .welcome-section {
    padding: 1.5rem;
  }

  .welcome-title {
    font-size: 2rem;
  }

  .stats-banner {
    flex-direction: column;
    gap: 1.5rem;
  }

  .stats-grid,
  .actions-grid,
  .status-grid {
    grid-template-columns: 1fr;
  }
}
</style>
