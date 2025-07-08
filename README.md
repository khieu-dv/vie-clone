
# VieClone - Cross-platform Video Downloader

**VieClone** is a modern desktop video downloader powered by [Tauri](https://tauri.app/) and [Next.js](https://nextjs.org/). It allows users to download single videos or playlists using `yt-dlp` and `ffmpeg` in a simple, elegant interface.

## 🔗 Download

You can download the latest Windows release (.exe) here:

➡️ [Download VieClone for Windows](https://github.com/your-username/vieclone/releases/latest/download/VieClone-setup.exe)


## 🚀 Features

- ✅ Cross-platform (Windows, macOS, Linux)
- 🎞 Supports both video and playlist URLs
- 📁 Choose output directory and filename
- 🌓 Supports dark and light themes
- ⚙️ Installs `yt-dlp` and `ffmpeg` automatically
- 💡 Lightweight Tauri + React frontend

---

## 🛠 Requirements

Make sure the following are installed on your system:

- [Node.js](https://nodejs.org/) (v18 or later)
- [Rust & Cargo](https://rustup.rs/)
- npm (comes with Node.js)


---

## 📦 Install & Setup

```bash
git clone https://github.com/khieu-dv/vieclone.git
cd vieclone
npm install
````

---

## 🧪 Run in Development Mode

To launch the Tauri + Next.js app for development:

```bash
npx tauri dev
```

This will:

* Build the Next.js frontend
* Launch the Tauri window
* Automatically install `yt-dlp` and `ffmpeg` if needed

---

## 🏗 Build for Production

To generate a production-ready desktop application:

```bash
npm run build         # Build Next.js frontend
npx tauri build       # Build Tauri app for your OS
```

You will find the final binary (e.g., `.exe`, `.dmg`, `.AppImage`) in:

```
src-tauri/target/release/bundle/
```

---

## 📂 Project Structure

```
vieclone/
├── src-tauri/           # Tauri Rust backend
├── src/                 # Next.js frontend (App Router)
│   ├── app/             # Pages
│   ├── lib/tauri/       # Tauri bindings
│   └── ui/              # UI components
├── public/              # Static files
├── package.json
├── tauri.conf.json
└── README.md
```

---

## 🐞 Common Issues

* **yt-dlp/ffmpeg not found:**
  These tools will auto-install at runtime. If errors persist, try restarting the app or deleting temporary `.cache` folders.

* **Permission Errors in Tauri:**
  Ensure your `tauri.conf.json` allows access to Tauri APIs like `window`, `process`, or `shell`.



## 🙌 Acknowledgements

* [yt-dlp](https://github.com/yt-dlp/yt-dlp)
* [ffmpeg](https://ffmpeg.org/)
* [Tauri](https://tauri.app/)
* [Next.js](https://nextjs.org/)

