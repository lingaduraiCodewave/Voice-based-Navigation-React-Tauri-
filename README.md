# 🎙️ Voice Navigation App (React + Tauri + Vosk)

This project enables **voice-based page navigation** in a React + Tauri desktop application using the **Vosk offline speech recognition model**.

It’s built with:

- ⚛️ **React + TypeScript + Vite** – for the frontend
- 🦀 **Tauri + Rust** – for the desktop backend
- 🧠 **Vosk** – for offline voice recognition

---

## 🚀 Features

- 🎤 Voice-to-text using Vosk
- 🧭 Voice-based page navigation
- 💻 Cross-platform (Windows, macOS, Linux)
- ⚡ Offline support (no internet required)
- 🪶 Lightweight and fast

---

## 🧩 Tech Stack

| Layer              | Technology                   |
| ------------------ | ---------------------------- |
| Frontend           | React, TypeScript, Vite      |
| Desktop Shell      | Tauri                        |
| Speech Recognition | Vosk                         |
| Build Tooling      | Cargo (Rust) + npm/yarn/pnpm |

---

## 🧠 Prerequisites

Before running the project, make sure you have:

- **Node.js** ≥ 18
- **Rust & Cargo** → [Install Rust](https://rustup.rs)
- **Tauri CLI** → install globally:
  ```bash
  cargo install tauri-cli
  ⚙️ Installation Steps
  1️⃣ Clone the Repository
  cd voice-navigation-app
  2️⃣ Install Dependencies
  npm install
  bash
  ```

3️⃣ Set Up Vosk Model
Since the model is large, it’s not included in the repo.

Download the model from:
👉 https://alphacephei.com/vosk/models

Recommended model:
vosk-model-small-en-us-0.15

Extract and place it inside:

```
src-tauri/vosk-model-small-en-us-0.15/
```

---

## 🧰 Development

Run the app in development mode:

npm run tauri dev
This command starts:

The Vite dev server (React frontend)

## The Tauri runtime (Rust backend)

## 🏗️ Build for Production

To build the production-ready desktop app:

```
npm run tauri build

```

---

## 🧹 .gitignore Highlights

We’ve excluded these files and directories:

```
node_modules
dist
src-tauri/target
vosk-model-small-en-us-0.15/
\*.log
.DS_Store
.vscode/
.idea/
```

---

## ✅ The Vosk model folder is ignored — you’ll need to download it manually before building.

## 🧑‍💻 ESLint & React Compiler Notes

We use a minimal ESLint setup for simplicity.
If you plan to expand the configuration, enable type-safe lint rules:

js

tseslint.configs.strictTypeChecked
You can also install React-specific lint plugins for deeper linting:

```
npm install eslint-plugin-react-x eslint-plugin-react-dom
📁 Project Structure
```

```
voice-navigation-app/
│
├── src/ # React frontend
│ ├── components/
│ └── pages/
│
├── src-tauri/ # Rust + Tauri backend
│ ├── src/
│ └── vosk-model-small-en-us-0.15/ (ignored)
│
├── package.json
├── tauri.conf.json
└── README.md
```

---

## 🧩 Windows Users Note

Make sure the following runtime libraries exist in your system or `src-tauri` folder before building:

- libvosk.dll
- libvosk.lib
- libwinpthread-1.dll
- libgcc_s_seh-1.dll
- libstdc++-6.dll

> These are not included in the repo. They’ll be auto-bundled by Tauri when you build.

---
