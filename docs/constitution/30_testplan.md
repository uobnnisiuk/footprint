# footprint acceptance / testplan (SSOT)

この文書は「観測可能な受け入れ条件（Acceptance）」のSSOTである。実装の内部構造ではなく、挙動で定義する。

## Acceptance (minimal)
- TST-0001: `./ci.sh` が green になる
- TST-0002: contracts の schema ファイルが有効な JSON である
- TST-0003: （後で追加）痕跡を1件保存でき、保存結果を観測できる（API/CLI/ログなど。手段はspecで決める）
- TST-0004: **送達キュー（Outbox）のクラッシュ復旧** — enqueue が成功を返した通知は、プロセス強制終了後の再起動でキューに残っていること（IF-NOTIFIED-001 の観測可能な受け入れ条件）
- TST-0005: `docs/constitution/contracts/trace.schema.json` の `required` に `traceId` / `deviceId` / `capturedAt` / `kind` が含まれることを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; req=set(json.load(open('docs/constitution/contracts/trace.schema.json'))['required']); need={'traceId','deviceId','capturedAt','kind'}; print('required=',sorted(req)); sys.exit(0 if need<=req else 1)"`
- TST-0006: `docs/constitution/contracts/trace.schema.json` が未知フィールドを拒否する（`additionalProperties: false`）ことを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; s=json.load(open('docs/constitution/contracts/trace.schema.json')); print('additionalProperties=',s.get('additionalProperties')); sys.exit(0 if s.get('additionalProperties') is False else 1)"`
- TST-0007: `docs/constitution/contracts/trace.schema.json` の `kind` 列挙値が `beacon` / `manual` / `sensor` / `network` を含むことを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; e=set(json.load(open('docs/constitution/contracts/trace.schema.json'))['properties']['kind']['enum']); need={'beacon','manual','sensor','network'}; print('kind.enum=',sorted(e)); sys.exit(0 if need<=e else 1)"`
- TST-0008: `docs/constitution/contracts/trace.schema.json` の `capturedAt` が `type=string` かつ `format=date-time` であることを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; p=json.load(open('docs/constitution/contracts/trace.schema.json'))['properties']['capturedAt']; ok=(p.get('type')=='string' and p.get('format')=='date-time'); print('capturedAt=',p); sys.exit(0 if ok else 1)"`
- TST-0009: `docs/constitution/contracts/trace.schema.json` の `payload` が任意（`required` に含まれない）かつ `type=object` であることを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; s=json.load(open('docs/constitution/contracts/trace.schema.json')); req=set(s.get('required',[])); p=s['properties'].get('payload',{}); ok=('payload' not in req and p.get('type')=='object'); print('payload=',p,'required_contains_payload=',('payload' in req)); sys.exit(0 if ok else 1)"`
- TST-0010: `docs/constitution/contracts/trace.schema.json` の `traceId` と `deviceId` が空文字を許容しない（`minLength=1`）ことを観測できる
  - 観測手順（例）: `python3 -c "import json,sys; p=json.load(open('docs/constitution/contracts/trace.schema.json'))['properties']; ok=(p.get('traceId',{}).get('minLength')==1 and p.get('deviceId',{}).get('minLength')==1); print('traceId.minLength=',p.get('traceId',{}).get('minLength'),'deviceId.minLength=',p.get('deviceId',{}).get('minLength')); sys.exit(0 if ok else 1)"`
- TST-0011: `docs/rfc/DEC-0004-encounter-capsule-key.md` において、遭遇カプセル鍵の SR が救助機関鍵（Authorized Rescue 公開鍵）であることを観測できる
  - 観測手順（例）: `grep -nE "決定内容（SR）|救助機関|Authorized Rescue 公開鍵" docs/rfc/DEC-0004-encounter-capsule-key.md`

## Non-negotiable rules
- “通すために弱める変更”は禁止
- 変更が必要なら mini RFC + DEC を残す
