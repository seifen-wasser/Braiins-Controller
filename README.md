# Braiins Controller

> **⚠️ THIS IS STILL IN DEVELOPMENT**  
> *And I’m still learning Rust, so don’t expect good code!* – [seifen-wasser](https://github.com/seifen-wasser)

**Braiins Controller** is a tool that dynamically adjusts a BraiinsOS‑based miner (e.g., Antminer S19) using real‑time power consumption data. It currently integrates with **Shelly EM / Shelly 3EM** power monitors and aims to optimise miner behaviour based on available power or user‑defined limits.

---

## 🧠 How It Works

1. The **Rust backend** periodically reads power data from your Shelly device(s).  
2. It communicates with the **BraiinsOS API** (`bos‑api` directory) on your miner.  
3. Based on a configured power target (e.g., stay below a certain wattage), the controller adjusts the miner’s frequency, voltage, or operating mode.  
4. A **Vue + TypeScript frontend** (`client` directory) provides a simple UI for monitoring and configuration.

---

## 🛠️ Supported Hardware

- **Miner**: Any miner running **BraiinsOS** (tested on Antminer S19)  
- **Power monitor**:  
  - Shelly EM  
  - Shelly 3EM  
  - *(Other devices can be added by extending the code)*

---

## 📋 Prerequisites

- **Rust** (latest stable) – for the backend  
- **Node.js** + **npm/yarn** – for the Vue frontend  
- A **BraiinsOS miner** accessible on your network  
- A **Shelly EM / 3EM** device on the same network  
- (Optional) A reverse proxy or static file server for production deployment

---

## 🚀 Getting Started

### 1. Clone the Repository
```bash
git clone https://github.com/seifen-wasser/Braiins-Controller.git
cd Braiins-Controller
```

### 2. Build and Run the Backend
```bash
cargo build --release
cargo run --release
```

### 3. Build and Serve the Frontend
```bash
cd client
npm install
npm run build
```

### 4. Build and Serve the Frontend
```bash
cd client
npm install
npm run build
npx serve -s dist -l 3000
```
