# footprint acceptance / testplan (SSOT)

この文書は「観測可能な受け入れ条件（Acceptance）」のSSOTである。実装の内部構造ではなく、挙動で定義する。

## Acceptance (minimal)
- TST-0001: `./ci.sh` が green になる
- TST-0002: contracts の schema ファイルが有効な JSON である
- TST-0003: （後で追加）痕跡を1件保存でき、保存結果を観測できる（API/CLI/ログなど。手段はspecで決める）

## Non-negotiable rules
- “通すために弱める変更”は禁止
- 変更が必要なら mini RFC + DEC を残す
