# Tauri v2 Context Menu Test

This is a minimally reproducible repository for testing the Context Menu on Tauri v2.

This code runs fine on a Mac. 

But fails on Windows 10. Output of `tauri info` -
```
[✔] Environment
    - OS: Windows 10.0.19045 X64
    ✔ WebView2: 123.0.2420.97
    ✔ MSVC:
        - Visual Studio Build Tools 2022  
        - Visual Studio Community 2022    
    ✔ rustc: 1.77.0 (aedd173a2 2024-03-17)
    ✔ cargo: 1.77.0 (3fe68eabf 2024-02-29)
    ✔ rustup: 1.27.0 (bbb9276d2 2024-03-08)
    ✔ Rust toolchain: stable-x86_64-pc-windows-msvc (default)
    - node: 20.9.0
    - npm: 10.1.0

[-] Packages
    - tauri [RUST]: 2.0.0-beta.15
    - tauri-build [RUST]: 2.0.0-beta.12
    - wry [RUST]: 0.39.1
    - tao [RUST]: 0.27.0
    - @tauri-apps/api [NPM]: 2.0.0-beta.8
    - @tauri-apps/cli [NPM]: 2.0.0-beta.13

[-] App
    - build-type: bundle
    - CSP: unset
    - frontendDist: ../dist
    - devUrl: http://localhost:1420/
    - bundler: Vite
```