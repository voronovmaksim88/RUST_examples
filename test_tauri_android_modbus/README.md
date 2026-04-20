# test_tauri_android_modbus

Приложение **Tauri 2** + **Vue 3** + **TypeScript** (шаблон `create-tauri-app`). Каталог лежит в монорепозитории [`RUST_examples`](../README.md). В Rust подключён **`tauri-plugin-opener`** (для Android в Gradle есть нюансы, см. раздел про Kotlin).

Ниже — **рекомендуемая IDE**, **сборка с нуля на другой машине** (краткий чеклист) и **подробные шаги** для Windows (десктоп и Android), по аналогии с [`test-tauri`](../test-tauri/README.md).

---

## С нуля на другой машине (краткий чеклист)

Выполняйте по порядку. Все команды терминала — из каталога **`test_tauri_android_modbus`**, если не указано иное.

1. **Клонировать** репозиторий `RUST_examples` и перейти в проект:
   ```bash
   git clone <url-репозитория> RUST_examples
   cd RUST_examples/test_tauri_android_modbus
   ```
2. Установить **Rust** (stable) с [rustup.rs](https://rustup.rs/), **Node.js** LTS с [nodejs.org](https://nodejs.org/), выполнить [системные требования Tauri для Windows](https://v2.tauri.app/start/prerequisites/) (**Microsoft C++ Build Tools**, **WebView2**).
3. Установить зависимости фронтенда:
   ```bash
   npm install
   ```
4. **Только для Android:** установить [Android Studio](https://developer.android.com/studio), SDK / NDK / Build-Tools (подробно в разделе **Android Studio и SDK** ниже), задать **`JAVA_HOME`**, **`ANDROID_HOME`**, **`NDK_HOME`**, при необходимости выполнить скрипт из корня репозитория:
   ```powershell
   pwsh -ExecutionPolicy Bypass -File ..\scripts\set-android-env.ps1
   ```
   Закрыть и снова открыть терминал, проверить: `Test-Path $env:NDK_HOME` → `True`.
5. **Только для Android:** включить **режим разработчика Windows** (симлинки для `jniLibs`; подробности в одноимённом разделе ниже).
6. **Только для Android:** добавить цели Rust:
   ```bash
   rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
   ```
7. **Только для Android:** если в репозитории **нет** каталога `src-tauri/gen/android` (например, его не закоммитили), инициализировать Android-проект:
   ```bash
   npm run tauri android init -- --ci
   ```
   Если каталог **уже есть** после `git clone`, этот шаг можно пропустить.
8. Проверка **десктоп:** `npm run tauri dev` или `npm run tauri build`.
9. Проверка **Android:** `npm run tauri android dev` (устройство/эмулятор и `adb devices`) или сборка APK (раздел **Сборка APK** ниже).

После этого машина готова к разработке и сборке так же, как у автора проекта.

---

## Рекомендуемая среда разработки

- [VS Code](https://code.visualstudio.com/) + [Vue — Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

---

## 1. Подготовка окружения (Windows)

1. Установите **Rust** (stable) с [rustup.rs](https://rustup.rs/) и перезапустите терминал.
2. Установите **Node.js** LTS с [nodejs.org](https://nodejs.org/) (нужен для фронтенда и `npm run tauri …`).
3. Выполните [системные требования Tauri для Windows](https://v2.tauri.app/start/prerequisites/): **Microsoft C++ Build Tools** (рабочая нагрузка «Разработка классических приложений на C++»), **WebView2** (часто уже есть в Windows 10/11).

---

## 2. Создание похожего приложения с нуля (если не этот репозиторий)

Для **нового** имени и папки (стек как здесь — Vue + TS):

```bash
npm create tauri-app@latest имя-проекта -- --manager npm --template vue-ts --yes --tauri-version 2
cd имя-проекта
npm install
```

Другие шаблоны: `vanilla-ts`, `react-ts`, `svelte-ts` и т.д. (`npx create-tauri-app --help`).

---

## 3. Локальная разработка (рабочий стол)

```bash
npm install
npm run tauri dev
```

---

## 4. Сборка под Windows (desktop)

```bash
npm run tauri build
```

Артефакты: `src-tauri/target/release/` и при необходимости `src-tauri/target/release/bundle/` (форматы зависят от `tauri.conf.json`).

---

## 5. Распространение desktop

1. В [конфигурации Tauri](https://v2.tauri.app/reference/config/) проверьте идентификатор, имя, иконки, версию. В этом проекте **`identifier`**: `com.maksim.test-tauri-android-modbus` (в bundle id **нельзя** использовать подчёркивания в сегментах — только буквы, цифры, точки и дефисы; иначе `tauri android build` завершится ошибкой).
2. Для установщиков на Windows настройте **код-подпись**, иначе SmartScreen может предупреждать.
3. Отдавайте файлы из `bundle` или публикуйте релизы (GitHub Releases и т.д.).

---

## 6. CLI: `tauri` и `cargo tauri`

- В проекте: **`npm run tauri dev`** / **`npm run tauri build`** / **`npm run tauri android …`**.
- Глобально: `npm install -g @tauri-apps/cli` → команда **`tauri`** в PATH.
- Или: `cargo install tauri-cli` → **`cargo tauri …`**.

---

## 7. Android

Официальные требования: [Configure for Mobile Targets → Android](https://v2.tauri.app/start/prerequisites/#android).

### Android Studio и SDK

1. Установите [Android Studio](https://developer.android.com/studio).
2. Откройте **SDK Manager**: **File → Settings** (или **Android Studio → Settings** на macOS) → слева **Languages & Frameworks → Android SDK**.  
   **Не** раздел **Settings → Tools** — там нет вкладки SDK Tools.
3. Вкладка **SDK Tools** (при необходимости **Show Package Details**):
   - **Android SDK Platform-Tools**
   - **Android SDK Build-Tools**
   - **NDK (Side by side)**
   - **Android SDK Command-line Tools**
4. На вкладке **SDK Platforms** при необходимости отметьте нужную версию платформы.
5. Проверка: в `%LOCALAPPDATA%\Android\Sdk` должна быть папка **`ndk\<версия>`**. Если её нет — Tauri сообщит *«Android NDK not found»*.

### Переменные среды (Windows)

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

Из каталога этого проекта: `..\scripts\set-android-env.ps1`.

Вручную: **Win+R → `sysdm.cpl` → Дополнительно → Переменные среды** — создайте переменные пользователя, затем **закройте и откройте терминал** заново.

Проверка:

```powershell
Test-Path $env:NDK_HOME   # должно быть True в новом окне PowerShell
```

### Цели Rust для Android

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### Режим разработчика Windows (симлинки)

При `android build` / `android dev` Tauri создаёт **символическую ссылку** на `.so` в `jniLibs`. Без прав на symlink сборка падает (*Creation symbolic link is not allowed*).

**Включите режим разработчика Windows:** **Параметры → Обновление и безопасность → Для разработчиков** (Windows 10) или **Параметры → Конфиденциальность и безопасность → Для разработчиков** (Windows 11) → **Режим разработчика**.

[Документация Microsoft](https://learn.microsoft.com/windows/apps/get-started/enable-your-device-for-development).

### Инициализация Android в проекте

Если после `git clone` **нет** `src-tauri/gen/android`:

```bash
npm run tauri android init -- --ci
```

Без Java/SDK/NDK команда завершится ошибкой — сначала настройте SDK и переменные среды.

### Kotlin, `tauri-plugin-opener` и два диска (Windows)

В `src-tauri/gen/android/gradle.properties` для этого репозитория задано **`kotlin.incremental=false`**. Иначе на Windows часто падает Kotlin daemon, если зависимости Cargo лежат на диске **`C:`** (например `C:\Users\…\.cargo\…`), а проект — на **`D:\`**. Если вы заново выполните `android init` и перезапишете `gen/android`, **верните эту строку** в `gradle.properties` или перенесите клон на тот же диск, что и домашний каталог с `.cargo`.

### Запуск на устройстве или эмуляторе

1. На телефоне: режим разработчика и **отладка по USB**, либо эмулятор **AVD**.
2. Проверка: `%ANDROID_HOME%\platform-tools\adb.exe devices`.

Для разработки по сети к хосту с Vite задайте **`TAURI_DEV_HOST`** (IP машины с `npm run dev`), см. [документацию Tauri по mobile](https://v2.tauri.app/develop/).

```bash
npm run tauri android dev
```

### Сборка APK

**Важно:** флаги вроде **`--apk`**, **`--ci`**, **`--debug`**, **`--split-per-abi`**, **`-t aarch64`** передаются **одной** группой после **`--`** у npm, **без второго `--`** перед ними. Второй `--` пробрасывает аргументы дальше (в т.ч. в `cargo`) и может сломать сборку.

Примеры:

```bash
# Несколько APK по ABI (debug)
npm run tauri android build -- --apk --ci --debug --split-per-abi

# Один debug APK только под ARM64
npm run tauri android build -- --apk --ci --debug -t aarch64

# Release (часто universal unsigned)
npm run tauri android build
```

Типичные пути к APK:

- `src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk`
- при `--split-per-abi`: подкаталоги `arm64-v8a`, `armeabi-v7a` и т.д.

**Release без подписи** на **Android 14+** часто **не устанавливается**. Для проверки на телефоне используйте **debug**-APK или настройте **подпись release** (keystore + signing в Gradle).

Поиск всех собранных APK:

```powershell
Get-ChildItem -Path "src-tauri\gen\android\app\build\outputs\apk" -Recurse -Filter *.apk
```

---

## Краткая последовательность

**Desktop (Windows):** Rust + Node + Build Tools + WebView2 → `git clone` → `cd test_tauri_android_modbus` → `npm install` → `npm run tauri dev` / `npm run tauri build`.

**Android:** всё из desktop → Android Studio (SDK, NDK) → `JAVA_HOME` / `ANDROID_HOME` / `NDK_HOME` → режим разработчика Windows → `rustup target add …` → при отсутствии каталога: `npm run tauri android init -- --ci` → `npm run tauri android dev` или сборка APK (см. выше).

**CI / iOS** здесь не расписаны.
