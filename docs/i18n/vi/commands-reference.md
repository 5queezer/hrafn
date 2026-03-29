# Tham khảo lệnh Hrafn

Dựa trên CLI hiện tại (`hrafn --help`).

Xác minh lần cuối: **2026-02-20**.

## Lệnh cấp cao nhất

| Lệnh | Mục đích |
|---|---|
| `onboard` | Khởi tạo workspace/config nhanh hoặc tương tác |
| `agent` | Chạy chat tương tác hoặc chế độ gửi tin nhắn đơn |
| `gateway` | Khởi động gateway webhook và HTTP WhatsApp |
| `daemon` | Khởi động runtime có giám sát (gateway + channels + heartbeat/scheduler tùy chọn) |
| `service` | Quản lý vòng đời dịch vụ cấp hệ điều hành |
| `doctor` | Chạy chẩn đoán và kiểm tra trạng thái |
| `status` | Hiển thị cấu hình và tóm tắt hệ thống |
| `cron` | Quản lý tác vụ định kỳ |
| `models` | Làm mới danh mục model của provider |
| `providers` | Liệt kê ID provider, bí danh và provider đang dùng |
| `channel` | Quản lý kênh và kiểm tra sức khỏe kênh |
| `integrations` | Kiểm tra chi tiết tích hợp |
| `skills` | Liệt kê/cài đặt/gỡ bỏ skills |
| `migrate` | Nhập dữ liệu từ runtime khác (hiện hỗ trợ OpenClaw) |
| `config` | Xuất schema cấu hình dạng máy đọc được |
| `completions` | Tạo script tự hoàn thành cho shell ra stdout |
| `hardware` | Phát hiện và kiểm tra phần cứng USB |
| `peripheral` | Cấu hình và nạp firmware thiết bị ngoại vi |

## Nhóm lệnh

### `onboard`

- `hrafn onboard`
- `hrafn onboard --channels-only`
- `hrafn onboard --api-key <KEY> --provider <ID> --memory <sqlite|lucid|markdown|none>`
- `hrafn onboard --api-key <KEY> --provider <ID> --model <MODEL_ID> --memory <sqlite|lucid|markdown|none>`

### `agent`

- `hrafn agent`
- `hrafn agent -m "Hello"`
- `hrafn agent --provider <ID> --model <MODEL> --temperature <0.0-2.0>`
- `hrafn agent --peripheral <board:path>`

### `gateway` / `daemon`

- `hrafn gateway [--host <HOST>] [--port <PORT>]`
- `hrafn daemon [--host <HOST>] [--port <PORT>]`

### `service`

- `hrafn service install`
- `hrafn service start`
- `hrafn service stop`
- `hrafn service restart`
- `hrafn service status`
- `hrafn service uninstall`

### `cron`

- `hrafn cron list`
- `hrafn cron add <expr> [--tz <IANA_TZ>] <command>`
- `hrafn cron add-at <rfc3339_timestamp> <command>`
- `hrafn cron add-every <every_ms> <command>`
- `hrafn cron once <delay> <command>`
- `hrafn cron remove <id>`
- `hrafn cron pause <id>`
- `hrafn cron resume <id>`

### `models`

- `hrafn models refresh`
- `hrafn models refresh --provider <ID>`
- `hrafn models refresh --force`

`models refresh` hiện hỗ trợ làm mới danh mục trực tiếp cho các provider: `openrouter`, `openai`, `anthropic`, `groq`, `mistral`, `deepseek`, `xai`, `together-ai`, `gemini`, `ollama`, `astrai`, `venice`, `fireworks`, `cohere`, `moonshot`, `glm`, `zai`, `qwen` và `nvidia`.

### `channel`

- `hrafn channel list`
- `hrafn channel start`
- `hrafn channel doctor`
- `hrafn channel bind-telegram <IDENTITY>`
- `hrafn channel add <type> <json>`
- `hrafn channel remove <name>`

Lệnh trong chat khi runtime đang chạy (Telegram/Discord):

- `/models`
- `/models <provider>`
- `/model`
- `/model <model-id>`

Channel runtime cũng theo dõi `config.toml` và tự động áp dụng thay đổi cho:
- `default_provider`
- `default_model`
- `default_temperature`
- `api_key` / `api_url` (cho provider mặc định)
- `reliability.*` cài đặt retry của provider

`add/remove` hiện chuyển hướng về thiết lập có hướng dẫn / cấu hình thủ công (chưa hỗ trợ đầy đủ mutator khai báo).

### `integrations`

- `hrafn integrations info <name>`

### `skills`

- `hrafn skills list`
- `hrafn skills install <source>`
- `hrafn skills remove <name>`

`<source>` chấp nhận git remote (`https://...`, `http://...`, `ssh://...` và `git@host:owner/repo.git`) hoặc đường dẫn cục bộ.

Skill manifest (`SKILL.toml`) hỗ trợ `prompts` và `[[tools]]`; cả hai được đưa vào system prompt của agent khi chạy, giúp model có thể tuân theo hướng dẫn skill mà không cần đọc thủ công.

### `migrate`

- `hrafn migrate openclaw [--source <path>] [--dry-run]`

### `config`

- `hrafn config schema`

`config schema` xuất JSON Schema (draft 2020-12) cho toàn bộ hợp đồng `config.toml` ra stdout.

### `completions`

- `hrafn completions bash`
- `hrafn completions fish`
- `hrafn completions zsh`
- `hrafn completions powershell`
- `hrafn completions elvish`

`completions` chỉ xuất ra stdout để script có thể được source trực tiếp mà không bị lẫn log/cảnh báo.

### `hardware`

- `hrafn hardware discover`
- `hrafn hardware introspect <path>`
- `hrafn hardware info [--chip <chip_name>]`

### `peripheral`

- `hrafn peripheral list`
- `hrafn peripheral add <board> <path>`
- `hrafn peripheral flash [--port <serial_port>]`
- `hrafn peripheral setup-uno-q [--host <ip_or_host>]`
- `hrafn peripheral flash-nucleo`

## Kiểm tra nhanh

Để xác minh nhanh tài liệu với binary hiện tại:

```bash
hrafn --help
hrafn <command> --help
```
