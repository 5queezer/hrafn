<p align="center">
  <img src="https://raw.githubusercontent.com/5queezer/hrafn/master/docs/assets/hrafn-banner.png" alt="Hrafn" width="600" />
</p>

<h1 align="center">🦀 Hrafn — Kişisel AI Asistanı</h1>

<p align="center">
  <strong>Sıfır ek yük. Sıfır uzlaşma. %100 Rust. %100 Agnostik.</strong><br>
  ⚡️ <strong>$10'lık donanımda <5MB RAM ile çalışır: OpenClaw'dan %99 daha az bellek ve Mac mini'den %98 daha ucuz!</strong>
</p>

<p align="center">
  <a href="https://github.com/5queezer/hrafn/actions/workflows/ci-run.yml"><img src="https://img.shields.io/github/actions/workflow/status/5queezer/hrafn/ci-run.yml?branch=master&label=build" alt="Build Status" /></a>
  <a href="LICENSE-APACHE"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg" alt="License: MIT OR Apache-2.0" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-edition%202024-orange?logo=rust" alt="Rust Edition 2024" /></a>
  <a href="https://github.com/5queezer/hrafn/releases/latest"><img src="https://img.shields.io/badge/version-v0.6.5-blue" alt="Version v0.6.5" /></a>
  <a href="https://github.com/5queezer/hrafn/graphs/contributors"><img src="https://img.shields.io/github/contributors/5queezer/hrafn?color=green" alt="Contributors" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>
  <a href="https://x.com/hrafn?s=21"><img src="https://img.shields.io/badge/X-%40hrafn-000000?style=flat&logo=x&logoColor=white" alt="X: @hrafn" /></a>
  <a href="https://discord.com/invite/wDshRVqRjx"><img src="https://img.shields.io/badge/Discord-Join-5865F2?style=flat&logo=discord&logoColor=white" alt="Discord" /></a>
  <a href="https://www.reddit.com/r/hrafn/"><img src="https://img.shields.io/badge/Reddit-r%2Fhrafn-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit: r/hrafn" /></a>
</p>

<p align="center">
Harvard, MIT ve Sundai.Club topluluklarının öğrencileri ve üyeleri tarafından geliştirilmiştir.
</p>

<p align="center">
  🌐 <strong>Diller:</strong>
  <a href="../../../README.md">🇺🇸 English</a> ·
  <a href="../zh-CN/README.md">🇨🇳 简体中文</a> ·
  <a href="../ja/README.md">🇯🇵 日本語</a> ·
  <a href="../ko/README.md">🇰🇷 한국어</a> ·
  <a href="../vi/README.md">🇻🇳 Tiếng Việt</a> ·
  <a href="../tl/README.md">🇵🇭 Tagalog</a> ·
  <a href="../es/README.md">🇪🇸 Español</a> ·
  <a href="../pt/README.md">🇧🇷 Português</a> ·
  <a href="../it/README.md">🇮🇹 Italiano</a> ·
  <a href="../de/README.md">🇩🇪 Deutsch</a> ·
  <a href="../fr/README.md">🇫🇷 Français</a> ·
  <a href="../ar/README.md">🇸🇦 العربية</a> ·
  <a href="../hi/README.md">🇮🇳 हिन्दी</a> ·
  <a href="../ru/README.md">🇷🇺 Русский</a> ·
  <a href="../bn/README.md">🇧🇩 বাংলা</a> ·
  <a href="../he/README.md">🇮🇱 עברית</a> ·
  <a href="../pl/README.md">🇵🇱 Polski</a> ·
  <a href="../cs/README.md">🇨🇿 Čeština</a> ·
  <a href="../nl/README.md">🇳🇱 Nederlands</a> ·
  <a href="../tr/README.md">🇹🇷 Türkçe</a> ·
  <a href="../uk/README.md">🇺🇦 Українська</a> ·
  <a href="../id/README.md">🇮🇩 Bahasa Indonesia</a> ·
  <a href="../th/README.md">🇹🇭 ไทย</a> ·
  <a href="../ur/README.md">🇵🇰 اردو</a> ·
  <a href="../ro/README.md">🇷🇴 Română</a> ·
  <a href="../sv/README.md">🇸🇪 Svenska</a> ·
  <a href="../el/README.md">🇬🇷 Ελληνικά</a> ·
  <a href="../hu/README.md">🇭🇺 Magyar</a> ·
  <a href="../fi/README.md">🇫🇮 Suomi</a> ·
  <a href="../da/README.md">🇩🇰 Dansk</a> ·
  <a href="../nb/README.md">🇳🇴 Norsk</a>
</p>

Hrafn, kendi cihazlarınızda çalıştırdığınız kişisel bir AI asistanıdır. Zaten kullandığınız kanallarda size yanıt verir (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work ve daha fazlası). Gerçek zamanlı kontrol için bir web paneli bulunur ve donanım çevre birimlerine bağlanabilir (ESP32, STM32, Arduino, Raspberry Pi). Gateway sadece kontrol düzlemidir — ürün asistanın kendisidir.

Yerel, hızlı ve her zaman açık hissettiren kişisel, tek kullanıcılı bir asistan istiyorsanız, işte bu.

<p align="center">
  <a href="https://hrafn.ai">Web sitesi</a> ·
  <a href="docs/README.md">Belgeler</a> ·
  <a href="docs/architecture.md">Mimari</a> ·
  <a href="#hızlı-başlangıç">Başlarken</a> ·
  <a href="#openclawdan-geçiş">OpenClaw'dan Geçiş</a> ·
  <a href="docs/ops/troubleshooting.md">Sorun Giderme</a> ·
  <a href="https://discord.com/invite/wDshRVqRjx">Discord</a>
</p>

> **Önerilen kurulum:** terminalinizde `hrafn onboard` komutunu çalıştırın. Hrafn Onboard, gateway, workspace, kanallar ve sağlayıcı kurulumunda sizi adım adım yönlendirir. Önerilen kurulum yoludur ve macOS, Linux ve Windows'ta (WSL2 ile) çalışır. Yeni kurulum mu? Buradan başlayın: [Başlarken](#hızlı-başlangıç)

### Abonelik Kimlik Doğrulama (OAuth)

- **OpenAI Codex** (ChatGPT aboneliği)
- **Gemini** (Google OAuth)
- **Anthropic** (API anahtarı veya yetkilendirme tokeni)

Model notu: birçok sağlayıcı/model desteklense de, en iyi deneyim için kullanabileceğiniz en güçlü son nesil modeli kullanın. Bkz. [Onboarding](#hızlı-başlangıç).

Model yapılandırması + CLI: [Sağlayıcı referansı](docs/reference/api/providers-reference.md)
Yetkilendirme profili rotasyonu (OAuth vs API anahtarları) + failover: [Model failover](docs/reference/api/providers-reference.md)

## Kurulum (önerilen)

Çalışma zamanı: Kararlı Rust toolchain. Tek ikili dosya, çalışma zamanı bağımlılığı yok.

### Homebrew (macOS/Linuxbrew)

```bash
brew install hrafn
```

### Tek tıkla kurulum

```bash
git clone https://github.com/5queezer/hrafn.git
cd hrafn
./install.sh
```

`hrafn onboard` kurulumdan sonra workspace ve sağlayıcınızı yapılandırmak için otomatik olarak çalışır.

## Hızlı başlangıç (TL;DR)

Tam başlangıç kılavuzu (kimlik doğrulama, eşleştirme, kanallar): [Başlarken](docs/setup-guides/one-click-bootstrap.md)

```bash
# Kurulum + onboarding
./install.sh --api-key "sk-..." --provider openrouter

# Gateway'i başlatın (webhook sunucusu + web paneli)
hrafn gateway                # varsayılan: 127.0.0.1:42617
hrafn gateway --port 0       # rastgele port (güvenlik güçlendirilmiş)

# Asistanla konuşun
hrafn agent -m "Hello, Hrafn!"

# Etkileşimli mod
hrafn agent

# Tam otonom çalışma zamanını başlatın (gateway + kanallar + cron + hands)
hrafn daemon

# Durumu kontrol edin
hrafn status

# Tanılama çalıştırın
hrafn doctor
```

Güncelleme mi yapıyorsunuz? Güncellemeden sonra `hrafn doctor` çalıştırın.

### Kaynaktan (geliştirme)

```bash
git clone https://github.com/5queezer/hrafn.git
cd hrafn

cargo build --release --locked
cargo install --path . --force --locked

hrafn onboard
```

> **Geliştirici fallback (global kurulum yok):** komutların başına `cargo run --release --` ekleyin (örnek: `cargo run --release -- status`).

## OpenClaw'dan Geçiş

Hrafn, OpenClaw workspace'inizi, belleğinizi ve yapılandırmanızı içe aktarabilir:

```bash
# Nelerin taşınacağını önizleyin (güvenli, salt okunur)
hrafn migrate openclaw --dry-run

# Geçişi çalıştırın
hrafn migrate openclaw
```

Bu, bellek girişlerinizi, workspace dosyalarınızı ve yapılandırmanızı `~/.openclaw/` dizininden `~/.hrafn/` dizinine taşır. Yapılandırma otomatik olarak JSON'dan TOML'a dönüştürülür.

## Güvenlik varsayılanları (DM erişimi)

Hrafn gerçek mesajlaşma platformlarına bağlanır. Gelen DM'leri güvenilmeyen girdi olarak değerlendirin.

Tam güvenlik kılavuzu: [SECURITY.md](SECURITY.md)

Tüm kanallarda varsayılan davranış:

- **DM eşleştirme** (varsayılan): bilinmeyen gönderenler kısa bir eşleştirme kodu alır ve bot mesajlarını işlemez.
- Şununla onaylayın: `hrafn pairing approve <channel> <code>` (ardından gönderen yerel izin listesine eklenir).
- Genel gelen DM'ler, `config.toml`'da açık bir opt-in gerektirir.
- Riskli veya yanlış yapılandırılmış DM politikalarını tespit etmek için `hrafn doctor` çalıştırın.

**Otonomi seviyeleri:**

| Seviye | Davranış |
|--------|----------|
| `ReadOnly` | Ajan gözlemleyebilir ama harekete geçemez |
| `Supervised` (varsayılan) | Ajan, orta/yüksek riskli işlemler için onay ile hareket eder |
| `Full` | Ajan politika sınırları içinde otonom hareket eder |

**Sandboxing katmanları:** workspace izolasyonu, yol geçişi engelleme, komut izin listeleri, yasaklı yollar (`/etc`, `/root`, `~/.ssh`), hız sınırlama (maks eylem/saat, maliyet/gün sınırları).

<!-- BEGIN:WHATS_NEW -->
<!-- END:WHATS_NEW -->

### 📢 Duyurular

Bu panoyu önemli bildirimler (breaking change'ler, güvenlik tavsiyeleri, bakım pencereleri ve sürüm engelleyicileri) için kullanın.

| Tarih (UTC) | Seviye       | Bildirim                                                                                                                                                                                                                                                                                                                                                 | Eylem                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-02-19 | _Kritik_  | `openagen/hrafn`, `hrafn.org` veya `hrafn.net` ile **bağlantılı değiliz**. `hrafn.org` ve `hrafn.net` alan adları şu anda `openagen/hrafn` fork'una yönlendirmektedir ve bu alan adı/depo, resmi web sitemizi/projemizi taklit etmektedir.                                                                                       | Bu kaynaklardan gelen bilgilere, ikili dosyalara, bağış toplama faaliyetlerine veya duyurulara güvenmeyin. Yalnızca [bu depoyu](https://github.com/5queezer/hrafn) ve doğrulanmış sosyal hesaplarımızı kullanın.                                                                                                                                                                                                                                                                                                                                                                       |
| 2026-02-19 | _Önemli_ | Anthropic, Kimlik Doğrulama ve Kimlik Bilgisi Kullanımı koşullarını 2026-02-19'da güncelledi. Claude Code OAuth token'ları (Free, Pro, Max) yalnızca Claude Code ve Claude.ai için tasarlanmıştır; Claude Free/Pro/Max'tan OAuth token'larını başka herhangi bir üründe, araçta veya hizmette (Agent SDK dahil) kullanmak izin verilmez ve Tüketici Hizmet Koşullarını ihlal edebilir. | Olası kayıpları önlemek için lütfen Claude Code OAuth entegrasyonlarından geçici olarak kaçının. Orijinal madde: [Authentication and Credential Use](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use).                                                                                                                                                                                                                                                                                                                                                    |

## Öne Çıkanlar

- **Varsayılan olarak hafif çalışma zamanı** — yaygın CLI ve durum iş akışları, release derlemelerinde birkaç megabaytlık bellek zarfında çalışır.
- **Maliyet etkin dağıtım** — $10'lık kartlar ve küçük bulut örnekleri için tasarlanmış, ağır çalışma zamanı bağımlılığı yok.
- **Hızlı soğuk başlatmalar** — tek ikili Rust çalışma zamanı, komut ve daemon başlatmayı neredeyse anlık tutar.
- **Taşınabilir mimari** — ARM, x86 ve RISC-V'de değiştirilebilir sağlayıcılar/kanallar/araçlarla tek ikili dosya.
- **Yerel gateway** — oturumlar, kanallar, araçlar, cron, SOP'lar ve olaylar için tek kontrol düzlemi.
- **Çok kanallı gelen kutusu** — WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WebSocket ve daha fazlası.
- **Çok ajanlı orkestrasyon (Hands)** — zamanlanmış çalışan ve zamanla daha akıllı hale gelen otonom ajan kümeleri.
- **Standart İşletim Prosedürleri (SOP'lar)** — MQTT, webhook, cron ve çevre birimi tetikleyicileriyle olay odaklı iş akışı otomasyonu.
- **Web paneli** — gerçek zamanlı sohbet, bellek tarayıcısı, yapılandırma düzenleyicisi, cron yöneticisi ve araç denetçisi ile React 19 + Vite web arayüzü.
- **Donanım çevre birimleri** — `Peripheral` trait'i üzerinden ESP32, STM32 Nucleo, Arduino, Raspberry Pi GPIO.
- **Birinci sınıf araçlar** — shell, dosya G/Ç, tarayıcı, git, web fetch/search, MCP, Jira, Notion, Google Workspace ve 70+ daha fazlası.
- **Yaşam döngüsü hook'ları** — her aşamada LLM çağrılarını, araç yürütmelerini ve mesajları yakalayın ve değiştirin.
- **Yetenek platformu** — güvenlik denetimi ile yerleşik, topluluk ve workspace yetenekleri.
- **Tünel desteği** — uzaktan erişim için Cloudflare, Tailscale, ngrok, OpenVPN ve özel tüneller.

### Ekipler neden Hrafn'u tercih ediyor

- **Varsayılan olarak hafif:** küçük Rust ikili dosyası, hızlı başlatma, düşük bellek ayak izi.
- **Tasarımdan güvenli:** eşleştirme, sıkı sandboxing, açık izin listeleri, workspace kapsamlandırma.
- **Tamamen değiştirilebilir:** temel sistemler trait'lerdir (sağlayıcılar, kanallar, araçlar, bellek, tüneller).
- **Satıcı bağımlılığı yok:** OpenAI uyumlu sağlayıcı desteği + takılabilir özel endpoint'ler.

## Benchmark Özeti (Hrafn vs OpenClaw, Tekrarlanabilir)

Yerel makine hızlı benchmark'ı (macOS arm64, Şubat 2026) 0.8GHz edge donanımı için normalleştirilmiş.

|                           | OpenClaw      | NanoBot        | PicoClaw        | Hrafn 🦀          |
| ------------------------- | ------------- | -------------- | --------------- | -------------------- |
| **Dil**                   | TypeScript    | Python         | Go              | **Rust**             |
| **RAM**                   | > 1GB         | > 100MB        | < 10MB          | **< 5MB**            |
| **Başlatma (0.8GHz çekirdek)** | > 500s   | > 30s          | < 1s            | **< 10ms**           |
| **İkili Boyut**           | ~28MB (dist)  | N/A (Script'ler) | ~8MB          | **~8.8 MB**          |
| **Maliyet**               | Mac Mini $599 | Linux SBC ~$50 | Linux Board $10 | **Herhangi bir donanım $10** |

> Notlar: Hrafn sonuçları, `/usr/bin/time -l` kullanılarak release derlemelerinde ölçülmüştür. OpenClaw, Node.js çalışma zamanı gerektirir (tipik olarak ~390MB ek bellek yükü), NanoBot ise Python çalışma zamanı gerektirir. PicoClaw ve Hrafn statik ikili dosyalardır. Yukarıdaki RAM rakamları çalışma zamanı belleğidir; derleme gereksinimleri daha yüksektir.

<p align="center">
  <img src="docs/assets/hrafn-comparison.jpeg" alt="Hrafn vs OpenClaw Comparison" width="800" />
</p>

### Tekrarlanabilir yerel ölçüm

```bash
cargo build --release
ls -lh target/release/hrafn

/usr/bin/time -l target/release/hrafn --help
/usr/bin/time -l target/release/hrafn status
```

## Şimdiye kadar inşa ettiğimiz her şey

### Çekirdek platform

- Gateway HTTP/WS/SSE kontrol düzlemi: oturumlar, varlık, yapılandırma, cron, webhook'lar, web paneli ve eşleştirme.
- CLI yüzeyi: `gateway`, `agent`, `onboard`, `doctor`, `status`, `service`, `migrate`, `auth`, `cron`, `channel`, `skills`.
- Araç dispatch'i, prompt oluşturma, mesaj sınıflandırma ve bellek yükleme ile ajan orkestrasyon döngüsü.
- Güvenlik politikası uygulama, otonomi seviyeleri ve onay kapılamayla oturum modeli.
- 20+ LLM backend'inde failover, yeniden deneme ve model yönlendirme ile dayanıklı sağlayıcı wrapper'ı.

### Kanallar

Kanallar: WhatsApp (yerel), Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, DingTalk, Lark, Mattermost, Nextcloud Talk, Nostr, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WATI, Mochat, Linq, Notion, WebSocket, ClawdTalk.

Feature-gated: Matrix (`channel-matrix`), Lark (`channel-lark`), Nostr (`channel-nostr`).

### Web paneli

Gateway'den doğrudan sunulan React 19 + Vite 6 + Tailwind CSS 4 web paneli:

- **Dashboard** — sistem genel görünümü, sağlık durumu, çalışma süresi, maliyet takibi
- **Ajan Sohbeti** — ajanla etkileşimli sohbet
- **Bellek** — bellek girişlerini gözatma ve yönetme
- **Yapılandırma** — yapılandırmayı görüntüleme ve düzenleme
- **Cron** — zamanlanmış görevleri yönetme
- **Araçlar** — kullanılabilir araçları gözatma
- **Günlükler** — ajan etkinlik günlüklerini görüntüleme
- **Maliyet** — token kullanımı ve maliyet takibi
- **Doctor** — sistem sağlık tanılaması
- **Entegrasyonlar** — entegrasyon durumu ve kurulumu
- **Eşleştirme** — cihaz eşleştirme yönetimi

### Firmware hedefleri

| Hedef | Platform | Amaç |
|-------|----------|------|
| ESP32 | Espressif ESP32 | Kablosuz çevresel ajan |
| ESP32-UI | ESP32 + Ekran | Görsel arayüzlü ajan |
| STM32 Nucleo | STM32 (ARM Cortex-M) | Endüstriyel çevre birimi |
| Arduino | Arduino | Temel sensör/aktüatör köprüsü |
| Uno Q Bridge | Arduino Uno | Ajana seri köprü |

### Araçlar + otomasyon

- **Çekirdek:** shell, dosya okuma/yazma/düzenleme, git işlemleri, glob arama, içerik arama
- **Web:** tarayıcı kontrolü, web fetch, web arama, ekran görüntüsü, görüntü bilgisi, PDF okuma
- **Entegrasyonlar:** Jira, Notion, Google Workspace, Microsoft 365, LinkedIn, Composio, Pushover
- **MCP:** Model Context Protocol araç wrapper'ı + ertelenmiş araç setleri
- **Zamanlama:** cron add/remove/update/run, zamanlama aracı
- **Bellek:** recall, store, forget, knowledge, project intel
- **Gelişmiş:** delegate (ajan-ajana), swarm, model switch/routing, security ops, cloud ops
- **Donanım:** board info, memory map, memory read (feature-gated)

### Çalışma zamanı + güvenlik

- **Otonomi seviyeleri:** ReadOnly, Supervised (varsayılan), Full.
- **Sandboxing:** workspace izolasyonu, yol geçişi engelleme, komut izin listeleri, yasaklı yollar, Landlock (Linux), Bubblewrap.
- **Hız sınırlama:** saat başı maks eylem, gün başı maks maliyet (yapılandırılabilir).
- **Onay kapılama:** orta/yüksek riskli işlemler için etkileşimli onay.
- **E-stop:** acil durum kapatma yeteneği.
- **129+ güvenlik testi** otomatik CI'da.

### İşletim + paketleme

- Web paneli doğrudan Gateway'den sunulur.
- Tünel desteği: Cloudflare, Tailscale, ngrok, OpenVPN, özel komut.
- Konteynerleştirilmiş yürütme için Docker çalışma zamanı adaptörü.
- CI/CD: beta (push'ta otomatik) → stable (manuel dispatch) → Docker, crates.io, Scoop, AUR, Homebrew, tweet.
- Linux (x86_64, aarch64, armv7), macOS (x86_64, aarch64), Windows (x86_64) için önceden derlenmiş ikili dosyalar.


## Yapılandırma

Minimal `~/.hrafn/config.toml`:

```toml
default_provider = "anthropic"
api_key = "sk-ant-..."
```

Tam yapılandırma referansı: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md).

### Kanal yapılandırması

**Telegram:**
```toml
[channels.telegram]
bot_token = "123456:ABC-DEF..."
```

**Discord:**
```toml
[channels.discord]
token = "your-bot-token"
```

**Slack:**
```toml
[channels.slack]
bot_token = "xoxb-..."
app_token = "xapp-..."
```

**WhatsApp:**
```toml
[channels.whatsapp]
enabled = true
```

**Matrix:**
```toml
[channels.matrix]
homeserver_url = "https://matrix.org"
username = "@bot:matrix.org"
password = "..."
```

**Signal:**
```toml
[channels.signal]
phone_number = "+1234567890"
```

### Tünel yapılandırması

```toml
[tunnel]
kind = "cloudflare"  # veya "tailscale", "ngrok", "openvpn", "custom", "none"
```

Ayrıntılar: [Kanal referansı](docs/reference/api/channels-reference.md) · [Yapılandırma referansı](docs/reference/api/config-reference.md)

### Çalışma zamanı desteği (mevcut)

- **`native`** (varsayılan) — doğrudan süreç yürütme, en hızlı yol, güvenilir ortamlar için ideal.
- **`docker`** — tam konteyner izolasyonu, zorunlu güvenlik politikaları, Docker gerektirir.

Sıkı sandboxing veya ağ izolasyonu için `runtime.kind = "docker"` ayarlayın.

## Abonelik Kimlik Doğrulama (OpenAI Codex / Claude Code / Gemini)

Hrafn, yerel abonelik yetkilendirme profillerini destekler (çoklu hesap, durağan halde şifreli).

- Depolama dosyası: `~/.hrafn/auth-profiles.json`
- Şifreleme anahtarı: `~/.hrafn/.secret_key`
- Profil ID formatı: `<provider>:<profile_name>` (örnek: `openai-codex:work`)

```bash
# OpenAI Codex OAuth (ChatGPT aboneliği)
hrafn auth login --provider openai-codex --device-code

# Gemini OAuth
hrafn auth login --provider gemini --profile default

# Anthropic setup-token
hrafn auth paste-token --provider anthropic --profile default --auth-kind authorization

# Kontrol / yenileme / profil değiştirme
hrafn auth status
hrafn auth refresh --provider openai-codex --profile default
hrafn auth use --provider openai-codex --profile work

# Ajanı abonelik auth ile çalıştırma
hrafn agent --provider openai-codex -m "hello"
hrafn agent --provider anthropic -m "hello"
```

## Ajan workspace + yetenekler

Workspace kök dizini: `~/.hrafn/workspace/` (config ile yapılandırılabilir).

Enjekte edilen prompt dosyaları:
- `IDENTITY.md` — ajan kişiliği ve rolü
- `USER.md` — kullanıcı bağlamı ve tercihleri
- `MEMORY.md` — uzun vadeli gerçekler ve dersler
- `AGENTS.md` — oturum kuralları ve başlatma kuralları
- `SOUL.md` — temel kimlik ve çalışma prensipleri

Yetenekler: `~/.hrafn/workspace/skills/<skill>/SKILL.md` veya `SKILL.toml`.

```bash
# Yüklü yetenekleri listele
hrafn skills list

# Git'ten yükle
hrafn skills install https://github.com/user/my-skill.git

# Yüklemeden önce güvenlik denetimi
hrafn skills audit https://github.com/user/my-skill.git

# Bir yeteneği kaldır
hrafn skills remove my-skill
```

## CLI komutları

```bash
# Workspace yönetimi
hrafn onboard              # Rehberli kurulum sihirbazı
hrafn status               # Daemon/ajan durumunu göster
hrafn doctor               # Sistem tanılaması çalıştır

# Gateway + daemon
hrafn gateway              # Gateway sunucusunu başlat (127.0.0.1:42617)
hrafn daemon               # Tam otonom çalışma zamanını başlat

# Ajan
hrafn agent                # Etkileşimli sohbet modu
hrafn agent -m "message"   # Tek mesaj modu

# Hizmet yönetimi
hrafn service install      # OS hizmeti olarak yükle (launchd/systemd)
hrafn service start|stop|restart|status

# Kanallar
hrafn channel list         # Yapılandırılmış kanalları listele
hrafn channel doctor       # Kanal sağlığını kontrol et
hrafn channel bind-telegram 123456789

# Cron + zamanlama
hrafn cron list            # Zamanlanmış görevleri listele
hrafn cron add "*/5 * * * *" --prompt "Check system health"
hrafn cron remove <id>

# Bellek
hrafn memory list          # Bellek girişlerini listele
hrafn memory get <key>     # Bir bellek al
hrafn memory stats         # Bellek istatistikleri

# Yetkilendirme profilleri
hrafn auth login --provider <name>
hrafn auth status
hrafn auth use --provider <name> --profile <profile>

# Donanım çevre birimleri
hrafn hardware discover    # Bağlı cihazları tara
hrafn peripheral list      # Bağlı çevre birimlerini listele
hrafn peripheral flash     # Cihaza firmware yükle

# Geçiş
hrafn migrate openclaw --dry-run
hrafn migrate openclaw

# Kabuk tamamlama
source <(hrafn completions bash)
hrafn completions zsh > ~/.zfunc/_hrafn
```

Tam komut referansı: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)

<!-- markdownlint-disable MD001 MD024 -->

## Ön koşullar

<details>
<summary><strong>Windows</strong></summary>

#### Gerekli

1. **Visual Studio Build Tools** (MSVC linker ve Windows SDK sağlar):

    ```powershell
    winget install Microsoft.VisualStudio.2022.BuildTools
    ```

    Kurulum sırasında (veya Visual Studio Installer aracılığıyla) **"Desktop development with C++"** workload'unu seçin.

2. **Rust toolchain:**

    ```powershell
    winget install Rustlang.Rustup
    ```

    Kurulumdan sonra yeni bir terminal açın ve kararlı toolchain'in aktif olduğundan emin olmak için `rustup default stable` çalıştırın.

3. Her ikisinin de çalıştığını **doğrulayın**:
    ```powershell
    rustc --version
    cargo --version
    ```

#### İsteğe bağlı

- **Docker Desktop** — yalnızca [Docker sandbox'lu çalışma zamanı](#çalışma-zamanı-desteği-mevcut) (`runtime.kind = "docker"`) kullanıyorsanız gereklidir. `winget install Docker.DockerDesktop` ile yükleyin.

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### Gerekli

1. **Derleme araçları:**
    - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
    - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
    - **macOS:** Xcode Command Line Tools yükleyin: `xcode-select --install`

2. **Rust toolchain:**

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Ayrıntılar için [rustup.rs](https://rustup.rs) sayfasına bakın.

3. Her ikisinin de çalıştığını **doğrulayın**:
    ```bash
    rustc --version
    cargo --version
    ```

#### Tek satır yükleyici

Veya yukarıdaki adımları atlayın ve her şeyi (sistem bağımlılıkları, Rust, Hrafn) tek komutla yükleyin:

```bash
curl -LsSf https://raw.githubusercontent.com/5queezer/hrafn/master/install.sh | bash
```

#### Derleme kaynak gereksinimleri

Kaynaktan derleme, ortaya çıkan ikili dosyayı çalıştırmaktan daha fazla kaynak gerektirir:

| Kaynak         | Minimum | Önerilen    |
| -------------- | ------- | ----------- |
| **RAM + swap** | 2 GB    | 4 GB+       |
| **Boş disk**   | 6 GB    | 10 GB+      |

Host'unuz minimumun altındaysa, önceden derlenmiş ikili dosyaları kullanın:

```bash
./install.sh --prefer-prebuilt
```

Kaynak fallback'ı olmadan yalnızca ikili kurulum zorlamak için:

```bash
./install.sh --prebuilt-only
```

#### İsteğe bağlı

- **Docker** — yalnızca [Docker sandbox'lu çalışma zamanı](#çalışma-zamanı-desteği-mevcut) (`runtime.kind = "docker"`) kullanıyorsanız gereklidir. Paket yöneticiniz veya [docker.com](https://docs.docker.com/engine/install/) aracılığıyla yükleyin.

> **Not:** Varsayılan `cargo build --release`, derleme baskısını düşürmek için `codegen-units=1` kullanır. Güçlü makinelerde daha hızlı derlemeler için `cargo build --profile release-fast` kullanın.

</details>

<!-- markdownlint-enable MD001 MD024 -->

### Önceden derlenmiş ikili dosyalar

Sürüm varlıkları şunlar için yayınlanır:

- Linux: `x86_64`, `aarch64`, `armv7`
- macOS: `x86_64`, `aarch64`
- Windows: `x86_64`

En son varlıkları şuradan indirin:
<https://github.com/5queezer/hrafn/releases/latest>

## Belgeler

Onboarding akışını geçtikten sonra daha derin referans istediğinizde bunları kullanın.

- Navigasyon ve "ne nerede" için [belge dizini](docs/README.md) ile başlayın.
- Tam sistem modeli için [mimari genel bakış](docs/architecture.md) okuyun.
- Her anahtar ve örneğe ihtiyacınız olduğunda [yapılandırma referansı](docs/reference/api/config-reference.md) kullanın.
- [İşletim el kitabı](docs/ops/operations-runbook.md) ile Gateway'i kitabına göre çalıştırın.
- Rehberli kurulum için [Hrafn Onboard](#hızlı-başlangıç) takip edin.
- Yaygın hataları [sorun giderme kılavuzu](docs/ops/troubleshooting.md) ile ayıklayın.
- Herhangi bir şeyi açığa çıkarmadan önce [güvenlik rehberliği](docs/security/README.md) gözden geçirin.

### Referans belgeleri

- Belge merkezi: [docs/README.md](docs/README.md)
- Birleşik içindekiler: [docs/SUMMARY.md](docs/SUMMARY.md)
- Komut referansı: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)
- Yapılandırma referansı: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md)
- Sağlayıcı referansı: [docs/reference/api/providers-reference.md](docs/reference/api/providers-reference.md)
- Kanal referansı: [docs/reference/api/channels-reference.md](docs/reference/api/channels-reference.md)
- İşletim el kitabı: [docs/ops/operations-runbook.md](docs/ops/operations-runbook.md)
- Sorun giderme: [docs/ops/troubleshooting.md](docs/ops/troubleshooting.md)

### İşbirliği belgeleri

- Katkıda bulunma rehberi: [CONTRIBUTING.md](CONTRIBUTING.md)
- PR iş akışı politikası: [docs/contributing/pr-workflow.md](docs/contributing/pr-workflow.md)
- CI iş akışı rehberi: [docs/contributing/ci-map.md](docs/contributing/ci-map.md)
- İncelemeci el kitabı: [docs/contributing/reviewer-playbook.md](docs/contributing/reviewer-playbook.md)
- Güvenlik açıklama politikası: [SECURITY.md](SECURITY.md)
- Belge şablonu: [docs/contributing/doc-template.md](docs/contributing/doc-template.md)

### Dağıtım + işletim

- Ağ dağıtım rehberi: [docs/ops/network-deployment.md](docs/ops/network-deployment.md)
- Proxy ajan el kitabı: [docs/ops/proxy-agent-playbook.md](docs/ops/proxy-agent-playbook.md)
- Donanım rehberleri: [docs/hardware/README.md](docs/hardware/README.md)

## Smooth Crab 🦀

Hrafn, smooth crab 🦀 için inşa edildi — hızlı ve verimli bir AI asistanı. Argenis De La Rosa ve topluluk tarafından geliştirildi.

- [hrafn.ai](https://hrafn.ai)
- [@hrafn](https://x.com/hrafn)

## Hrafn'u Destekleyin

Hrafn işinize yarıyorsa ve süregelen geliştirmeyi desteklemek istiyorsanız, buradan bağış yapabilirsiniz:

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 Özel Teşekkürler

Bu açık kaynak çalışmaya ilham veren ve yakıt sağlayan topluluklara ve kurumlara içten bir teşekkür:

- **Harvard University** — entelektüel merakı beslemek ve mümkün olanın sınırlarını zorlamak için.
- **MIT** — açık bilgiyi, açık kaynağı ve teknolojinin herkes için erişilebilir olması gerektiği inancını savunmak için.
- **Sundai Club** — topluluk, enerji ve önemli şeyler inşa etmeye yönelik amansız istek için.
- **Dünya ve Ötesi** 🌍✨ — açık kaynağı iyilik için bir güç yapan her katkıda bulunan, hayalci ve inşaatçıya. Bu sizin için.

En iyi fikirler her yerden geldiği için açıkta inşa ediyoruz. Bunu okuyorsanız, bunun bir parçasısınız. Hoş geldiniz. 🦀❤️

## Katkıda Bulunma

Hrafn'da yeni misiniz? [`good first issue`](https://github.com/5queezer/hrafn/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) etiketli issue'ları arayın — nasıl başlayacağınızı öğrenmek için [Katkıda Bulunma Rehberi](CONTRIBUTING.md#first-time-contributors)mize bakın. AI/vibe-coded PR'lar hoş geldiniz! 🤖

[CONTRIBUTING.md](CONTRIBUTING.md) ve [CLA.md](docs/contributing/cla.md)'ye bakın. Bir trait uygulayın, PR gönderin:

- CI iş akışı rehberi: [docs/contributing/ci-map.md](docs/contributing/ci-map.md)
- Yeni `Provider` → `src/providers/`
- Yeni `Channel` → `src/channels/`
- Yeni `Observer` → `src/observability/`
- Yeni `Tool` → `src/tools/`
- Yeni `Memory` → `src/memory/`
- Yeni `Tunnel` → `src/tunnel/`
- Yeni `Peripheral` → `src/peripherals/`
- Yeni `Skill` → `~/.hrafn/workspace/skills/<name>/`

<!-- BEGIN:RECENT_CONTRIBUTORS -->
<!-- END:RECENT_CONTRIBUTORS -->

## ⚠️ Resmi Depo ve Kimlik Taklidi Uyarısı

**Bu, tek resmi Hrafn deposudur:**

> https://github.com/5queezer/hrafn

"Hrafn" olduğunu iddia eden veya Hrafn Labs ile bağlantı ima eden başka herhangi bir depo, organizasyon, alan adı veya paket **yetkisiz olup bu projeyle bağlantılı değildir**. Bilinen yetkisiz fork'lar [TRADEMARK.md](docs/maintainers/trademark.md)'de listelenecektir.

Kimlik taklidi veya ticari marka kötüye kullanımıyla karşılaşırsanız, lütfen [bir issue açın](https://github.com/5queezer/hrafn/issues).

---

## Lisans

Hrafn, maksimum açıklık ve katkıda bulunan koruması için çift lisanslıdır:

| Lisans | Kullanım senaryosu |
|--------|-------------------|
| [MIT](LICENSE-MIT) | Açık kaynak, araştırma, akademik, kişisel kullanım |
| [Apache 2.0](LICENSE-APACHE) | Patent koruması, kurumsal, ticari dağıtım |

Her iki lisanstan birini seçebilirsiniz. **Katkıda bulunanlar her ikisi altında otomatik olarak hak verir** — tam katkıda bulunan sözleşmesi için [CLA.md](docs/contributing/cla.md)'ye bakın.

### Ticari Marka

**Hrafn** adı ve logosu, Hrafn Labs'ın ticari markalarıdır. Bu lisans, onay veya bağlantı ima etmek için bunları kullanma izni vermez. İzin verilen ve yasaklanan kullanımlar için [TRADEMARK.md](docs/maintainers/trademark.md)'ye bakın.

### Katkıda Bulunan Korumaları

- Katkılarınızın **telif hakkını elinizde tutarsınız**
- **Patent hakkı** (Apache 2.0) sizi diğer katkıda bulunanların patent taleplerinden korur
- Katkılarınız commit geçmişinde ve [NOTICE](NOTICE)'da **kalıcı olarak atfedilir**
- Katkıda bulunarak hiçbir ticari marka hakkı devredilmez

---

**Hrafn** — Sıfır ek yük. Sıfır uzlaşma. Her yere dağıtın. Her şeyi değiştirin. 🦀

## Katkıda Bulunanlar

<a href="https://github.com/5queezer/hrafn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=5queezer/hrafn" alt="Hrafn contributors" />
</a>

Bu liste GitHub katkıda bulunanlar grafiğinden oluşturulur ve otomatik olarak güncellenir.

## Yıldız Geçmişi

<p align="center">
  <a href="https://www.star-history.com/#5queezer/hrafn&type=date&legend=top-left">
    <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&theme=dark&legend=top-left" />
     <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&legend=top-left" />
     <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&legend=top-left" />
    </picture>
  </a>
</p>
