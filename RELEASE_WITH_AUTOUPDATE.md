# GTerm Release Process with Auto-Update

Това ръководство описва пълния процес за създаване на release с работещ auto-update.

## Предварителни изисквания

1. **GitHub CLI (`gh`)** инсталиран и конфигуриран
2. **Tauri signing key** генериран (виж по-долу)
3. Код е готов за release

## Стъпка 1: Подготовка на signing key (еднократно)

Ако нямаш signing key, генерирай го:

```bash
npm run tauri signer generate -- -w ~/.tauri/gterm.key
```

**ВАЖНО:**
- Запази файла `~/.tauri/gterm.key` на сигурно място
- Никога не го commit-вай в Git
- Public key-ят вече е в `tauri.conf.json`

## Стъпка 2: Обновяване на версията

1. Обнови версията в `src-tauri/tauri.conf.json`:
```json
{
  "version": "1.0.3"
}
```

2. Запази промените:
```bash
git add src-tauri/tauri.conf.json
git commit -m "Bump version to 1.0.3"
```

## Стъпка 3: Build на приложението

```bash
npm run tauri:build
```

Това ще създаде:
- DMG файл в `src-tauri/target/release/bundle/dmg/`
- APP bundle в `src-tauri/target/release/bundle/macos/`

## Стъпка 4: Подписване на app bundle

```bash
cd src-tauri/target/release/bundle/macos

# Създай tar.gz архив от .app
tar -czf GTerm.app.tar.gz GTerm.app

# Подпиши архива
npm run tauri signer sign GTerm.app.tar.gz -- -k ~/.tauri/gterm.key

# Това ще създаде GTerm.app.tar.gz.sig
```

## Стъпка 5: Генериране на latest.json

Върни се в root директорията на проекта:

```bash
cd /Users/g0shk3/Developer/GTerm

# Генерирай latest.json
./scripts/generate-update-manifest.sh
```

Скриптът ще създаде `latest.json` файл с правилната структура.

## Стъпка 6: Създаване на GitHub Release

```bash
# Създай git tag
git tag v1.0.3
git push origin v1.0.3

# Създай GitHub release
gh release create v1.0.3 \
  --title "Release v1.0.3" \
  --notes "## What's Changed

- Fixed auto-update functionality
- Updated release process

**Full Changelog**: https://github.com/g0shk3/GTerm/compare/v1.0.2...v1.0.3"
```

## Стъпка 7: Upload на файловете

```bash
# Upload app bundle и signature
gh release upload v1.0.3 \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig

# Upload latest.json (КРИТИЧНО за auto-update!)
gh release upload v1.0.3 latest.json

# Опционално: Upload DMG за ръчно теглене
gh release upload v1.0.3 \
  src-tauri/target/release/bundle/dmg/GTerm_1.0.3_aarch64.dmg
```

## Стъпка 8: Проверка

Провери, че всички файлове са качени:

```bash
gh release view v1.0.3
```

Трябва да видиш:
- ✅ `GTerm.app.tar.gz`
- ✅ `GTerm.app.tar.gz.sig`
- ✅ `latest.json`
- ✅ DMG файл (опционално)

## Тестване на Auto-Update

1. Инсталирай **предишната** версия (1.0.2)
2. Пусни приложението
3. При стартиране трябва да се появи диалог за update
4. Натисни "Update Now"
5. Приложението трябва да се рестартира с новата версия

## Структура на latest.json

```json
{
  "version": "v1.0.3",
  "notes": "See the release notes on GitHub",
  "pub_date": "2025-01-14T10:30:00Z",
  "platforms": {
    "darwin-x86_64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkK...",
      "url": "https://github.com/g0shk3/GTerm/releases/download/v1.0.3/GTerm.app.tar.gz"
    },
    "darwin-aarch64": {
      "signature": "dW50cnVzdGVkIGNvbW1lbnQ6IHNpZ25hdHVyZSBmcm9tIHRhdXJpIHNlY3JldCBrZXkK...",
      "url": "https://github.com/g0shk3/GTerm/releases/download/v1.0.3/GTerm.app.tar.gz"
    }
  }
}
```

## Troubleshooting

### Auto-update не работи

1. Провери че `latest.json` съществува:
   ```bash
   curl https://github.com/g0shk3/GTerm/releases/latest/download/latest.json
   ```

2. Провери конзолата в DevTools (Cmd+Option+I):
   ```javascript
   // Трябва да видиш update check logs
   ```

3. Провери че версията в `latest.json` е по-нова от текущата

### Signature verification failed

1. Убеди се, че използваш същия key за подписване
2. Провери че signature файлът е качен правилно
3. Провери че public key в `tauri.conf.json` е верен

## Бързи команди за цялостен release

Създай скрипт `release.sh`:

```bash
#!/bin/bash
set -e

VERSION=$1
if [ -z "$VERSION" ]; then
  echo "Usage: ./release.sh <version>"
  exit 1
fi

echo "Building version $VERSION..."
npm run tauri:build

cd src-tauri/target/release/bundle/macos
tar -czf GTerm.app.tar.gz GTerm.app
npm run tauri signer sign GTerm.app.tar.gz -- -k ~/.tauri/gterm.key
cd -

./scripts/generate-update-manifest.sh

git tag v$VERSION
git push origin v$VERSION

gh release create v$VERSION \
  --title "Release v$VERSION" \
  --generate-notes

gh release upload v$VERSION \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz \
  src-tauri/target/release/bundle/macos/GTerm.app.tar.gz.sig \
  latest.json

echo "✓ Release v$VERSION created successfully!"
```

Използване:
```bash
chmod +x release.sh
./release.sh 1.0.3
```

## Важни бележки

1. **Винаги** качвай `latest.json` - това е критичното за auto-update
2. **Никога** не качвай DMG с име `latest.dmg` - може да объркаш updater-а
3. Проверявай signature-а преди upload
4. Тествай на чисто приложение (без dev build)
5. Bump версията ПРЕДИ build

## Автоматизация с GitHub Actions (бъдещо)

TODO: Създай GitHub Action, който автоматично:
- Build-ва приложението
- Подписва го
- Генерира latest.json
- Създава release
- Upload-ва всички файлове
