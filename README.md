<h1 style="text-align: center">
Vue Desktop App Boilerplate 

<div style="display: flex; justify-content: center; gap: 20px; margin-top: 20px">
    <a href="https://tauri.app" alt="Tauri" target="_blank"> 
        <img src="/docs/assets/tauri.svg" style="width: 24px">
    </a>
    <a href="https://vite.dev" alt="Vite" target="_blank">
        <img src="/docs/assets/vite.svg" style="width: 24px">
    </a>
    <a href="https://vuejs.org" alt="Vue" target="_blank">
        <img src="/docs/assets/vue.svg" style="width: 24px">
    </a>
    <a href="https://www.typescriptlang.org" alt="Typescript" target="_blank">
        <img src="/docs/assets/ts.svg" style="width: 24px">
    </a>
</div>
</h1>

This desktop boilerplate is using tauri framework and vue 3 frontend view.
## Feature
* Auto Update
* Auto Startup
* Auto Fullscreen
* Status Network
* Logging
* File System
* Crash catch With Sentry

## Getting Started

### Prerequisites
In order to get started building project first need to install a few dependencies:

* [System Dependencies](https://v2.tauri.app/start/prerequisites/#system-dependencies)
* [Rust](https://v2.tauri.app/start/prerequisites/#rust)
* [Bun](https://bun.sh)
* [Node.js](https://nodejs.org/) (version >= 18.17.0)
* Docker

### 1. Clone the repository


### 2. Install npm dependencies
```sh
bun install
```
### 3. Copy the environment variables to .env and change the values
```sh
cp .env.example .env
```
### 4. Run the dev server
Run only server
```sh
bun dev
```
Run with desktop
```sh
bun tauri dev
```
### 5. Build desktop
```sh
bun tauri build
```

## Documentation

If you want more information about this project you should open [docs]()

[//]: # ()
[//]: # (Project Structure)

[//]: # ()
[//]: # (Commands)

[//]: # ()
[//]: # ( Tecnology use)

[//]: # ()
[//]: # (| Tecnology                                              | Description                   |)

[//]: # (|--------------------------------------------------------|-------------------------------|)

[//]: # (| [vue 3]&#40;https://vuejs.org/&#41;                            | Javascript Framework          |)

[//]: # (| [Tailwind CSS]&#40;https://tailwindcss.com/&#41;               | A utility-first CSS framework |)

[//]: # (| [Flowbite]&#40;https://flowbite.com/&#41;                      | Components use Tailwind CSS   |)

[//]: # (| [Heroicons]&#40;https://heroicons.com/&#41;                    | Advanced Table                |)

[//]: # (| [Pinia]&#40;https://pinia.vuejs.org/ &#41;                     | State Management              |)

[//]: # (| [vue-axios]&#40;https://www.npmjs.com/package/vue-axios&#41;   | Http Client                   |)

[//]: # (| [vue-i18n]&#40;https://kazupon.github.io/&#41;                 | Internationalization Plugin   |)

[//]: # (auto start)

[//]: # (network)

[//]: # (auto update)

[//]: # (shutdown)

[//]: # (logging)
[//]: # (Stronghold)

