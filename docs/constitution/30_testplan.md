# footprint acceptance / testplan (SSOT)

この文書は「観測可能な受け入れ条件（Acceptance）」のSSOTである。実装の内部構造ではなく、挙動で定義する。

## Acceptance (minimal)
- TST-0001: `./ci.sh` が green になる
- TST-0002: contracts の schema ファイルが有効な JSON である
- TST-0003: （後で追加）痕跡を1件保存でき、保存結果を観測できる（API/CLI/ログなど。手段はspecで決める）
- TST-0004: **送達キュー（Outbox）のクラッシュ復旧** — enqueue が成功を返した通知は、プロセス強制終了後の再起動でキューに残っていること（IF-NOTIFIED-001 の観測可能な受け入れ条件）
- TST-0005: `docs/constitution/contracts/trace.schema.json` の `required` に `traceId` / `deviceId` / `capturedAt` / `kind` が含まれることを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; req=set(json.load(open('docs/constitution/contracts/trace.schema.json'))['required']); need={'traceId','deviceId','capturedAt','kind'}; print('required=',sorted(req)); sys.exit(0 if need<=req else 1)"`

## Non-negotiable rules
- “通すために弱める変更”は禁止
- 変更が必要なら mini RFC + DEC を残す
