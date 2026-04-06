опиши подробнее как сделать пункт 1# Пошаговый план развёртывания нового Tauri-приложения

План ориентирован на **Windows** и **Tauri 2**.

## 1. Подготовка окружения

1. Установите **Rust** (stable) с [rustup.rs](https://rustup.rs/) и перезапустите терминал.
2. Установите **Node.js** LTS с [nodejs.org](https://nodejs.org/) (нужен для фронтенда и скриптов `npm run tauri …`).
3. Выполните [системные требования Tauri для Windows](https://v2.tauri.app/start/prerequisites/): **Microsoft C++ Build Tools** (рабочая нагрузка «Разработка классических приложений на C++»), **WebView2** (обычно уже есть в Windows 10/11).

## 2. Создание нового приложения

1. Перейдите в каталог, где должен лежать проект.
2. Создайте проект одной командой (без интерактива):

   ```bash
   npm create tauri-app@latest имя-проекта -- --manager npm --template vanilla-ts --yes --tauri-version 2
   ```

   Другие шаблоны: `react-ts`, `vue-ts`, `svelte-ts` и т.д. (полный список: `npx create-tauri-app --help`).

3. Перейдите в папку проекта и установите зависимости:

   ```bash
   cd имя-проекта
   npm install
   ```

## 3. Локальная разработка

1. Запуск в режиме разработки:

   ```bash
   npm run tauri dev
   ```

2. Убедитесь, что окно открывается и что горячая перезагрузка фронта работает (если шаблон это поддерживает).

## 4. Сборка под продакшен (релизная сборка)

1. Соберите приложение:

   ```bash
   npm run tauri build
   ```

2. Артефакты появятся в `src-tauri/target/release/` и в выходной папке бандлера (часто `src-tauri/target/release/bundle/`: установщик `.msi`/`.exe` и т.п. — в зависимости от настроек в `tauri.conf.json`).

## 5. Распространение для пользователей

1. В [конфигурации Tauri](https://v2.tauri.app/reference/config/) проверьте идентификатор приложения, имя, иконки, версию.
2. Для установщиков на Windows настройте **код-подпись** (сертификат и CI), иначе SmartScreen может предупреждать пользователей.
3. Отдавайте пользователям файлы из `bundle` или публикуйте их как релизы (например, GitHub Releases / свой хостинг).

## 6. Альтернатива: только Cargo-CLI

Если нужно добавить Tauri в **уже существующий** фронтенд-проект, а не создавать шаблон с нуля:

1. Установите CLI: `cargo install tauri-cli`
2. В корне фронтенда выполните `cargo tauri init` (для неинтерактивного режима смотрите флаги: `cargo tauri init --help`).

**Замечание:** отдельная команда `tauri` в PATH появляется при глобальной установке `@tauri-apps/cli` (`npm install -g @tauri-apps/cli`). При установке через Cargo используйте **`cargo tauri`**. В проектах из `create-tauri-app` обычно вызывают **`npm run tauri dev`** / **`npm run tauri build`**.

## 7. Android: подготовка и запуск (`test-tauri`)

Официальные требования: [Configure for Mobile Targets → Android](https://v2.tauri.app/start/prerequisites/#android).

### 7.1. Установите Android Studio и компоненты SDK

1. Скачайте и установите [Android Studio](https://developer.android.com/studio).
2. В **SDK Manager** установите (вкладка *SDK Tools*, при необходимости включите *Show Package Details*):
   - **Android SDK Platform** (платформа, например для целевого API вашего устройства);
   - **Android SDK Platform-Tools**;
   - **Android SDK Build-Tools**;
   - **NDK (Side by side)**;
   - **Android SDK Command-line Tools**.

3. Проверка: в каталоге `%LOCALAPPDATA%\Android\Sdk` должна появиться папка **`ndk`** с вложенной папкой версии (например `26.1.10909125`). Если **`ndk` нет**, Tauri выдаст *«Android NDK not found»* — вернитесь в **SDK Tools**, включите **NDK (Side by side)**, **Apply** и дождитесь установки.

### 7.2. Переменные среды (Windows), подробно

Нужны три переменные: **`JAVA_HOME`**, **`ANDROID_HOME`**, **`NDK_HOME`**. Они должны указывать на **реальные папки** на диске (без кавычек в значении, без пробела в конце).

Готовый скрипт для записи этих переменных в профиль пользователя: **[scripts/set-android-env.ps1](../scripts/set-android-env.ps1)** (из корня репозитория: `pwsh -ExecutionPolicy Bypass -File .\scripts\set-android-env.ps1`).

#### Какие значения подставить

| Переменная       | Типичный путь | Как уточнить |
|------------------|---------------|--------------|
| **`JAVA_HOME`**  | `C:\Program Files\Android\Android Studio\jbr` | В Проводнике откройте `C:\Program Files\Android\Android Studio\` — внутри должна быть папка **`jbr`**. Если Studio в другом месте, найдите рядом с `Android Studio.exe` папку **`jbr`**. |
| **`ANDROID_HOME`** | `C:\Users\<ваш_логин>\AppData\Local\Android\Sdk` | В адресной строке Проводника введите `%LOCALAPPDATA%\Android\Sdk` и нажмите Enter — скопируйте **полный** путь из адресной строки после перехода. |
| **`NDK_HOME`**   | `...\Android\Sdk\ndk\<версия>` | Откройте `ANDROID_HOME`, затем папку **`ndk`**. Внутри одна или несколько папок с номером версии (например `26.1.10909125`). **`NDK_HOME` должен указывать на одну такую внутреннюю папку**, а не на `Sdk` и не просто на `ndk`. |

Если версий NDK несколько, можно взять **самую новую** по номеру папки или ту, что рекомендует Android Studio.

#### Как открыть окно переменных среды (Windows 10)

1. Нажмите **Win + R**, введите **`sysdm.cpl`**, нажмите Enter.  
   *(Другой путь: **Панель управления → Система → Дополнительные параметры системы**.)*
2. Вкладка **Дополнительно**.
3. Кнопка **Переменные среды…** внизу.

Откроются два списка: **переменные среды пользователя** (только для вашей учётной записи) и **системные**. Для разработки достаточно **пользовательских**.

#### Как добавить каждую переменную

Для **`ANDROID_HOME`** (если её ещё нет):

1. В блоке **«Переменные среды пользователя …»** нажмите **Создать…**.
2. **Имя переменной:** `ANDROID_HOME`
3. **Значение переменной:** полный путь к SDK, например `C:\Users\Maksim\AppData\Local\Android\Sdk`
4. **OK**

Повторите **Создать…** для **`NDK_HOME`** (значение — полный путь к **подпапке версии** внутри `ndk`, например `C:\Users\Maksim\AppData\Local\Android\Sdk\ndk\26.1.10909125`).

Повторите для **`JAVA_HOME`** (значение — путь к папке **`jbr`**, например `C:\Program Files\Android\Android Studio\jbr`).

Если переменная уже есть — выберите её → **Изменить…** и поправьте значение.

Закройте все окна кнопками **OK**, чтобы сохранить.

#### После сохранения

1. **Закройте** все окна PowerShell, CMD, терминала в Cursor и **откройте заново** (старые сессии не подхватывают новые переменные).
2. При необходимости **полностью закройте Cursor** и запустите снова.

#### Проверка в PowerShell

Откройте **новый** PowerShell и выполните:

```powershell
echo $env:JAVA_HOME
echo $env:ANDROID_HOME
echo $env:NDK_HOME
Test-Path $env:JAVA_HOME
Test-Path $env:ANDROID_HOME
Test-Path $env:NDK_HOME
```

Последние три строки должны вывести **`True`**. Если пусто или **`False`** — имя переменной опечатано, окно терминала было открыто до изменения среды, или путь неверный.

#### Временная установка только на текущее окно терминала

Если не хотите трогать систему, можно задать переменные только для одной сессии (после закрытия окна они пропадут):

```powershell
$env:JAVA_HOME = "C:\Program Files\Android\Android Studio\jbr"
$env:ANDROID_HOME = "$env:LOCALAPPDATA\Android\Sdk"
$env:NDK_HOME = (Get-ChildItem "$env:ANDROID_HOME\ndk" -Directory | Sort-Object Name -Descending | Select-Object -First 1).FullName
```

Проверьте, что `$env:NDK_HOME` не пустой: `echo $env:NDK_HOME`.

### 7.3. Цели Rust для Android

Один раз выполните (если ещё не сделали):

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
```

### 7.4. Инициализация Android в проекте

Из корня приложения (например `test-tauri`):

```bash
cd D:\MyProgGit\RUST_examples\test-tauri
npm run tauri android init
```

Интерактивные вопросы можно пропустить: `npm run tauri android init -- --ci`.

Без **Java** и корректных **`JAVA_HOME` / SDK** команда завершится ошибкой — сначала выполните шаги 7.1–7.2.

### 7.5. Запуск на устройстве или эмуляторе

1. Включите на телефоне **режим разработчика** и **отладку по USB**, подключите кабелем **или** запустите **AVD** в Android Studio.
2. Убедитесь, что `adb devices` видит устройство (Platform-Tools должны быть в `PATH` или вызывайте `%ANDROID_HOME%\platform-tools\adb.exe`).

Запуск в режиме разработки:

```bash
npm run tauri android dev
```

Сборка релиза (APK/AAB):

```bash
npm run tauri android build -- --apk --ci
```

APK обычно появляется под `src-tauri/gen/android/app/build/outputs/apk/` (поиск по `*.apk`).

### 7.6. Windows: «Creation symbolic link is not allowed»

При `android build` / `android dev` Tauri создаёт **символическую ссылку** из собранного `.so` в `app/src/main/jniLibs/`. На Windows без прав на symlink сборка падает с этой ошибкой.

**Рекомендуется:** включить **режим разработчика** Windows:

- **Windows 10:** **Параметры → Обновление и безопасность → Для разработчиков** → включить **Режим разработчика**.
- **Windows 11:** **Параметры → Конфиденциальность и безопасность → Для разработчиков** → **Режим разработчика**.

Подробнее: [Включение режима разработчика (Microsoft)](https://learn.microsoft.com/windows/apps/get-started/enable-your-device-for-development).

После включения перезапустите терминал и повторите `npm run tauri android build -- --apk --ci`.

Альтернатива (менее удобно): выдать учётной записи политику **Create symbolic links** или запускать сборку из процесса с правами администратора — обычно проще включить режим разработчика.

---

## Краткая последовательность

Prerequisites → `npm create tauri-app` → `npm install` → `npm run tauri dev` → `npm run tauri build` → раздача артефактов из `bundle` (и при необходимости подпись).

**Android:** Android Studio + SDK/NDK + `JAVA_HOME` / `ANDROID_HOME` / `NDK_HOME` → `rustup target add …` → `npm run tauri android init` → `npm run tauri android dev`.

Для сценария **CI (GitHub Actions)** или **iOS** план задаётся отдельно.
