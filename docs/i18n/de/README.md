<p align="center">
  <img src="https://raw.githubusercontent.com/5queezer/hrafn/master/docs/assets/hrafn-banner.png" alt="Hrafn" width="600" />
</p>

<h1 align="center">🦀 Hrafn — Persönlicher KI-Assistent</h1>

<p align="center">
  <strong>Null Overhead. Null Kompromisse. 100% Rust. 100% Agnostisch.</strong><br>
  ⚡️ <strong>Läuft auf $10-Hardware mit <5MB RAM: 99% weniger Speicher als OpenClaw und 98% günstiger als ein Mac mini!</strong>
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
Entwickelt von Studenten und Mitgliedern der Communitys von Harvard, MIT und Sundai.Club.
</p>

<p align="center">
  🌐 <strong>Sprachen:</strong>
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

Hrafn ist ein persönlicher KI-Assistent, den du auf deinen eigenen Geräten ausführst. Er antwortet dir auf den Kanälen, die du bereits nutzt (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work und mehr). Er verfügt über ein Web-Dashboard für Echtzeitkontrolle und kann sich mit Hardware-Peripheriegeräten verbinden (ESP32, STM32, Arduino, Raspberry Pi). Das Gateway ist nur die Steuerungsebene — das Produkt ist der Assistent.

Wenn du einen persönlichen Einzelbenutzer-Assistenten willst, der sich lokal, schnell und immer verfügbar anfühlt, ist das genau das Richtige.

<p align="center">
  <a href="https://hrafn.ai">Website</a> ·
  <a href="docs/README.md">Dokumentation</a> ·
  <a href="docs/architecture.md">Architektur</a> ·
  <a href="#schnellstart">Erste Schritte</a> ·
  <a href="#migration-von-openclaw">Migration von OpenClaw</a> ·
  <a href="docs/ops/troubleshooting.md">Fehlerbehebung</a> ·
  <a href="https://discord.com/invite/wDshRVqRjx">Discord</a>
</p>

> **Empfohlene Einrichtung:** Führe `hrafn onboard` in deinem Terminal aus. Hrafn Onboard führt dich Schritt für Schritt durch die Einrichtung von Gateway, Workspace, Kanälen und Provider. Es ist der empfohlene Einrichtungspfad und funktioniert auf macOS, Linux und Windows (über WSL2). Neue Installation? Starte hier: [Erste Schritte](#schnellstart)

### Abonnement-Authentifizierung (OAuth)

- **OpenAI Codex** (ChatGPT-Abonnement)
- **Gemini** (Google OAuth)
- **Anthropic** (API-Schlüssel oder Auth-Token)

Modellhinweis: Obwohl viele Provider/Modelle unterstützt werden, verwende für die beste Erfahrung das stärkste verfügbare Modell der neuesten Generation. Siehe [Onboarding](#schnellstart).

Modellkonfiguration + CLI: [Provider-Referenz](docs/reference/api/providers-reference.md)
Auth-Profilrotation (OAuth vs API-Schlüssel) + Failover: [Modell-Failover](docs/reference/api/providers-reference.md)

## Installation (empfohlen)

Voraussetzung: Stabile Rust-Toolchain. Einzelnes Binary, keine Laufzeitabhängigkeiten.

### Homebrew (macOS/Linuxbrew)

```bash
brew install hrafn
```

### Ein-Klick-Bootstrap

```bash
git clone https://github.com/5queezer/hrafn.git
cd hrafn
./install.sh
```

`hrafn onboard` wird nach der Installation automatisch ausgeführt, um deinen Workspace und Provider zu konfigurieren.

## Schnellstart (TL;DR)

Vollständige Einsteiger-Anleitung (Authentifizierung, Pairing, Kanäle): [Erste Schritte](docs/setup-guides/one-click-bootstrap.md)

```bash
# Installieren + Onboard
./install.sh --api-key "sk-..." --provider openrouter

# Gateway starten (Webhook-Server + Web-Dashboard)
hrafn gateway                # Standard: 127.0.0.1:42617
hrafn gateway --port 0       # Zufälliger Port (gehärtete Sicherheit)

# Mit dem Assistenten sprechen
hrafn agent -m "Hello, Hrafn!"

# Interaktiver Modus
hrafn agent

# Vollständige autonome Laufzeit starten (Gateway + Kanäle + Cron + Hands)
hrafn daemon

# Status prüfen
hrafn status

# Diagnose ausführen
hrafn doctor
```

Aktualisierung? Führe `hrafn doctor` nach dem Update aus.

### Aus dem Quellcode (Entwicklung)

```bash
git clone https://github.com/5queezer/hrafn.git
cd hrafn

cargo build --release --locked
cargo install --path . --force --locked

hrafn onboard
```

> **Entwicklungs-Fallback (ohne globale Installation):** Stelle Befehlen `cargo run --release --` voran (Beispiel: `cargo run --release -- status`).

## Migration von OpenClaw

Hrafn kann deinen OpenClaw-Workspace, Speicher und Konfiguration importieren:

```bash
# Vorschau, was migriert wird (sicher, nur lesen)
hrafn migrate openclaw --dry-run

# Migration ausführen
hrafn migrate openclaw
```

Dies migriert deine Speichereinträge, Workspace-Dateien und Konfiguration von `~/.openclaw/` nach `~/.hrafn/`. Die Konfiguration wird automatisch von JSON nach TOML konvertiert.

## Sicherheitsstandards (DM-Zugriff)

Hrafn verbindet sich mit echten Messaging-Oberflächen. Behandle eingehende DMs als nicht vertrauenswürdige Eingabe.

Vollständiger Sicherheitsleitfaden: [SECURITY.md](SECURITY.md)

Standardverhalten auf allen Kanälen:

- **DM-Pairing** (Standard): Unbekannte Absender erhalten einen kurzen Pairing-Code und der Bot verarbeitet ihre Nachricht nicht.
- Genehmige mit: `hrafn pairing approve <channel> <code>` (der Absender wird dann zu einer lokalen Allowlist hinzugefügt).
- Öffentliche eingehende DMs erfordern eine explizite Aktivierung in `config.toml`.
- Führe `hrafn doctor` aus, um riskante oder falsch konfigurierte DM-Richtlinien aufzudecken.

**Autonomiestufen:**

| Stufe | Verhalten |
|-------|-----------|
| `ReadOnly` | Der Agent kann beobachten, aber nicht handeln |
| `Supervised` (Standard) | Der Agent handelt mit Genehmigung für Operationen mit mittlerem/hohem Risiko |
| `Full` | Der Agent handelt autonom innerhalb der Richtliniengrenzen |

**Sandboxing-Schichten:** Workspace-Isolation, Pfad-Traversal-Blockierung, Befehls-Allowlisting, verbotene Pfade (`/etc`, `/root`, `~/.ssh`), Ratenbegrenzung (max. Aktionen/Stunde, Kosten/Tag-Obergrenzen).

<!-- BEGIN:WHATS_NEW -->
<!-- END:WHATS_NEW -->

### 📢 Ankündigungen

Verwende dieses Board für wichtige Hinweise (Breaking Changes, Sicherheitshinweise, Wartungsfenster und Release-Blocker).

| Datum (UTC) | Stufe       | Hinweis                                                                                                                                                                                                                                                                                                                                                 | Aktion                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| ---------- | ----------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 2026-02-19 | _Kritisch_  | Wir sind **nicht verbunden** mit `openagen/hrafn`, `hrafn.org` oder `hrafn.net`. Die Domains `hrafn.org` und `hrafn.net` verweisen derzeit auf den Fork `openagen/hrafn`, und diese Domain/dieses Repository geben sich als unsere offizielle Website/unser offizielles Projekt aus.                                                                                       | Vertraue keinen Informationen, Binaries, Spendenaktionen oder Ankündigungen aus diesen Quellen. Verwende nur [dieses Repository](https://github.com/5queezer/hrafn) und unsere verifizierten Social-Media-Konten.                                                                                                                                                                                                                                                                                                                                                                                                                       |
| 2026-02-19 | _Wichtig_ | Anthropic hat die Bedingungen zur Authentifizierung und Nutzung von Zugangsdaten am 2026-02-19 aktualisiert. Claude Code OAuth-Tokens (Free, Pro, Max) sind ausschließlich für Claude Code und Claude.ai bestimmt; die Verwendung von OAuth-Tokens von Claude Free/Pro/Max in anderen Produkten, Tools oder Diensten (einschließlich Agent SDK) ist nicht gestattet und kann gegen die Verbrauchernutzungsbedingungen verstoßen. | Bitte vermeide vorübergehend Claude Code OAuth-Integrationen, um potenzielle Verluste zu vermeiden. Originalklausel: [Authentication and Credential Use](https://code.claude.com/docs/en/legal-and-compliance#authentication-and-credential-use).                                                                                                                                                                                                                                                                                                                                                                                    |

## Highlights

- **Leichte Laufzeitumgebung standardmäßig** — gängige CLI- und Status-Workflows laufen in einem Speicherumfang von wenigen Megabyte bei Release-Builds.
- **Kosteneffiziente Bereitstellung** — entwickelt für $10-Boards und kleine Cloud-Instanzen, keine schwergewichtigen Laufzeitabhängigkeiten.
- **Schnelle Kaltstarts** — die Rust-Single-Binary-Laufzeit hält den Start von Befehlen und Daemon nahezu sofortig.
- **Portable Architektur** — ein Binary für ARM, x86 und RISC-V mit austauschbaren Providern/Kanälen/Tools.
- **Local-first Gateway** — einzelne Steuerungsebene für Sitzungen, Kanäle, Tools, Cron, SOPs und Events.
- **Multi-Kanal-Posteingang** — WhatsApp, Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, Nostr, Mattermost, Nextcloud Talk, DingTalk, Lark, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WebSocket und mehr.
- **Multi-Agenten-Orchestrierung (Hands)** — autonome Agentenschwärme, die nach Zeitplan laufen und mit der Zeit intelligenter werden.
- **Standardbetriebsverfahren (SOPs)** — ereignisgesteuerte Workflow-Automatisierung mit MQTT, Webhook, Cron und Peripherie-Triggern.
- **Web-Dashboard** — React 19 + Vite Web-UI mit Echtzeit-Chat, Speicher-Browser, Konfigurationseditor, Cron-Manager und Tool-Inspektor.
- **Hardware-Peripheriegeräte** — ESP32, STM32 Nucleo, Arduino, Raspberry Pi GPIO über den `Peripheral`-Trait.
- **Erstklassige Tools** — Shell, Datei-I/O, Browser, Git, Web Fetch/Search, MCP, Jira, Notion, Google Workspace und über 70 weitere.
- **Lifecycle-Hooks** — LLM-Aufrufe, Tool-Ausführungen und Nachrichten in jeder Phase abfangen und modifizieren.
- **Skills-Plattform** — mitgelieferte, Community- und Workspace-Skills mit Sicherheitsaudit.
- **Tunnel-Unterstützung** — Cloudflare, Tailscale, ngrok, OpenVPN und benutzerdefinierte Tunnel für Remote-Zugriff.

### Warum Teams Hrafn wählen

- **Standardmäßig leicht:** kleines Rust-Binary, schneller Start, geringer Speicherverbrauch.
- **Sicher by Design:** Pairing, striktes Sandboxing, explizite Allowlists, Workspace-Scoping.
- **Vollständig austauschbar:** Kernsysteme sind Traits (Provider, Kanäle, Tools, Speicher, Tunnel).
- **Kein Vendor Lock-in:** OpenAI-kompatible Provider-Unterstützung + steckbare benutzerdefinierte Endpunkte.

## Benchmark-Übersicht (Hrafn vs OpenClaw, reproduzierbar)

Schneller lokaler Benchmark (macOS arm64, Feb 2026), normalisiert für 0,8GHz Edge-Hardware.

|                           | OpenClaw      | NanoBot        | PicoClaw        | Hrafn 🦀          |
| ------------------------- | ------------- | -------------- | --------------- | -------------------- |
| **Sprache**               | TypeScript    | Python         | Go              | **Rust**             |
| **RAM**                   | > 1GB         | > 100MB        | < 10MB          | **< 5MB**            |
| **Start (0,8GHz Core)**  | > 500s        | > 30s          | < 1s            | **< 10ms**           |
| **Binary-Größe**          | ~28MB (dist)  | N/A (Scripts)  | ~8MB            | **~8.8 MB**          |
| **Kosten**                | Mac Mini $599 | Linux SBC ~$50 | Linux Board $10 | **Beliebige Hardware $10** |

> Hinweise: Hrafn-Ergebnisse werden bei Release-Builds mit `/usr/bin/time -l` gemessen. OpenClaw benötigt die Node.js-Laufzeit (typischerweise ~390MB zusätzlicher Speicherverbrauch), während NanoBot die Python-Laufzeit benötigt. PicoClaw und Hrafn sind statische Binaries. Die RAM-Zahlen oben sind Laufzeitspeicher; die Kompilierungsanforderungen sind höher.

<p align="center">
  <img src="docs/assets/hrafn-comparison.jpeg" alt="Hrafn vs OpenClaw Comparison" width="800" />
</p>

### Reproduzierbare lokale Messung

```bash
cargo build --release
ls -lh target/release/hrafn

/usr/bin/time -l target/release/hrafn --help
/usr/bin/time -l target/release/hrafn status
```

## Alles, was wir bisher gebaut haben

### Kernplattform

- Gateway HTTP/WS/SSE-Steuerungsebene mit Sitzungen, Präsenz, Konfiguration, Cron, Webhooks, Web-Dashboard und Pairing.
- CLI-Oberfläche: `gateway`, `agent`, `onboard`, `doctor`, `status`, `service`, `migrate`, `auth`, `cron`, `channel`, `skills`.
- Agenten-Orchestrierungsschleife mit Tool-Dispatch, Prompt-Konstruktion, Nachrichtenklassifizierung und Speicherladung.
- Sitzungsmodell mit Durchsetzung von Sicherheitsrichtlinien, Autonomiestufen und Genehmigungsgating.
- Resiliente Provider-Wrapper mit Failover, Retry und Modell-Routing über 20+ LLM-Backends.

### Kanäle

Kanäle: WhatsApp (nativ), Telegram, Slack, Discord, Signal, iMessage, Matrix, IRC, Email, Bluesky, DingTalk, Lark, Mattermost, Nextcloud Talk, Nostr, QQ, Reddit, LinkedIn, Twitter, MQTT, WeChat Work, WATI, Mochat, Linq, Notion, WebSocket, ClawdTalk.

Feature-gated: Matrix (`channel-matrix`), Lark (`channel-lark`), Nostr (`channel-nostr`).

### Web-Dashboard

React 19 + Vite 6 + Tailwind CSS 4 Web-Dashboard, direkt vom Gateway bereitgestellt:

- **Dashboard** — Systemübersicht, Gesundheitsstatus, Betriebszeit, Kostenverfolgung
- **Agenten-Chat** — interaktiver Chat mit dem Agenten
- **Speicher** — Speichereinträge durchsuchen und verwalten
- **Konfiguration** — Konfiguration anzeigen und bearbeiten
- **Cron** — geplante Aufgaben verwalten
- **Tools** — verfügbare Tools durchsuchen
- **Logs** — Aktivitätsprotokolle des Agenten anzeigen
- **Kosten** — Token-Nutzung und Kostenverfolgung
- **Doctor** — Systemdiagnose
- **Integrationen** — Integrationsstatus und Einrichtung
- **Pairing** — Gerätekopplung verwalten

### Firmware-Ziele

| Ziel | Plattform | Zweck |
|------|-----------|-------|
| ESP32 | Espressif ESP32 | Drahtloser Peripherie-Agent |
| ESP32-UI | ESP32 + Display | Agent mit visueller Oberfläche |
| STM32 Nucleo | STM32 (ARM Cortex-M) | Industrielle Peripherie |
| Arduino | Arduino | Grundlegende Sensor-/Aktor-Brücke |
| Uno Q Bridge | Arduino Uno | Serielle Brücke zum Agenten |

### Tools + Automatisierung

- **Core:** Shell, Datei lesen/schreiben/bearbeiten, Git-Operationen, Glob-Suche, Inhaltssuche
- **Web:** Browser-Steuerung, Web Fetch, Web Search, Screenshot, Bildinformation, PDF-Lesen
- **Integrationen:** Jira, Notion, Google Workspace, Microsoft 365, LinkedIn, Composio, Pushover
- **MCP:** Model Context Protocol Tool-Wrapper + verzögerte Tool-Sets
- **Planung:** cron add/remove/update/run, Planungstool
- **Speicher:** recall, store, forget, knowledge, project intel
- **Erweitert:** delegate (Agent-zu-Agent), swarm, Modellwechsel/-routing, Sicherheitsoperationen, Cloud-Operationen
- **Hardware:** board info, memory map, memory read (feature-gated)

### Laufzeit + Sicherheit

- **Autonomiestufen:** ReadOnly, Supervised (Standard), Full.
- **Sandboxing:** Workspace-Isolation, Pfad-Traversal-Blockierung, Befehls-Allowlists, verbotene Pfade, Landlock (Linux), Bubblewrap.
- **Ratenbegrenzung:** max. Aktionen pro Stunde, max. Kosten pro Tag (konfigurierbar).
- **Genehmigungsgating:** interaktive Genehmigung für Operationen mit mittlerem/hohem Risiko.
- **Notfall-Stopp:** Notabschaltungsfähigkeit.
- **129+ Sicherheitstests** in automatisiertem CI.

### Betrieb + Paketierung

- Web-Dashboard direkt vom Gateway bereitgestellt.
- Tunnel-Unterstützung: Cloudflare, Tailscale, ngrok, OpenVPN, benutzerdefinierter Befehl.
- Docker-Laufzeitadapter für containerisierte Ausführung.
- CI/CD: beta (automatisch bei Push) → stable (manueller Dispatch) → Docker, crates.io, Scoop, AUR, Homebrew, Tweet.
- Vorgefertigte Binaries für Linux (x86_64, aarch64, armv7), macOS (x86_64, aarch64), Windows (x86_64).


## Konfiguration

Minimale `~/.hrafn/config.toml`:

```toml
default_provider = "anthropic"
api_key = "sk-ant-..."
```

Vollständige Konfigurationsreferenz: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md).

### Kanalkonfiguration

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

### Tunnel-Konfiguration

```toml
[tunnel]
kind = "cloudflare"  # oder "tailscale", "ngrok", "openvpn", "custom", "none"
```

Details: [Kanal-Referenz](docs/reference/api/channels-reference.md) · [Konfigurationsreferenz](docs/reference/api/config-reference.md)

### Laufzeitunterstützung (aktuell)

- **`native`** (Standard) — direkte Prozessausführung, schnellster Pfad, ideal für vertrauenswürdige Umgebungen.
- **`docker`** — vollständige Container-Isolation, erzwungene Sicherheitsrichtlinien, erfordert Docker.

Setze `runtime.kind = "docker"` für striktes Sandboxing oder Netzwerkisolation.

## Abonnement-Authentifizierung (OpenAI Codex / Claude Code / Gemini)

Hrafn unterstützt native Abonnement-Authentifizierungsprofile (Multi-Account, verschlüsselt im Ruhezustand).

- Speicherdatei: `~/.hrafn/auth-profiles.json`
- Verschlüsselungsschlüssel: `~/.hrafn/.secret_key`
- Profil-ID-Format: `<provider>:<profile_name>` (Beispiel: `openai-codex:work`)

```bash
# OpenAI Codex OAuth (ChatGPT-Abonnement)
hrafn auth login --provider openai-codex --device-code

# Gemini OAuth
hrafn auth login --provider gemini --profile default

# Anthropic setup-token
hrafn auth paste-token --provider anthropic --profile default --auth-kind authorization

# Prüfen / aktualisieren / Profil wechseln
hrafn auth status
hrafn auth refresh --provider openai-codex --profile default
hrafn auth use --provider openai-codex --profile work

# Agenten mit Abonnement-Auth ausführen
hrafn agent --provider openai-codex -m "hello"
hrafn agent --provider anthropic -m "hello"
```

## Agenten-Workspace + Skills

Workspace-Root: `~/.hrafn/workspace/` (konfigurierbar über Config).

Injizierte Prompt-Dateien:
- `IDENTITY.md` — Persönlichkeit und Rolle des Agenten
- `USER.md` — Benutzerkontext und Präferenzen
- `MEMORY.md` — Langzeitfakten und Lektionen
- `AGENTS.md` — Sitzungskonventionen und Initialisierungsregeln
- `SOUL.md` — Kernidentität und Betriebsprinzipien

Skills: `~/.hrafn/workspace/skills/<skill>/SKILL.md` oder `SKILL.toml`.

```bash
# Installierte Skills auflisten
hrafn skills list

# Von Git installieren
hrafn skills install https://github.com/user/my-skill.git

# Sicherheitsaudit vor der Installation
hrafn skills audit https://github.com/user/my-skill.git

# Einen Skill entfernen
hrafn skills remove my-skill
```

## CLI-Befehle

```bash
# Workspace-Verwaltung
hrafn onboard              # Geführter Einrichtungsassistent
hrafn status               # Daemon/Agenten-Status anzeigen
hrafn doctor               # Systemdiagnose ausführen

# Gateway + Daemon
hrafn gateway              # Gateway-Server starten (127.0.0.1:42617)
hrafn daemon               # Vollständige autonome Laufzeit starten

# Agent
hrafn agent                # Interaktiver Chat-Modus
hrafn agent -m "message"   # Einzelnachrichten-Modus

# Service-Verwaltung
hrafn service install      # Als OS-Dienst installieren (launchd/systemd)
hrafn service start|stop|restart|status

# Kanäle
hrafn channel list         # Konfigurierte Kanäle auflisten
hrafn channel doctor       # Kanalgesundheit prüfen
hrafn channel bind-telegram 123456789

# Cron + Planung
hrafn cron list            # Geplante Aufgaben auflisten
hrafn cron add "*/5 * * * *" --prompt "Check system health"
hrafn cron remove <id>

# Speicher
hrafn memory list          # Speichereinträge auflisten
hrafn memory get <key>     # Speicher abrufen
hrafn memory stats         # Speicherstatistiken

# Auth-Profile
hrafn auth login --provider <name>
hrafn auth status
hrafn auth use --provider <name> --profile <profile>

# Hardware-Peripherie
hrafn hardware discover    # Angeschlossene Geräte scannen
hrafn peripheral list      # Angeschlossene Peripherie auflisten
hrafn peripheral flash     # Firmware auf Gerät flashen

# Migration
hrafn migrate openclaw --dry-run
hrafn migrate openclaw

# Shell-Vervollständigung
source <(hrafn completions bash)
hrafn completions zsh > ~/.zfunc/_hrafn
```

Vollständige Befehlsreferenz: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)

<!-- markdownlint-disable MD001 MD024 -->

## Voraussetzungen

<details>
<summary><strong>Windows</strong></summary>

#### Erforderlich

1. **Visual Studio Build Tools** (stellt den MSVC-Linker und das Windows SDK bereit):

    ```powershell
    winget install Microsoft.VisualStudio.2022.BuildTools
    ```

    Wähle während der Installation (oder über den Visual Studio Installer) den Workload **"Desktopentwicklung mit C++"** aus.

2. **Rust-Toolchain:**

    ```powershell
    winget install Rustlang.Rustup
    ```

    Öffne nach der Installation ein neues Terminal und führe `rustup default stable` aus, um sicherzustellen, dass die stabile Toolchain aktiv ist.

3. **Überprüfe**, dass beide funktionieren:
    ```powershell
    rustc --version
    cargo --version
    ```

#### Optional

- **Docker Desktop** — nur erforderlich bei Verwendung der [Docker-Sandbox-Laufzeit](#laufzeitunterstützung-aktuell) (`runtime.kind = "docker"`). Installation über `winget install Docker.DockerDesktop`.

</details>

<details>
<summary><strong>Linux / macOS</strong></summary>

#### Erforderlich

1. **Grundlegende Build-Tools:**
    - **Linux (Debian/Ubuntu):** `sudo apt install build-essential pkg-config`
    - **Linux (Fedora/RHEL):** `sudo dnf group install development-tools && sudo dnf install pkg-config`
    - **macOS:** Xcode Command Line Tools installieren: `xcode-select --install`

2. **Rust-Toolchain:**

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Siehe [rustup.rs](https://rustup.rs) für Details.

3. **Überprüfe**, dass beide funktionieren:
    ```bash
    rustc --version
    cargo --version
    ```

#### Ein-Zeilen-Installer

Oder überspringe die obigen Schritte und installiere alles (Systemabhängigkeiten, Rust, Hrafn) mit einem einzigen Befehl:

```bash
curl -LsSf https://raw.githubusercontent.com/5queezer/hrafn/master/install.sh | bash
```

#### Ressourcenanforderungen für die Kompilierung

Das Kompilieren aus dem Quellcode benötigt mehr Ressourcen als das Ausführen des resultierenden Binary:

| Ressource      | Minimum | Empfohlen   |
| -------------- | ------- | ----------- |
| **RAM + Swap** | 2 GB    | 4 GB+       |
| **Freier Speicher** | 6 GB | 10 GB+     |

Wenn dein Host unter dem Minimum liegt, verwende vorgefertigte Binaries:

```bash
./install.sh --prefer-prebuilt
```

Um eine reine Binary-Installation ohne Quellcode-Fallback zu erfordern:

```bash
./install.sh --prebuilt-only
```

#### Optional

- **Docker** — nur erforderlich bei Verwendung der [Docker-Sandbox-Laufzeit](#laufzeitunterstützung-aktuell) (`runtime.kind = "docker"`). Installation über deinen Paketmanager oder [docker.com](https://docs.docker.com/engine/install/).

> **Hinweis:** Der Standard `cargo build --release` verwendet `codegen-units=1`, um den maximalen Kompilierungsdruck zu senken. Für schnellere Builds auf leistungsstarken Maschinen verwende `cargo build --profile release-fast`.

</details>

<!-- markdownlint-enable MD001 MD024 -->

### Vorgefertigte Binaries

Release-Assets werden veröffentlicht für:

- Linux: `x86_64`, `aarch64`, `armv7`
- macOS: `x86_64`, `aarch64`
- Windows: `x86_64`

Lade die neuesten Assets herunter von:
<https://github.com/5queezer/hrafn/releases/latest>

## Dokumentation

Verwende diese Ressourcen, wenn du den Onboarding-Prozess abgeschlossen hast und die tiefere Referenz benötigst.

- Starte mit dem [Docs-Index](docs/README.md) für die Navigation und "was ist wo."
- Lies die [Architekturübersicht](docs/architecture.md) für das vollständige Systemmodell.
- Verwende die [Konfigurationsreferenz](docs/reference/api/config-reference.md), wenn du jede Einstellung und jedes Beispiel brauchst.
- Betreibe das Gateway nach Buch mit dem [Betriebs-Runbook](docs/ops/operations-runbook.md).
- Folge [Hrafn Onboard](#schnellstart) für eine geführte Einrichtung.
- Behebe häufige Fehler mit der [Fehlerbehebungsanleitung](docs/ops/troubleshooting.md).
- Überprüfe die [Sicherheitshinweise](docs/security/README.md), bevor du etwas exponierst.

### Referenzdokumentation

- Dokumentations-Hub: [docs/README.md](docs/README.md)
- Einheitliches Docs-TOC: [docs/SUMMARY.md](docs/SUMMARY.md)
- Befehlsreferenz: [docs/reference/cli/commands-reference.md](docs/reference/cli/commands-reference.md)
- Konfigurationsreferenz: [docs/reference/api/config-reference.md](docs/reference/api/config-reference.md)
- Provider-Referenz: [docs/reference/api/providers-reference.md](docs/reference/api/providers-reference.md)
- Kanal-Referenz: [docs/reference/api/channels-reference.md](docs/reference/api/channels-reference.md)
- Betriebs-Runbook: [docs/ops/operations-runbook.md](docs/ops/operations-runbook.md)
- Fehlerbehebung: [docs/ops/troubleshooting.md](docs/ops/troubleshooting.md)

### Zusammenarbeitsdokumentation

- Beitragsleitfaden: [CONTRIBUTING.md](CONTRIBUTING.md)
- PR-Workflow-Richtlinie: [docs/contributing/pr-workflow.md](docs/contributing/pr-workflow.md)
- CI-Workflow-Leitfaden: [docs/contributing/ci-map.md](docs/contributing/ci-map.md)
- Reviewer-Handbuch: [docs/contributing/reviewer-playbook.md](docs/contributing/reviewer-playbook.md)
- Sicherheitsoffenlegungsrichtlinie: [SECURITY.md](SECURITY.md)
- Dokumentationsvorlage: [docs/contributing/doc-template.md](docs/contributing/doc-template.md)

### Bereitstellung + Betrieb

- Netzwerk-Bereitstellungsleitfaden: [docs/ops/network-deployment.md](docs/ops/network-deployment.md)
- Proxy-Agent-Handbuch: [docs/ops/proxy-agent-playbook.md](docs/ops/proxy-agent-playbook.md)
- Hardware-Leitfäden: [docs/hardware/README.md](docs/hardware/README.md)

## Smooth Crab 🦀

Hrafn wurde für den glatten Krebs 🦀 gebaut, einen schnellen und effizienten KI-Assistenten. Entwickelt von Argenis De La Rosa und der Community.

- [hrafn.ai](https://hrafn.ai)
- [@hrafn](https://x.com/hrafn)

## Hrafn unterstützen

Wenn Hrafn bei deiner Arbeit hilft und du die laufende Entwicklung unterstützen möchtest, kannst du hier spenden:

<a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=for-the-badge&logo=buy-me-a-coffee" alt="Buy Me a Coffee" /></a>

### 🙏 Besonderer Dank

Ein herzliches Dankeschön an die Communitys und Institutionen, die diese Open-Source-Arbeit inspirieren und antreiben:

- **Harvard University** — für die Förderung intellektueller Neugier und das Verschieben der Grenzen des Möglichen.
- **MIT** — für den Einsatz für offenes Wissen, Open Source und den Glauben, dass Technologie für alle zugänglich sein sollte.
- **Sundai Club** — für die Community, die Energie und den unermüdlichen Antrieb, Dinge zu bauen, die wichtig sind.
- **Die Welt und darüber hinaus** 🌍✨ — an jeden Mitwirkenden, Träumer und Erbauer, der Open Source zu einer Kraft für das Gute macht. Das ist für dich.

Wir bauen offen, weil die besten Ideen von überall kommen. Wenn du das hier liest, bist du Teil davon. Willkommen. 🦀❤️

## Beitragen

Neu bei Hrafn? Suche nach Issues mit dem Label [`good first issue`](https://github.com/5queezer/hrafn/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) — siehe unseren [Beitragsleitfaden](CONTRIBUTING.md#first-time-contributors) für den Einstieg. KI-/Vibe-coded PRs willkommen! 🤖

Siehe [CONTRIBUTING.md](CONTRIBUTING.md) und [CLA.md](docs/contributing/cla.md). Implementiere einen Trait, reiche einen PR ein:

- CI-Workflow-Leitfaden: [docs/contributing/ci-map.md](docs/contributing/ci-map.md)
- Neuer `Provider` → `src/providers/`
- Neuer `Channel` → `src/channels/`
- Neuer `Observer` → `src/observability/`
- Neues `Tool` → `src/tools/`
- Neuer `Memory` → `src/memory/`
- Neuer `Tunnel` → `src/tunnel/`
- Neues `Peripheral` → `src/peripherals/`
- Neuer `Skill` → `~/.hrafn/workspace/skills/<name>/`

<!-- BEGIN:RECENT_CONTRIBUTORS -->
<!-- END:RECENT_CONTRIBUTORS -->

## ⚠️ Offizielles Repository & Warnung vor Identitätsdiebstahl

**Dies ist das einzige offizielle Hrafn-Repository:**

> https://github.com/5queezer/hrafn

Jedes andere Repository, jede Organisation, Domain oder jedes Paket, das behauptet, "Hrafn" zu sein oder eine Zugehörigkeit zu Hrafn Labs impliziert, ist **nicht autorisiert und nicht mit diesem Projekt verbunden**. Bekannte nicht autorisierte Forks werden in [TRADEMARK.md](docs/maintainers/trademark.md) aufgelistet.

Wenn du auf Identitätsdiebstahl oder Markenrechtsmissbrauch stößt, [eröffne bitte ein Issue](https://github.com/5queezer/hrafn/issues).

---

## Lizenz

Hrafn ist doppelt lizenziert für maximale Offenheit und Schutz der Mitwirkenden:

| Lizenz | Anwendungsfall |
|---|---|
| [MIT](LICENSE-MIT) | Open Source, Forschung, akademisch, persönliche Nutzung |
| [Apache 2.0](LICENSE-APACHE) | Patentschutz, institutionell, kommerzielle Bereitstellung |

Du kannst eine der beiden Lizenzen wählen. **Mitwirkende gewähren automatisch Rechte unter beiden** — siehe [CLA.md](docs/contributing/cla.md) für die vollständige Mitwirkendenvereinbarung.

### Markenrecht

Der **Hrafn**-Name und das Logo sind Marken von Hrafn Labs. Diese Lizenz gewährt keine Erlaubnis, sie zu verwenden, um Unterstützung oder Zugehörigkeit zu implizieren. Siehe [TRADEMARK.md](docs/maintainers/trademark.md) für erlaubte und verbotene Verwendungen.

### Schutz für Mitwirkende

- Du **behältst das Urheberrecht** deiner Beiträge
- **Patentgewährung** (Apache 2.0) schützt dich vor Patentansprüchen anderer Mitwirkender
- Deine Beiträge werden **dauerhaft** in der Commit-Historie und [NOTICE](NOTICE) zugeordnet
- Keine Markenrechte werden durch Beiträge übertragen

---

**Hrafn** — Null Overhead. Null Kompromisse. Überall bereitstellen. Alles austauschen. 🦀

## Mitwirkende

<a href="https://github.com/5queezer/hrafn/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=5queezer/hrafn" alt="Hrafn contributors" />
</a>

Diese Liste wird aus dem GitHub-Mitwirkendengraph generiert und aktualisiert sich automatisch.

## Stern-Verlauf

<p align="center">
  <a href="https://www.star-history.com/#5queezer/hrafn&type=date&legend=top-left">
    <picture>
     <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&theme=dark&legend=top-left" />
     <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&legend=top-left" />
     <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=5queezer/hrafn&type=date&legend=top-left" />
    </picture>
  </a>
</p>
