# test-tauri

Приложение **Tauri 2** + **Vanilla TypeScript** (шаблон `create-tauri-app`). Каталог лежит в монорепозитории [`RUST_examples`](../README.md).

В этом файле собраны и **быстрый старт для IDE**, и **пошаговый план** (Windows, рабочий стол и Android) по итогам реальной настройки.

---

## Рекомендуемая среда разработки

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Все команды ниже для терминала выполняйте из каталога **`test-tauri`** (если не указано иное).

---

## 1. Подготовка окружения (Windows)

1. Установите **Rust** (stable) с [rustup.rs](https://rustup.rs/) и перезапустите терминал.
2. Установите **Node.js** LTS с [nodejs.org](https://nodejs.org/) (нужен для фронтенда и `npm run tauri …`).
3. Выполните [системные требования Tauri для Windows](https://v2.tauri.app/start/prerequisites/): **Microsoft C++ Build Tools** (рабочая нагрузка «Разработка классических приложений на C++»), **WebView2** (часто уже есть в Windows 10/11).

---

## 2. Создание нового приложения (если начинаете с нуля)

**Этот каталог уже является готовым проектом.** Для **нового** имени и папки:

```bash
npm create tauri-app@latest имя-проекта -- --manager npm --template vanilla-ts --yes --tauri-version 2
cd имя-проекта
npm install
```

Другие шаблоны: `react-ts`, `vue-ts`, `svelte-ts` и т.д. (список: `npx create-tauri-app --help`).

---

## 3. Локальная разработка (рабочий стол)

```bash
npm run tauri dev
```

---

## 4. Сборка под Windows (desktop)

```bash
npm run tauri build
```

Артефакты: `src-tauri/target/release/` и часто `src-tauri/target/release/bundle/` (`.msi`/`.exe` — по настройкам `tauri.conf.json`).

---

## 5. Распространение desktop

1. В [конфигурации Tauri](https://v2.tauri.app/reference/config/) проверьте идентификатор, имя, иконки, версию.
2. Для установщиков на Windows настройте **код-подпись**, иначе SmartScreen может предупреждать.
3. Отдавайте файлы из `bundle` или публикуйте релизы (GitHub Releases и т.д.).

---

## 6. CLI: `tauri` и `cargo tauri`

- В проектах из `create-tauri-app` обычно: **`npm run tauri dev`** / **`npm run tauri build`**.
- Глобально: `npm install -g @tauri-apps/cli` → команда **`tauri`** в PATH.
- Или: `cargo install tauri-cli` → команды **`cargo tauri …`**.

Чтобы добавить Tauri в **существующий** фронтенд: `cargo tauri init` (флаги: `cargo tauri init --help`).

---

## 7. Android

Официальные требования: [Configure for Mobile Targets → Android](https://v2.tauri.app/start/prerequisites/#android).

### 7.1. Android Studio и SDK

1. Установите [Android Studio](https://developer.android.com/studio).
2. Откройте **SDK Manager** так: **File → Settings** (или **Android Studio → Settings** на macOS) → слева **Languages & Frameworks → Android SDK**.  
   **Не** раздел **Settings → Tools** — там нет вкладки SDK Tools.
3. Вкладка **SDK Tools** (при необходимости **Show Package Details**):
   - **Android SDK Platform-Tools**
   - **Android SDK Build-Tools**
   - **NDK (Side by side)**
   - **Android SDK Command-line Tools**
4. На вкладке **SDK Platforms** при необходимости отметьте нужную версию платформы.
5. Проверка: в `%LOCALAPPDATA%\Android\Sdk` должна быть папка **`ndk\<версия>`**. Если её нет — Tauri сообщит *«Android NDK not found»*.

### 7.2. Переменные среды (Windows)

Нужны **`JAVA_HOME`**, **`ANDROID_HOME`**, **`NDK_HOME`** — пути к реальным каталогам (без лишних пробелов и кавычек в значениях в UI).

| Переменная | Типичное значение |
|------------|-------------------|
| `JAVA_HOME` | `C:\Program Files\Android\Android Studio\jbr` |
| `ANDROID_HOME` | `C:\Users\<логин>\AppData\Local\Android\Sdk` |
| `NDK_HOME` | `...\Sdk\ndk\<номер_версии>` (внутренняя папка версии, не просто `ndk`) |

**Скрипт** (из корня монорепозитория `RUST_examples`):

```powershell
pwsh -ExecutionPolicy Bypass -File .\scripts\set-android-env.ps1
```

Файл: [`scripts/set-android-env.ps1`](../scripts/set-android-env.ps1) (из `test-tauri`: `..\scripts\set-android-env.ps1`).

Подробно вручную: **Win+R → `sysdm.cpl` → Дополнительно → Переменные среды** — создайте переменные пользователя, затем **закройте и откройте терминал** заново.

Проверка:

```powershell
Test-Path $env:NDK_HOME   # должно быть True в новом окне PowerShell
```

### 7.3. Цели Rust для Android

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 7.4. Режим разработчика Windows (симлинки)

При `android build` / `android dev` Tauri создаёт **символическую ссылку** на `.so` в `jniLibs`. Без прав на symlink сборка падает (*Creation symbolic link is not allowed*).

**Включите режим разработчика Windows:** **Параметры → Обновление и безопасность → Для разработчиков** (Windows 10) или **Параметры → Конфиденциальность и безопасность → Для разработчиков** (Windows 11) → **Режим разработчика**.

[Документация Microsoft](https://learn.microsoft.com/windows/apps/get-started/enable-your-device-for-development).

### 7.5. Инициализация Android в проекте

Из каталога `test-tauri`:

```bash
npm run tauri android init -- --ci
```

Без Java/SDK/NDK команда завершится ошибкой — сначала п. 7.1–7.2.

### 7.6. Запуск на устройстве или эмуляторе

1. На телефоне: режим разработчика и **отладка по USB**, либо эмулятор **AVD**.
2. Проверка: `%ANDROID_HOME%\platform-tools\adb.exe devices`.

```bash
npm run tauri android dev
```

### 7.7. Сборка APK

**Важно:** флаги **`--apk`**, **`--ci`**, **`--debug`**, **`--split-per-abi`**, **`-t aarch64`** передаются **одной** группой после первого `--` у npm, **без второго `--`** перед ними. Второй `--` пробрасывает аргументы дальше (в т.ч. в `cargo`) и ломает сборку — например `--split-per-abi` окажется не на своём месте.

Примеры из каталога `test-tauri`:

```bash
# Несколько APK по ABI (debug)
npm run tauri android build -- --apk --ci --debug --split-per-abi

# Один debug APK только под ARM64 — удобно для Samsung Galaxy A52 (SM-A525F) и аналогов
npm run tauri android build -- --apk --ci --debug -t aarch64
```

Готовый файл для A52:

`src-tauri/gen/android/app/build/outputs/apk/arm64/debug/app-arm64-debug.apk`

**Release без подписи:** в `universal/release` часто бывает `app-universal-release-unsigned.apk`. Такой пакет на **Android 14** часто **не устанавливается**. Для проверки на телефоне используйте **debug**-APK (подписан debug-ключом) или настройте **подпись release** (keystore + signing в Gradle).

Поиск всех собранных APK:

```powershell
Get-ChildItem -Path "src-tauri\gen\android\app\build\outputs\apk" -Recurse -Filter *.apk
```

---

## Краткая последовательность

**Desktop (Windows):** Rust + Node + Build Tools + WebView2 → `npm install` → `npm run tauri dev` → `npm run tauri build`.

**Android:** Android Studio (SDK, NDK) → `JAVA_HOME` / `ANDROID_HOME` / `NDK_HOME` → режим разработчика Windows → `rustup target add …` → `npm run tauri android init -- --ci` → `npm run tauri android dev` или сборка APK (п. 7.7).

**CI / iOS** здесь не расписаны.
