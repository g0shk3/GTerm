# GTerm - Release & Update Guide

Пълна стъпка-по-стъпка инструкция за публикуване на нова версия с auto-update.

---

## 📋 Преди да започнеш

**Важна информация:**
- Private key: `~/.tauri/gterm-keys`
- Private key парола: **********
- ⚠️ **НЕ commit-вай private key в Git!**

---

## 🚀 Стъпки за нов release

### Стъпка 1: Промени версията

Отвори и промени версията в **3 файла**:

**1.1. `package.json`**
```json
{
  "version": "1.0.1"  // Промени тук
}
```

**1.2. `src-tauri/tauri.conf.json`**
```json
{
  "version": "1.0.1"  // Промени тук
}
```

**1.3. `src-tauri/Cargo.toml`**
```toml
[package]
version = "1.0.1"  # Промени тук
```

---

### Стъпка 2: Commit промените

```bash
git add .
git commit -m "Bump version to 1.0.1"
git push
```

---

### Стъпка 3: Build на апликацията

**3.1. Set environment variables за signing:**

```bash
export TAURI_SIGNING_PRIVATE_KEY=$(cat ~/.tauri/gterm-keys)
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=********
```

⚠️ **Важно:** Използвай **single quotes** `'` за паролата!

**3.2. Build:**

```bash
npm run tauri:build
```

Това отнема 5-10 минути. Изчакай да приключи.

---

### Стъпка 4: Създай update файлове

**4.1. Navigate към build директорията:**

```bash
cd src-tauri/target/release/bundle/dmg
```

**4.2. Провери създадените файлове:**

```bash
ls -lh
```

Трябва да видиш:
- `GTerm_1.0.1_aarch64.dmg` (за инсталация)

**4.3. Създай tar.gz архив:**

```bash
tar -czf GTerm_1.0.1_aarch64.dmg.tar.gz GTerm_1.0.1_aarch64.dmg
```

**4.4. Подпиши архива:**

```bash
tauri signer sign GTerm_1.0.1_aarch64.dmg.tar.gz --private-key-path ~/.tauri/gterm-keys
```

Ще те попита за парола → въведи: ********

Това създава: `GTerm_1.0.1_aarch64.dmg.tar.gz.sig`

**4.5. Провери създадените файлове:**

```bash
ls -lh | grep -E "(1.0.1|tar.gz|sig)"
```

Трябва да видиш:
- ✅ `GTerm_1.0.1_aarch64.dmg`
- ✅ `GTerm_1.0.1_aarch64.dmg.tar.gz`
- ✅ `GTerm_1.0.1_aarch64.dmg.tar.gz.sig`

---

### Стъпка 5: Създай latest.json

**5.1. Извади signature от .sig файла:**

```bash
cat GTerm_1.0.1_aarch64.dmg.tar.gz.sig
```

Копирай **целия output** (това е signature-а).

**5.2. Създай latest.json файл:**

Използвай текст редактор или:

```bash
nano latest.json
```

Постави следното съдържание (замени стойностите):

```json
{
  "version": "1.0.1",
  "notes": "Bug fixes and improvements\n\n- Fixed connection issues\n- Improved performance",
  "pub_date": "2025-01-14T17:00:00Z",
  "platforms": {
    "darwin-aarch64": {
      "signature": "ПОСТАВИ_SIGNATURE_ТУК_ОТ_.sig_ФАЙЛА",
      "url": "https://github.com/g0shk3/GTerm/releases/download/v1.0.1/GTerm_1.0.1_aarch64.dmg.tar.gz"
    }
  }
}
```

**Промени:**
- `version` → новата версия (1.0.1)
- `notes` → какво е ново в тази версия
- `pub_date` → текуща дата/час (формат ISO 8601)
- `signature` → копирай целия signature от стъпка 5.1
- `url` → смени версията на v1.0.1 и GTerm_1.0.1

Запази файла (`Ctrl+O`, `Enter`, `Ctrl+X` за nano).

**5.3. Провери latest.json:**

```bash
cat latest.json
```

Убедете се че е валиден JSON.

---

### Стъпка 6: Създай GitHub Release

**6.1. Отиди на:**

```
https://github.com/g0shk3/GTerm/releases/new
```

**6.2. Попълни формата:**

- **Tag version:** `v1.0.1` (винаги започва с `v`)
- **Release title:** `v1.0.1 - Bug Fixes`
- **Description:**
  ```
  🔧 Bug fixes and improvements

  ## What's new:
  - Fixed connection stability issues
  - Improved terminal performance
  - UI improvements

  ## Installation:
  Download `GTerm_1.0.1_aarch64.dmg` below
  ```

**6.3. Upload файловете:**

Scroll надолу до **"Attach binaries..."** и качи:

1. ✅ `GTerm_1.0.1_aarch64.dmg` (за инсталация)
2. ✅ `GTerm_1.0.1_aarch64.dmg.tar.gz` (за auto-update)
3. ✅ `GTerm_1.0.1_aarch64.dmg.tar.gz.sig` (signature)
4. ✅ `latest.json` (update metadata)

**6.4. Публикувай:**

Натисни **"Publish release"** (не слагай checkmark на "Set as pre-release")

---

## ✅ Готово!

След публикуване:
- Users с версия 1.0.0 ще видят update notification при следващо стартиране
- Auto-update ще работи автоматично
- Новите users могат да свалят `.dmg` файла директно

---

## 🧪 Тестване на update

**За да тестваш дали update-ът работи:**

1. Инсталирай старата версия (1.0.0)
2. Стартирай апликацията
3. Трябва да видиш dialog: "Update to 1.0.1 is available!"
4. Натисни "Update Now"
5. Апликацията ще се update-не и рестартира

---

## 📝 Бързи команди (копирай и пастни)

### За build и signing на нова версия:

```bash
# Set signing vars
export TAURI_SIGNING_PRIVATE_KEY=$(cat ~/.tauri/gterm-keys)
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD=*******

# Build
npm run tauri:build

# Navigate
cd src-tauri/target/release/bundle/dmg

# Create tar.gz
tar -czf GTerm_X.X.X_aarch64.dmg.tar.gz GTerm_X.X.X_aarch64.dmg

# Sign
tauri signer sign GTerm_X.X.X_aarch64.dmg.tar.gz --private-key-path ~/.tauri/gterm-keys

# Show signature
cat GTerm_X.X.X_aarch64.dmg.tar.gz.sig
```

**Замени X.X.X с реалната версия!**

---

## ⚠️ Важни бележки

1. **Винаги commit-вай промените** преди build
2. **Пази паролата в тайна** - не я споделяй
3. **Не commit-вай private key** в Git
4. **GitHub Release винаги трябва да е "Latest"** за да работи auto-update
5. **latest.json файлът е критичен** - проверявай го преди upload
6. **URL в latest.json трябва да съвпада** с реалния URL на файла в GitHub

---

## 🆘 Проблеми?

### "Wrong password for that key"
- Използвай single quotes `'` за паролата, не double quotes `"`

### "File not found"
- Провери дали си в правилната директория: `src-tauri/target/release/bundle/dmg`

### Update не се показва
- Провери `latest.json` - URL и signature трябва да са правилни
- Провери дали release-ът е публикуван и е "Latest"
- Провери network в browser console (F12)

### Build грешка
- Изтрий `target` директорията: `rm -rf src-tauri/target`
- Build отново

---

**Версия на гида:** 1.0.0
**Последна промяна:** 2025-01-14
