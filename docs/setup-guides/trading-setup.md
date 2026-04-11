# Trading Signal Pipeline Setup

Hrafn can analyze trading signals from services like 100eyes by combining
Telegram channel monitoring with the Chartgen MCP server for chart generation
and technical analysis.

## Prerequisites

- A running [Chartgen](https://github.com/5queezer/chartgen) instance in trade
  mode: `chartgen --trade --testnet`
- A subscription to a signal provider (e.g. 100eyes Telegram channel)
- Telegram Bot API credentials (create a bot via
  [@BotFather](https://t.me/BotFather))
- Telegram API credentials (`api_id`, `api_hash`) from
  [my.telegram.org](https://my.telegram.org) if you plan to monitor channels
  with a user client

## Hrafn Configuration

### Chartgen MCP Server

Add the Chartgen MCP server to your `hrafn.toml` so the agent can call chart
generation and indicator tools:

```toml
[mcp]
enabled = true

[[mcp.servers]]
name = "chartgen"
transport = "http"
url = "https://chartgen.vasudev.xyz"
```

For local development, use stdio transport instead:

```toml
[[mcp.servers]]
name = "chartgen"
transport = "stdio"
command = "./chartgen"
args = ["--mcp"]
```

### Telegram Bot Channel

Configure the Telegram bot that receives forwarded signals or user queries:

```toml
[channels_config.telegram]
bot_token = "123456:ABC-DEF..."
allowed_users = ["your_telegram_username"]
```

### Telegram User Channel (optional)

If you want passive monitoring of third-party channels (e.g. 100eyes), add:

```toml
[channels_config.telegram_user]
api_id = 123456
api_hash = "your_api_hash"
phone = "+1 555 123 4567"
session_file = "~/.hrafn/telegram_user.session"
reply_via_bot = "@your_hrafn_bot"

[[channels_config.telegram_user.watch]]
channel = "100eyes"
handler = "trading_signal"
```

### Trading Signal Analysis Prompt

Hrafn loads personality/instruction files from the workspace directory. Create a
`SOUL.md` (or edit the existing one) in your Hrafn workspace
(`~/.hrafn/SOUL.md` by default) to include trading analysis instructions:

```markdown
# SOUL.md

You are a trading analyst assistant. When you receive a signal from 100eyes:

## Analysis Rules
- Use `chartgen__get_indicators` to fetch your own technical analysis
- No entry without Cipher B green dot confirmation
- ADX < 20 means no trend — do not trade
- Always generate your own chart and send it to the user
- Decide: place_order, set_alert, or skip — always with justification

## Response Format
1. Summarise the incoming signal (pair, direction, timeframe)
2. Run indicator checks (RSI, ADX, Cipher B)
3. Generate a chart via `chartgen__generate_chart`
4. State your decision with reasoning
```

Alternatively, if you use delegate agents, you can define a dedicated trading
analyst agent with its own system prompt:

```toml
[agents.trading_analyst]
provider = "anthropic"
model = "claude-sonnet-4-20250514"
agentic = true
allowed_tools = [
    "chartgen__generate_chart",
    "chartgen__list_indicators",
    "chartgen__get_indicators",
]
system_prompt = """
Du bist ein Trading-Analyst. Du erhältst Signale von 100eyes.
Nutze chartgen__get_indicators um eigene Analyse zu machen.
Regeln:
- Kein Entry ohne Cipher B grüner Dot
- ADX < 20 = kein Trend, nicht handeln
- Immer eigenen Chart generieren und an User senden
- Entscheidung: place_order, set_alert, oder skip mit Begründung
"""
```

## Testing

1. Start Chartgen: `chartgen --trade --testnet`
2. Start Hrafn: `hrafn`
3. Send a test trading signal image to your Hrafn Telegram bot
4. Verify the agent calls Chartgen MCP tools (`chartgen__generate_chart`,
   `chartgen__list_indicators`, `chartgen__get_indicators`) and responds with
   analysis

## Troubleshooting

- **MCP tools not appearing**: Ensure `[mcp] enabled = true` and the Chartgen
  server is reachable at the configured URL.
- **Agent not using tools**: Check that `deferred_loading` is working — the
  agent must call `tool_search` first if deferred loading is enabled (the
  default). Set `deferred_loading = false` to eagerly load all tool schemas.
- **Timeout errors**: Increase `tool_timeout_secs` on the MCP server config:
  ```toml
  [[mcp.servers]]
  name = "chartgen"
  transport = "http"
  url = "https://chartgen.vasudev.xyz"
  tool_timeout_secs = 60
  ```
