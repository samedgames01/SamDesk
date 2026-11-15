# SamDesk

RustDesk 1.5.1 tabanlı, SamDesk adıyla markalanmış sürüm. RustDesk'in genel sunucularını kullanır.

## Neler değişti

| Dosya | Değişiklik |
|---|---|
| `src/brand.rs` (yeni) | Uygulama adı (`SamDesk`) ve varsayılan güvenlik ayarları. Markayla ilgili her şey burada. |
| `src/lib.rs` | `mod brand;` satırı |
| `src/common.rs` | `load_custom_client()` başında `crate::brand::apply();` satırı |
| `flutter/windows/runner/Runner.rc` | Exe özelliklerindeki ürün adı ve açıklama |
| `Cargo.toml` | DLL özelliklerindeki ürün adı ve açıklama |
| `flutter/lib/desktop/widgets/tabbar_widget.dart` | Pencere başlık çubuğu artık sabit "RustDesk" yerine uygulama adını yazıyor |

Ad değişince aşağıdakiler kendiliğinden SamDesk olur:

- Pencere başlıkları, tepsi simgesi ve arayüz metinleri
- Kurulum klasörü (`C:\Program Files\SamDesk`), kurulu exe (`SamDesk.exe`), Windows servisi ve kısayollar
- Ayar klasörü (`%APPDATA%\SamDesk`). Bu yüzden aynı bilgisayardaki resmi RustDesk ile ayarlar karışmaz.
- Otomatik güncelleme kontrolü kapanır. Böylece uygulama kullanıcıya resmi RustDesk'i indirmeyi önermez.

Ana ekranda "Powered by RustDesk" yazısı görünür. RustDesk AGPL-3.0 lisanslıdır: SamDesk'i başkalarına dağıtırsan kaynak kodunu da paylaşman gerekir.

## Sadece senin izin verdiğin kişiler bağlansın

`src/brand.rs` içinde iki varsayılan ayar var:

- **Sadece kalıcı şifre:** Rastgele geçici şifre gösterilmez. Kalıcı şifre belirlenmeden cihaza kimse bağlanamaz.
- **Sadece şifreyle kabul:** "Kabul et" penceresi çıkmaz, yani biri kullanıcıyı kandırıp tıklatamaz.

Bunlar sadece varsayılan değerlerdir. Her cihazda Ayarlar > Güvenlik'ten değiştirilebilir.

Bağlanılacak her cihazda şunları yap:

1. Ayarlar > Güvenlik > **Kalıcı şifre** belirle (uzun ve güçlü olsun).
2. **"Use ID whitelisting"** seçeneğini aç ve kendi bağlanacağın cihazların ID'lerini ekle. `*` ve `?` joker karakterleri desteklenir.
3. İstersen **2FA**'yı da aç.

ID'yi bağlanan taraf kendisi bildirir. Beyaz liste riski azaltır ama asıl kilit şifredir.

İstersen beyaz listeyi doğrudan koda da gömebilirsin. Böylece her kurulum yalnızca bu ID'leri kabul eder. `src/brand.rs` içindeki tabloya şu satırı ekle:

```rust
(keys::OPTION_ID_WHITELIST, "123456789,987654321"),
```

## Derleme (GitHub Actions)

Bu klasör bir git deposu değil ve `libs/hbb_common` alt modülü boş. Kodu GitHub'a gönderirken alt modülü bağlaman gerekiyor. Boş klasör önce silinmeli, yoksa `git submodule add` hata verir.

```bash
cd /d/Remote
```

```bash
rmdir libs/hbb_common
```

```bash
git init -b master
```

```bash
git submodule add https://github.com/rustdesk/hbb_common libs/hbb_common
```

```bash
git add -A && git commit -m "SamDesk"
```

```bash
git remote add origin https://github.com/<kullanici-adin>/samdesk.git
```

```bash
git push -u origin master
```

Bu komutlar `hbb_common`'ın güncel halini çeker. Derleme `hbb_common` kaynaklı hatayla durursa alt modülü daha eski bir commit'e sabitlemek gerekir.

Ardından:

1. GitHub'da Settings > Actions > General > Workflow permissions bölümünden **Read and write** seç.
2. Actions sekmesinde **Flutter Nightly Build** > **Run workflow** ile derlemeyi başlat.
3. Windows çıktısı **Releases > nightly** altında `rustdesk-<sürüm>-x86_64.exe` adıyla oluşur. Dosyayı istediğin gibi yeniden adlandırabilirsin.

Notlar:

- Derleme tüm platformları dener. macOS ve Android imza anahtarı gerektirdiği için hata verebilir. Windows çıktısı bundan etkilenmez.
- Nightly iş akışı sadece elle başlatıldığında çalışır, her gece kendiliğinden çalışmaz.
- Koddaki her gönderim (push) "CI" ve "Full Flutter CI" testlerini başlatır. Bunlar kurulum dosyası üretmez. İstersen Actions'ta iptal edebilir ya da commit mesajına `[skip ci]` ekleyebilirsin.
- Depo herkese açıksa Actions dakikaları ücretsizdir. Özel depoda Windows dakikaları kotadan iki kat düşer.
- **MSI paketini kullanma.** MSI kendi adlandırmasını kullanır ve "RustDesk" olarak kalır. `.exe` kurulum dosyası doğru şekilde SamDesk olarak kurulur.

## Logo (sonra)

| Dosya | Nerede görünür |
|---|---|
| `flutter/windows/runner/resources/app_icon.ico` | Exe ve görev çubuğu simgesi |
| `res/icon.ico`, `res/tray-icon.ico` | Kurulum dosyası ve tepsi simgesi |
| `flutter/assets/icon.svg` | Uygulama içindeki küçük simge |
| `flutter/assets/icon.ico` (yeni) | Kurulumdan sonra masaüstü ve Başlat menüsü kısayollarının simgesi |
| `flutter/assets/logo.png` (yeni, en fazla 300x60) | Ana ekranın üstündeki logo. Dosya yoksa logo gösterilmez. |
