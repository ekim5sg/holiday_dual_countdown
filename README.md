# holiday_dual_countdown
This project was created to bring some festive fun into the browser — tracking not just one event, but two simultaneous countdowns

🎄🎉 Holiday Dual Countdown

A Rust + Yew + WASM app that counts down to TWO celebrations at once!

This project was created to bring some festive fun into the browser — tracking not just one event, but two simultaneous countdowns (for example: 🎅 Christmas & 🎆 New Year). It’s perfect for holiday parties, digital signage, classroom screens, SharePoint/Teams dashboards, or as a playful coding demo for Rust + WebAssembly.

Built with Rust 🦀, Yew, and Trunk, and deployed as a lightweight static web app, this runs in any modern browser — no backend needed!

✨ Features
Feature	Details
🕒 Dual Countdown Timers	Displays time remaining to two independent upcoming events
📱 Responsive UI	Mobile, desktop, and large display layouts supported
🎨 Seasonal Flair	Optional holiday emojis, colors, and themes
⚡ Fast & Lightweight	All logic runs in-browser via WebAssembly
🌐 Easy Hosting	Deploy anywhere static HTML is supported: Hostek, GitHub Pages, Netlify, etc.
🛠️ Configurable	Update labels, target dates, and themes with simple code changes
🧩 Tech Stack
Component	Technology
Language	🦀 Rust
Framework	🧷 Yew
Frontend Runtime	🧪 WebAssembly (WASM)
Build Tool	🛠️ trunk
📂 Project Structure
holiday_dual_countdown/
├── /src
│   ├── main.rs          # Core Yew app logic
├── index.html
├── Trunk.toml
├── Cargo.toml
└── README.md

🚀 Getting Started
1️⃣ Install Rust Target + Tools
rustup target add wasm32-unknown-unknown
cargo install trunk

2️⃣ Run Locally
trunk serve


The app should open automatically in your browser at:

http://127.0.0.1:8080/

3️⃣ Build for Production
trunk build --release


Output is stored in:

/dist

🎁 Configuring the Countdown Dates

Inside main.rs, set the targets:

let holiday_one = "December 25, 2025 00:00:00";
let holiday_two = "January 1, 2026 00:00:00";


Adjust labels, icons, or emoji as desired:

let label_one = "🎄 Christmas";
let label_two = "🎆 New Year";

🌎 Deployment

This app works on any static hosting platform:

✔ Hostek

✔ GitHub Pages

✔ Netlify

✔ Cloudflare Pages

✔ Azure Static Web Apps

✔ Local kiosk screen

Just upload the contents of the dist/ folder.

Example Hostek path:

/public_html/holiday_dual_countdown/


Make sure your server supports the .wasm MIME type:

application/wasm

❄ Future Enhancements (Optional)

 Countdown chimes or sound effects

 Theme selector (Christmas / Hanukkah / Lunar New Year / Birthday / Graduation)

 Confetti burst when timer reaches zero 🎊

 Auto-switch to celebratory message when complete

 GIF / SVG animation background

 User-entered custom dates + saved settings (localStorage)

📸 Live Demo

🔗 Demo link placeholder (update after deployment):

https://www.webhtml5.info/holiday_dual_countdown/

👨‍🚀 Built By

Created with care (and Christmas cookies) by:

Michael Givens (MikeGyver / Vibechemist)
NASA Contractor • Rust + Python Builder • Holiday Tech Experimenter

“Code is better with cocoa. Projects are better when shared. And countdowns are better when they sparkle.” ❄✨
