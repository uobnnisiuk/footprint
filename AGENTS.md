# AGENTS.md — footprint開発（自走運用SSOT）

このリポジトリは、複数エージェント（agent1〜agent6 / Codex profiles）で **footprint を自走で** 設計・実装・検証するための運用定義（SSOT）である。  
AGENTS.md は「運用・手順・品質ゲート・SSOTの置き場所」を固定し、**要件本文は docs/** に置く。

---

## 0. このファイルの優先順位
1) **AGENTS.md**（運用SSOT）  
2) `docs/constitution/`（憲法・仕様SSOT）  
3) `docs/rfc/`（RFC/DEC SSOT）  
4) `artifacts/`（タスク・実行ログ・証跡）

矛盾が出たら、上の優先順位で解決する。

---

## 1. パス/SSOT（読む場所を固定）
- 憲法/仕様（要件本文のSSOT）: `docs/constitution/`
  - `docs/constitution/00_constitution.md`（運用の原則・憲法）
  - `docs/constitution/10_core_fact_spec.md`（コア要件/仕様のSSOT）
  - `docs/constitution/20_share_envelope_spec.md`（共有エンベロープ仕様のSSOT）
- 受け入れ（Acceptance / TST）のSSOT: `docs/constitution/30_testplan.md`（owner = agent5）
- データ契約（schema 等）のSSOT: `docs/constitution/contracts/`
  - 例: `docs/constitution/contracts/trace.schema.json`
- mini RFC（合意の最小単位）: `docs/rfc/RFC-xxx.md`
- 決定ログ（DEC）: `docs/rfc/DEC-xxx.md`
- 成果物/実行ログ/スナップショット: `artifacts/`
  - エージェント定義/補助資料: `artifacts/agents/`
  - 受領（外から入る素材）: `artifacts/inbox/`
    - タスク（入力）: `artifacts/inbox/tasks/`
  - 共有コンテキスト束: `artifacts/packs/`
  - 実行ログ/証跡: `artifacts/runs/`
  - 提出/共有する最終物: `artifacts/outbox/`
  - テンプレート: `artifacts/templates/`
- 品質ゲート（単一エントリポイント）: `./ci.sh`

迷ったらこのパス定義を優先する。

---

## 2. 完全自走の定義（SSOT）
**完全自走**とは、次を満たす状態を指す。

- 人間の追加指示なしに、`./ci.sh` が green になるまで作業を継続できる。
- ブロック時は **停止しない**：`Q-xxx` を作り、**SRの仮定を1つ置いて前進**する（仮定は `IF-xxx`）。
- 例外（停止が許される条件）は `RISK-xxx` のみ（危険領域）。
- Work の完了条件は常に **「`./ci.sh` が green」**。

---

## 3. 要件SSOT（footprintの“中身”はここを見る）
- 要件本文のSSOTは `docs/constitution/10_core_fact_spec.md`。
- 共有のエンベロープ/配送/共有形状のSSOTは `docs/constitution/20_share_envelope_spec.md`。
- AGENTS.md は運用定義を保つため、**要件本文を増やさない**。
- ただし自走のため、以下の「要件インデックス（最小）」だけは AGENTS.md に固定する。

### 3.1 要件インデックス（最小）
- Goal: 「デジタル痕跡」を残し、後から追跡・共有・集計できること。
- Primary use-case: 災害時の捜索ヒント（通信断・端末故障・電池切れの現実を前提）。
- Secondary use-cases: 登山・釣り等の平時利用（平時から痕跡が溜まる運用を重視）。
- Non-goals: `docs/constitution/10_core_fact_spec.md` の Non-goals セクションを参照（ここでは定義しない）。
- Data contracts: `docs/constitution/contracts/` がデータ契約SSOT。実装は契約に従う。
- Acceptance: `docs/constitution/30_testplan.md` が受け入れSSOT（owner = agent5）。

### 3.2 「AGENTS.mdに書いてよい要件情報」の基準
- 書いてよい（インデックス扱い）:
  - Goal / Non-goals / Use-cases の **1行サマリ**
  - SSOT のパス参照（「詳細は docs/constitution を見よ」）
- 書いてはいけない:
  - 要件の詳細説明（目安：**3行以上になる場合**は `docs/constitution/` へ移動）
  - 設計判断の議論（mini RFC / DEC に移動）

### 3.3 仕様が未確定な場合の前進ルール
- `docs/constitution/10_core_fact_spec.md` が空/不足なら agent1 が最小骨格から埋める。
- 不明点は `OPEN`（仕様判断）か `Q-xxx`（実装中の疑問）に分類し、`IF-xxx` の仮定で前進する。
- 仕様が揺れる局面は mini RFC を使う（後述）。

---

## 4. 品質ゲート（Definition of Done）
- 品質の合否は **1つの標準コマンド**で判定する。
- 標準コマンド: `./ci.sh`
- すべての Work は、最終的に `./ci.sh` が green になることが完了条件。

### 4.1 `ci.sh` が満たすべき最低要件
- format / lint / typecheck（該当する言語だけ）
- unit tests
- integration tests（可能なら）
- security scan（依存脆弱性 / 既知の危険設定チェック：可能な範囲で）
- 失敗時は「原因ログ → 最小Diff修正 → 再実行」を繰り返す
- **no-op green 禁止**：実質的に何も実行しない green は禁止（“動いてないのに成功”を防ぐ）
- 注：bootstrap 段階では「最小チェック + どれか1つの実テスト」でもよいが、実装が進んだら必ず具体的テストを増やす。

### 4.2 受け入れ（Acceptance）の不可侵ルール
- 受け入れ条件（Acceptance）は agent5 が SSOT として定義する（`docs/constitution/30_testplan.md`）。
- 受け入れテストは **“通すために弱める変更” を禁止**。
- 受け入れテストの変更が必要な場合は、必ず mini RFC（SR + 理由 + 影響範囲）を切り、DEC を残す。

---

## 5. 初回 bootstrap（自走ブロッカー解除）
SR: **最初に agent4 が “最小スケルトン” を整備**し、全エージェントが `./ci.sh` を実行できる状態を作る。  
ただし **no-op green は禁止**。最低限の検証（ファイル整合・契約JSON妥当性など）が必ず走ること。

### 5.1 必須の初期ファイル（最小）
- `./ci.sh`（bootstrap段階でも「何かを検証」すること。以後ここがSSOT）
- `docs/constitution/10_core_fact_spec.md`（骨格のみで可）
- `docs/constitution/30_testplan.md`（骨格のみで可。owner = agent5）
- `docs/constitution/contracts/trace.schema.json`（最小スキーマ）
- `artifacts/inbox/tasks/TASK-0100-backend.md`（最小サンプル）
- `docs/rfc/RFC-0000-template.md`（すでに存在。RFCの雛形）
- `artifacts/inbox/tasks/TASK-0000-template.md`（すでに存在。Taskの雛形）

### 5.2 コピペ用スケルトン（初期コミット用）
#### (A) `./ci.sh`（bootstrap最小）
```bash
#!/usr/bin/env bash
set -euo pipefail

echo "[ci] start"

ran=0
run() { ran=1; "$@"; }

# --- Always-on minimal checks (bootstrapでも必ず何か走る) ---
run test -f docs/constitution/10_core_fact_spec.md
run test -f docs/constitution/30_testplan.md
run test -f docs/constitution/contracts/trace.schema.json

# JSON schema file must be valid JSON (use python3 if available; fallback to basic check)
if command -v python3 >/dev/null 2>&1; then
  run python3 -m json.tool docs/constitution/contracts/trace.schema.json >/dev/null
elif command -v node >/dev/null 2>&1; then
  run node -e "JSON.parse(require('fs').readFileSync('docs/constitution/contracts/trace.schema.json','utf8'));"
else
  echo "[ci] WARN: no python3/node; JSON validity check skipped"
  ran=1
fi

# --- Optional project checks (存在するものだけ実行) ---
# NOTE: 実装ディレクトリが追加されたら、ここに段階的にゲートを増やす（入口はci.shのまま）
if test -f core/Cargo.toml; then run cargo test --manifest-path core/Cargo.toml; fi
if test -f share/Cargo.toml; then run cargo test --manifest-path share/Cargo.toml; fi
if test -f aggregate/Cargo.toml; then run cargo test --manifest-path aggregate/Cargo.toml; fi
if test -f platform/Cargo.toml; then run cargo test --manifest-path platform/Cargo.toml; fi

if test -f backend/package.json; then
  if command -v pnpm >/dev/null 2>&1; then
    run pnpm -C backend test
  else
    echo "[ci] WARN: pnpm not found; backend tests skipped"
    ran=1
  fi
fi

if test -f android/gradlew; then
  run bash -lc "cd android && ./gradlew test"
fi

if test "$ran" -eq 0; then
  echo "[ci] ERROR: no-op green is forbidden. Configure at least one real check target."
  exit 1
fi

echo "[ci] green"
```

#### (B) `docs/constitution/10_core_fact_spec.md`（最小骨格）
```md
# footprint core fact spec (SSOT)

## 1. Purpose / Goal

## 2. Non-goals

## 3. Use-cases
- UC-xxx: ...

## 4. System overview

## 5. Data model (contracts)
- docs/constitution/contracts/ 配下の schema を参照

## 6. Interfaces / API

## 7. Ops / Deployment

## 8. Security / Privacy

## 9. Acceptance (SSOT owner = agent5)
- docs/constitution/30_testplan.md を参照

## 10. OPEN
- 仕様レベルの未決事項（採否/設計判断）
```

#### (C) `docs/constitution/30_testplan.md`（最小骨格）
```md
# footprint acceptance / testplan (SSOT)

この文書は「観測可能な受け入れ条件（Acceptance）」のSSOTである。実装の内部構造ではなく、挙動で定義する。

## Acceptance (minimal)
- TST-0001: `./ci.sh` が green になる
- TST-0002: contracts の schema ファイルが有効な JSON である
- TST-0003: （後で追加）痕跡を1件保存でき、保存結果を観測できる（API/CLI/ログなど。手段はspecで決める）

## Non-negotiable rules
- “通すために弱める変更”は禁止
- 変更が必要なら mini RFC + DEC を残す
```

#### (D) `docs/constitution/contracts/trace.schema.json`（最小例）
```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "docs/constitution/contracts/trace.schema.json",
  "title": "Trace",
  "type": "object",
  "additionalProperties": false,
  "required": ["traceId", "deviceId", "capturedAt", "kind"],
  "properties": {
    "traceId": { "type": "string", "minLength": 1 },
    "deviceId": { "type": "string", "minLength": 1 },
    "capturedAt": { "type": "string", "format": "date-time" },
    "kind": { "type": "string", "enum": ["beacon", "manual", "sensor", "network"] },
    "payload": { "type": "object" }
  }
}
```

#### (E) `artifacts/inbox/tasks/TASK-0100-backend.md`（最小サンプル）
```md
# artifacts/inbox/tasks/TASK-0100-backend.md

## 目的（SR）
痕跡を1件「保存」できる最小スライスを実装し、`./ci.sh` を green にする。

## 前提（不変条件）
- IF-001: データモデルは `docs/constitution/contracts/trace.schema.json` に従う
- IF-002: 受け入れSSOTは `docs/constitution/30_testplan.md`（弱める変更は禁止）

## 変更範囲（Diff最小）
- 実装パスはリポジトリの現実構造に従う（例：Rustなら `core/`、Nodeなら `backend/` 等。存在するものだけ触る）
- 新規追加の入口（API/CLI/関数）は `docs/constitution/10_core_fact_spec.md` に合わせる

## 受け入れ条件（TST）
- TST-0100-1: `./ci.sh` が green
- TST-0100-2: 保存が成功したことを観測できる（レスポンス/出力/ログ等。観測手段は spec に従う）
- TST-0100-3: 不正入力（schema違反）で失敗が観測できる（例：400/エラーコード/エラーメッセージ）

## 生成物
- `artifacts/runs/TASK-0100-backend.out.md`（実行ログ）
```

### 5.3 初回コミットの主体（迷いをなくす）
- 初回の bootstrap コミットは **人間（リポジトリオーナー）** が行う（5.2 をコピペしてコミット）。
- 以後の `ci.sh` 保守・拡張は **agent4** が担当する。

---

## 6. 基本ルール（全エージェント共通）
- 提案は SR（単一推奨）を明言し、理由を短く添える。
- 進捗リセット禁止：合意済み前提は不変条件として扱う。
- “簡略化”= 再構成＋深い説明。**情報削除・精度低下は禁止**。
- 暫定/デバッグ用コードは禁止。最終コードのみ。
- 既存の変数名/関数名は変えない。変える場合は事前に一言＋理由。
- 失敗したら「ログ → 最小Diff修正 → 再実行」を回す。
- タスク本文（エージェントに渡す指示）は必ず ``` で囲う。

---

## 7. タグ運用（止まらないための最小）
- `OPEN`: 仕様レベルの未決定事項（採否、設計判断）
- `Q-xxx`: 実装中の疑問（ブロック回避のため仮定が必要な箇所）
- `IF-xxx`: 仮定・不変条件（Qへの暫定回答を含む）
- `DEC-xxx`: 決定
- `RISK-xxx`: 危険領域（停止が必要）
- `TST-xxx`: 受け入れ/検証項目

---

## 8. mini RFC（変更の最小合意ユニット）
- ファイル: `docs/rfc/RFC-xxx.md`
- 必須項目:
  - タイトル
  - SR（推奨案）
  - 理由
  - 却下案と理由（必要なら）
  - 影響範囲（影響ファイル / 互換性 / 運用）
- 承認（完全自走モード）:
  - 通常の変更：agent1（またはagent6）が `DEC-xxx` を付与して採用可
  - **人間承認が必須**：`RISK-xxx` に該当、または互換性破壊/データ破壊/プライバシー重大影響の変更
- 決定ログ:
  - `docs/rfc/DEC-xxx.md` に記録する

### DEC テンプレート（最小）
```md
# DEC-xxx: タイトル

## 日付
YYYY-MM-DD

## 決定内容（SR）
- ...

## 理由
- ...

## 関連
- RFC-xxx（あれば）
- 影響を受ける OPEN / Q-xxx
```

---

## 9. ExecPlan（任意だが推奨：複雑タスクの自走安定化）
- 大きめの機能追加・大リファクタ・未知が多い変更では ExecPlan を推奨する。
- パス（現在の構成に合わせる）: `artifacts/inbox/tasks/EP-xxxx-<topic>.md`
  - 1ファイル自己完結（それ単体で初心者が実装できる）を目標にする。
- 実装エージェントは「次の指示待ち」をしない。ExecPlan の次マイルストーンへ進み、止まるときは Progress を更新する。

**ExecPlan を必須にする条件（推奨ルール）**:
- 変更対象が複数ディレクトリに跨る
- 外部ライブラリ/外部サービスの導入を含む
- 仕様未確定（OPEN/Q）が **3件以上** ある状態で実装を開始する

---

## 10. マルチエージェント実行プロトコル（ハンドオフ）
依存グラフ:
```
agent1 (spec) ──┬──▶ agent5 (Acceptance/TST)
               │
               └──▶ agent2/3 (実装) ──▶ agent4 (CI green維持) ──▶ agent6 (統合)
```

幅が狭い端末で崩れる場合の補足（箇条書き）:
- agent1 → agent5（spec確定 → Acceptance定義）
- agent1 → agent2/3（spec確定 → 実装）
- agent2/3 → agent4（実装 → CI green維持）
- agent4 → agent6（CI green → 統合編集）

標準手順:
1. agent1: `docs/constitution/10_core_fact_spec.md` を更新（SR / OPEN を反映）
2. agent5: `docs/constitution/30_testplan.md` を更新（Acceptance/TST をSSOT化）
3. agent2/agent3: 実装 + 必要テスト追加（Acceptance を満たす）
4. agent4: `ci.sh` を整備し、green を維持（入口は常に ci.sh）
5. agent6: 統合編集（矛盾解消・SSOT整合）

ブロック時:
- `Q-xxx` を作り、SRの仮定を1つ置いて前進（`IF-xxx` で明示）。
- 危険領域は停止し、`RISK-xxx` と代替策を出す。

---

## 11. Roles（Codex profiles と対応）
このリポジトリでは `--profile agentN` を「役割（Role）」として運用する。

### agent1（Spec Lead）
- Objective: 仕様SSOT（docs/constitution）を前進させる（SRで決める）。
- Scope: `docs/constitution/10_core_fact_spec.md` と関連仕様（必要なら `20_share_envelope_spec.md` も更新）。
- Defaults: 不明点は OPEN/Q/IF で前進可能な形に分解する。
- Guardrails: 要件本文を AGENTS.md に増やさない。
- Outputs: `docs/constitution/*`（specへの反映点）と `docs/rfc/*`（RFC/DECが必要な場合）。

### agent2（Backend）
- Objective: data contract準拠でサーバ側（または永続化/共有/集計）を実装し、`./ci.sh` を green にする。
- Defaults: 最小Diffで進め、必要テストを同時に足す。
- Guardrails: 契約（`docs/constitution/contracts/`）に反する実装は禁止。Acceptance弱体化は禁止。
- Inputs: `artifacts/inbox/tasks/*`、`docs/constitution/*`
- Outputs: `artifacts/runs/*`、最終コード、テスト、`./ci.sh` green。

### agent3（Android）
- Objective: data contract準拠で端末側を実装し、`./ci.sh` を green にする。
- Defaults: 最小Diffで進め、必要テストを同時に足す。
- Guardrails: 契約（`docs/constitution/contracts/`）に反する実装は禁止。Acceptance弱体化は禁止。
- Inputs: `artifacts/inbox/tasks/*`、`docs/constitution/*`
- Outputs: `artifacts/runs/*`、最終コード、テスト、`./ci.sh` green。

### agent4（Infra / DevEx）
- Objective: `./ci.sh` をSSOTとして整備し、再現性を固定する。
- Defaults: 入口は常に `./ci.sh` に統一。段階的にゲートを増やす。
- Guardrails: “別の手順” を増やさない（入口は ci.sh のみ）。
- Inputs: `artifacts/runs/*`（躓きログ）
- Outputs: `./ci.sh` green維持、bootstrap整備。

### agent5（QA / Security）
- Objective: 受け入れ条件（Acceptance/TST）のSSOTを定義し、守らせる。
- Scope: `docs/constitution/30_testplan.md`
- Defaults: 受け入れは観測可能・自動実行可能に寄せる。
- Guardrails: “通すために弱める変更” 禁止（必要なら mini RFC + DEC）。
- Outputs: `docs/constitution/30_testplan.md`、TST一覧、ゲート要求。

### agent6（Integrator / Editor）
- Objective: docs/constitution ↔ docs/rfc ↔ artifacts の整合を取り、矛盾を潰す。
- Defaults: 情報は削除しない。不要になった記述は **deprecated** に落とし、理由と置換先を明記する。
- Guardrails: 章構造の大改造やSSOT移動は mini RFCなしに行わない。
- Outputs: 統一版 `docs/constitution/*` / `docs/rfc/*`、参照関係の修正、deprecated管理。

---

## 12. セキュリティ/秘密情報（最低限ガードレール）
- APIキー・トークン・個人情報はログ/成果物に出力しない。
- 外部ネットワークアクセスは必要最小限。不要なら無効化する。
- `.env` や秘密情報ファイルはリポジトリにコミットしない。
- 外部サービス導入で **課金が発生する可能性** がある場合は `RISK-xxx` 扱い（停止して代替策を提示）。

---

## 13. Codex 実行コマンド（運用）
通常実行（例）:
```sh
codex exec --profile agent2 --color never \
  --output-last-message artifacts/runs/TASK-0100-backend.out.md \
  - < artifacts/inbox/tasks/TASK-0100-backend.md
```

resume（SESSION_ID を使う）:
```sh
codex exec resume --color never \
  --output-last-message artifacts/runs/TASK-0100-backend.out.md \
  <SESSION_ID> - < artifacts/inbox/tasks/TASK-0100-backend-resume.md
```

---

## 14. このファイルの更新ルール
- AGENTS.md は運用のSSOT。役割や手順が変わったら必ず更新する。
- 既存記述は原則削除しない。削除ではなく deprecated 運用を基本とする（理由と置換先を併記）。
- deprecated 記述は同一ファイル末尾の `## Deprecated` セクションに移動する。
- フォーマット: `<!-- DEPRECATED YYYY-MM-DD: 理由 / 置換先 -->`

---

## Deprecated
<!-- DEPRECATED YYYY-MM-DD: 理由 / 置換先 -->
