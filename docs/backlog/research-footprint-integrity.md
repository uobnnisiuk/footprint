# 調査レポート: 痕跡（footprint）情報の高信頼化（改ざん検知＋外部アンカー）

**対象アイデア**: IDEA-0003: 「痕跡（footprint）情報の高信頼化（改ざん検知＋外部アンカー）」
**調査日**: 2026-02-03
**調査者**: Claude (AI Assistant)

---

## エグゼクティブサマリー

痕跡データの改ざん検知・第三者検証可能性を高めるために、コア（オフライン完結の hash chain + 署名）とオプション（オンライン時の外部アンカー）の2層構成について調査した。

**主な結論**:
- **コア設計は既に SSOT に整合**: RFC-0002 で L1 に hash chain + 署名が確定済み。IDEA-0003 のコア部分は既存設計の延長で成立する
- **署名アルゴリズムは Ed25519 を推奨**: 決定論的署名（モバイルのエントロピー問題を回避）、高速、コンパクト
- **外部アンカーは OpenTimestamps（Bitcoin）が最有力**: 無料、信頼分散、ハッシュのみで最小開示。TSA（RFC 3161）も併用候補
- **鍵管理が最大の運用リスク**: Android Keystore / iOS Secure Enclave で保護可能だが、端末紛失・移行時の設計が必要
- **プライバシーは「ハッシュのみ外部公開」で対応可能**: 個人データは外部に出さず、Merkle root のみをアンカー

**推奨**: まず L1 の hash chain + 署名を PoC 実装し、検証の再現性を確認。外部アンカーは OpenTimestamps の統合を「任意拡張」として後追いで検証する

---

## 1. 既存設計との関係（現在地の確認）

### 1.1 RFC-0002 で確定済みの設計

RFC-0002（L0/L1 Integrity 責務境界）により、以下が既に SSOT に反映されている:

| レイヤ | 責務 | 暗号学的 integrity |
|--------|------|-------------------|
| L0（Core Fact） | 正規化（canonical bytes）の定義 | **持たない**（IF-INTEG-001） |
| L1（Share Envelope） | hash chain 構築 + 署名 | **必須**（IF-INTEG-002） |

**hash chain アルゴリズム**（20_share_envelope_spec.md より）:
```
e[i] = canonical(payload_events[i])
h[-1] = 0x00...00
h[i] = H( h[i-1] || e[i] )
```

**署名**: `chain_tail` を署名対象に含め、payload 全体の再シリアライズを回避（IF-INTEG-003）。

### 1.2 IDEA-0003 が追加で扱う範囲

既存設計で **未決定** のまま残されている事項:

| 項目 | 状態 | IDEA-0003 での扱い |
|------|------|-------------------|
| 署名アルゴリズムの具体選定 | 「例: Ed25519」のみ記載 | 本レポート 2章で調査 |
| 鍵管理（生成・ローテ・失効・復旧） | Non-goal として明示的に後回し | 本レポート 3章で調査 |
| 外部アンカー（存在証明の固定） | IDEA-0003 のオプション層 | 本レポート 4章で調査 |
| 第三者検証手順の標準化 | 未着手 | 本レポート 5章で考察 |

---

## 2. 署名アルゴリズムの選定

### 2.1 候補比較

| 項目 | Ed25519 (EdDSA) | ECDSA (P-256) | RSA-2048 |
|------|-----------------|---------------|----------|
| 鍵サイズ | 32 bytes | 32 bytes | 256 bytes |
| 署名サイズ | 64 bytes | ~72 bytes | 256 bytes |
| セキュリティ強度 | ~128-bit | ~128-bit | ~112-bit |
| 署名速度 | 最速 | 速い | 遅い（~33倍遅い） |
| 検証速度 | 速い | 速い | 最速 |
| ノンス | **決定論的**（安全） | ランダム（漏洩リスク） | N/A |
| サイドチャネル耐性 | 設計に組込み | 慎重な実装が必要 | 要注意 |
| FIPS 認定 | あり（FIPS 186-5） | あり | あり |
| モバイル HW 対応 | Android Keystore: 対応、iOS Secure Enclave: P-256 のみ | 両方対応 | 両方対応 |

**出典**: [Digital Signatures 2025: ECDSA vs EdDSA](https://www.onlinehashcrack.com/guides/cryptography-algorithms/digital-signatures-2025-ecdsa-vs-eddsa.php), [EdDSA and Ed25519 | Practical Cryptography](https://cryptobook.nakov.com/digital-signatures/eddsa-and-ed25519), [Digital Signatures: Mechanics and Go Benchmarks](https://dev.to/kanywst/digital-signatures-mechanics-and-go-benchmarks-rsa-vs-ecdsa-vs-ed25519-2d36)

### 2.2 推奨: Ed25519

**Ed25519 を第一候補とする理由**:

1. **決定論的署名**: ランダムノンスに依存しないため、モバイル端末でのエントロピー品質問題を回避できる。ECDSA は「署名時に良質な乱数が生成できないと秘密鍵が漏洩する」という致命的な弱点がある
2. **コンパクト**: 鍵 32 bytes + 署名 64 bytes は、帯域制約のある災害時通信に適する
3. **高速**: 署名・検証とも高速で、バッテリー制約のあるモバイル端末に適する
4. **Rust エコシステム**: `ed25519-dalek` クレートが成熟しており、IF-IMPL-003（Rust 実装）と相性が良い

**注意点（iOS Secure Enclave の制約）**:
- iOS Secure Enclave は **P-256 のみ** をハードウェア保護で対応。Ed25519 はソフトウェア実装になる
- 対策案: iOS では Secure Enclave の P-256 ECDSA を使い、アルゴリズムを `sig_alg` フィールドで識別する設計（マルチアルゴリズム対応）
- あるいは: Ed25519 をソフトウェア実装とし、Keychain で鍵を保護する（ハードウェア分離は諦める）

### 2.3 ハッシュアルゴリズム

| 項目 | SHA-256 | BLAKE3 | SHA-3 (Keccak) |
|------|---------|--------|-----------------|
| 速度 | 十分高速 | 最速（SIMD活用） | やや遅い |
| 出力 | 32 bytes | 可変（デフォルト32 bytes） | 32 bytes |
| 普及度 | 最も広い | 成長中 | 中程度 |
| 標準化 | NIST FIPS 180-4 | 未NIST | NIST FIPS 202 |
| Bitcoin 互換 | そのまま利用可 | 変換要 | 変換要 |

**推奨**: **SHA-256**。OpenTimestamps（Bitcoin）との互換性、NIST 標準、十分な速度。envelope spec に `hash_alg: "SHA-256"` と既に記載されており整合する。

---

## 3. 鍵管理

### 3.1 モバイル端末での鍵保護

| プラットフォーム | 保護機構 | 特徴 | 制約 |
|----------------|---------|------|------|
| Android Keystore (TEE/StrongBox) | Trusted Execution Environment または Secure Element | 鍵がアプリプロセスに入らない。Ed25519 対応 | 一部端末は TEE なし（ソフトウェアフォールバック） |
| iOS Secure Enclave | 専用ハードウェアコプロセッサ | 鍵生成・署名を Enclave 内で完結 | **P-256 ECDSA のみ**。鍵のインポート不可 |
| iOS Keychain | 暗号化ストレージ | Ed25519 鍵をソフトウェアで保持可能 | Secure Enclave ほどの分離はない |

**出典**: [Android Keystore system](https://developer.android.com/privacy-and-security/keystore), [iOS Keychain vs Android Keystore](https://medium.com/@musaddiq625/%EF%B8%8F-ios-keychain-vs-android-keystore-what-every-mobile-developer-should-know-2f677e4acac0), [Mobile Security: Weaknesses of Secure Elements and Enclaves](https://www.cryptomathic.com/blog/hidden-weaknesses-in-secure-elements-and-enclaves)

### 3.2 鍵ライフサイクル設計

```
┌─────────┐    ┌───────────┐    ┌──────────┐    ┌──────────┐
│  生成    │───→│  運用中    │───→│ ローテ   │───→│  失効    │
│ (端末内) │    │ (署名に   │    │ (新鍵    │    │ (旧鍵の  │
│          │    │  使用)    │    │  への    │    │  無効化) │
└─────────┘    └───────────┘    │  移行)   │    └──────────┘
                                └──────────┘
```

| フェーズ | 設計方針 | 備考 |
|---------|---------|------|
| **生成** | 端末内で生成。エクスポート不可が望ましい | Keystore/Secure Enclave の標準動作 |
| **公開鍵の登録** | サーバまたは公開鍵ディレクトリに登録 | `signer_key_id` で参照 |
| **ローテーション** | 旧鍵で「新公開鍵を承認する」署名付きメッセージを発行 | 鍵の連続性を証明 |
| **端末紛失・交換** | バックアップ鍵（紙/QRコード）または「信頼済みサーバ」経由で復旧 | 最大の運用課題 |
| **失効** | 公開鍵の失効リスト（CRL）またはサーバ側での無効化 | オフライン時は即時失効が困難 |

### 3.3 端末侵害への対策

IDEA-0003 のリスク #1「端末が乗っ取られると署名付きの嘘を作られる」への対策:

| 対策 | 効果 | 限界 |
|------|------|------|
| ハードウェア保護（TEE/SE） | root 化されていない端末では鍵抽出が困難 | root/jailbreak 環境では無力 |
| 署名タイムスタンプ | 侵害期間の特定に使える | 侵害が発覚するまでの嘘は防げない |
| 外部アンカー | 「その時点で存在した」事実を固定 | 内容の真偽は保証しない |
| 多重検証（複数端末の合意） | 単一端末の嘘に耐性 | 実装複雑度が高い |

**現実的な結論**: 端末侵害は「検出はできるが完全防止は不可能」。設計上は「署名付きの嘘は作れるが、後から検出できる仕組み」を目指すのが妥当。

---

## 4. 外部アンカー手段の比較

### 4.1 概要

外部アンカーの目的は、「この痕跡束が **その時点で存在していた**」ことを、サーバ運営者にも改ざんできない形で固定すること。

### 4.2 候補一覧

#### A. OpenTimestamps（Bitcoin OP_RETURN）

| 項目 | 内容 |
|------|------|
| 仕組み | 複数のハッシュを Merkle tree に集約し、ルートハッシュを Bitcoin トランザクションの OP_RETURN に埋め込む |
| コスト | **無料**（公開カレンダーサーバが集約。per-record コストはほぼゼロ） |
| 確認時間 | 即座にタイムスタンプ証明を取得（ただし Bitcoin ブロック確定まで数時間の「upgrade」が必要） |
| 信頼モデル | Bitcoin ネットワーク全体（分散・信頼最小化） |
| プライバシー | **ハッシュのみ**が外部に出る。元データは一切公開されない |
| 検証 | オフラインで検証可能（Bitcoin ブロックヘッダの履歴があれば十分） |
| 継続性 | Bitcoin が存続する限り有効。過去のタイムスタンプは 51% 攻撃でも覆せない |
| 実装 | `opentimestamps-client`（Python CLI）。Rust 実装も存在 |

**出典**: [OpenTimestamps](https://opentimestamps.org/), [OpenTimestamps Announcement - Peter Todd](https://petertodd.org/2016/opentimestamps-announcement), [OpenTimestamps - Wikipedia](https://en.wikipedia.org/wiki/OpenTimestamps)

#### B. RFC 3161 タイムスタンプ（TSA）

| 項目 | 内容 |
|------|------|
| 仕組み | 信頼された第三者（TSA）がハッシュにタイムスタンプ署名を付与 |
| コスト | 無料（FreeTSA）〜 有料（DigiCert, GlobalSign, Sectigo 等） |
| 確認時間 | 即時（秒単位） |
| 信頼モデル | TSA の信頼に依存（中央集権的）。eIDAS 認定 TSA は法的効力あり |
| プライバシー | ハッシュのみを送信 |
| 検証 | TSA 証明書チェーンの検証が必要。オフラインでも可能（証明書をキャッシュ） |
| 継続性 | TSA 事業者の継続に依存。事業者撤退リスクあり |
| 実装 | OpenSSL の `ts` コマンド、sigstore/timestamp-authority（OSS） |

**主要プロバイダ**:

| プロバイダ | 特徴 | eIDAS 認定 |
|-----------|------|-----------|
| DigiCert | 最大手 CA。HSM バック | あり |
| GlobalSign | REST API 対応。高スループット | あり |
| Sectigo | RSA 4096 + SHA-384 | あり |
| FreeTSA | 無料。SHA-1〜SHA-512 対応 | なし |
| sigstore/timestamp-authority | OSS。自前運用可能 | なし |

**出典**: [DigiCert TSA](https://knowledge.digicert.com/general-information/rfc3161-compliant-time-stamp-authority-server), [FreeTSA](https://freetsa.org/index_en.php), [GlobalSign Timestamp Service](https://www.globalsign.com/en/timestamp-service), [Trusted Timestamping in Digital Forensics - Metaspike](https://www.metaspike.com/trusted-timestamping-rfc-3161-digital-forensics/), [sigstore/timestamp-authority](https://github.com/sigstore/timestamp-authority)

#### C. 透明性ログ（Sigstore Rekor / Trillian）

| 項目 | 内容 |
|------|------|
| 仕組み | Append-only の Merkle tree ログ。包含証明（inclusion proof）と一貫性証明（consistency proof）を提供 |
| コスト | 無料（Sigstore 公開インスタンス）。自前運用も可能 |
| 確認時間 | 即時 |
| 信頼モデル | ログオペレータ＋モニタリングの組み合わせ。改ざん **検出可能**（tamper-evident）だが **防止不可能**（tamper-proof ではない） |
| プライバシー | 公開ログのため、エントリの内容は公開される（ハッシュのみにすれば OK） |
| 検証 | Merkle proof でオフライン検証可能 |
| 継続性 | Sigstore は OpenSSF/Linux Foundation が支援。Trillian は Google がメンテナンスモードに移行（後継: Tessera） |
| 実装 | Rekor REST API、Trillian gRPC API |

**出典**: [Sigstore Rekor](https://docs.sigstore.dev/logging/overview/), [Rekor GitHub](https://github.com/sigstore/rekor), [Google Trillian](https://github.com/google/trillian), [Transparency.dev](https://transparency.dev/)

#### D. Ethereum / 他のブロックチェーン

| 項目 | 内容 |
|------|------|
| 仕組み | トランザクションデータまたはイベントログにハッシュを記録 |
| コスト | ガス代（変動大。$0.10〜$10+） |
| 確認時間 | ~12秒（ブロック時間） |
| 信頼モデル | Ethereum ネットワーク全体 |
| 継続性 | Ethereum の存続に依存 |

Bitcoin（OpenTimestamps）と比較して特段の優位性がなく、コストが高いため **非推奨**。

### 4.3 比較表

| 基準 | OpenTimestamps (Bitcoin) | RFC 3161 TSA | Sigstore Rekor | Ethereum |
|------|------------------------|-------------|----------------|----------|
| **コスト** | 無料 | 無料〜有料 | 無料 | ガス代（変動） |
| **確認時間** | 数時間（upgrade） | 即時 | 即時 | ~12秒 |
| **信頼分散度** | 高（分散） | 低（中央集権） | 中（要モニタ） | 高（分散） |
| **法的効力** | 証拠能力あり | eIDAS 認定あり | 未確立 | 未確立 |
| **オフライン検証** | 可 | 可（証明書要） | 可（proof要） | 可 |
| **プライバシー** | ハッシュのみ | ハッシュのみ | 公開ログ | トランザクション公開 |
| **継続性リスク** | 低（Bitcoin依存） | 中（事業者依存） | 中（OSS/財団依存） | 低（Ethereum依存） |
| **実装難易度** | 低 | 低 | 中 | 中 |
| **本プロジェクト適合度** | **最高** | **高** | 中 | 低 |

### 4.4 推奨戦略: 多層アンカー

```
┌──────────────────────────────────────────────┐
│  L1: Share Envelope (hash chain + 署名)       │  ← コア（オフライン必須）
│  [常に有効。外部アンカーなしで検証可能]        │
└──────────────────────┬───────────────────────┘
                       │ オンライン時のみ（任意）
         ┌─────────────┼─────────────┐
         ▼             ▼             ▼
   ┌───────────┐ ┌──────────┐ ┌───────────┐
   │OpenTime-  │ │RFC 3161  │ │Sigstore   │
   │stamps     │ │TSA       │ │Rekor      │
   │(Bitcoin)  │ │          │ │           │
   └───────────┘ └──────────┘ └───────────┘
   1st priority   2nd priority  将来検討
```

**推奨**:
1. **第一優先: OpenTimestamps** — 無料、信頼分散、ハッシュのみ公開。Merkle root のバッチアンカリングでスケーラブル
2. **第二優先: RFC 3161 TSA** — 即時性が必要な場合や、法的効力（eIDAS）が求められる場合の補完
3. **将来検討: Sigstore Rekor** — ソフトウェア署名エコシステムとの親和性が出た場合

---

## 5. 第三者検証手順の設計

### 5.1 オフライン検証（コア）

envelope 単体で実行可能な検証:

```
1. payload_events を canonical(event) に変換
2. hash chain を再計算し、chain_tail と一致するか検証
3. chain_tail + envelope_header を再構成し、署名を検証
   → signer_key_id で公開鍵を取得し、signature を検証
```

**成功条件**: 改ざん・欠落があれば chain_tail が一致しない。署名偽造は秘密鍵なしに不可能。

### 5.2 外部アンカー検証（オプション）

```
4. envelope の chain_tail（または Merkle root）に対する
   外部アンカー証明を検証:
   - OpenTimestamps: .ots ファイルを解析し、Bitcoin ブロックヘッダまで辿る
   - TSA: タイムスタンプトークンの証明書チェーンを検証
```

**成功条件**: 「この chain_tail がその時点で存在していた」ことが外部証拠で裏付けられる。

### 5.3 検証パッケージの構成

第三者検証を再現可能にするため、以下を同梱して渡す:

```json
{
  "envelope": { ... },
  "verification_bundle": {
    "signer_public_key": "base64...",
    "anchor_proofs": [
      {
        "type": "opentimestamps",
        "proof": "base64(.ots file)...",
        "anchored_hash": "chain_tail or merkle_root"
      },
      {
        "type": "rfc3161",
        "token": "base64(TSA response)...",
        "tsa_cert_chain": ["base64...", "..."]
      }
    ]
  }
}
```

---

## 6. 先行事例・類似プロジェクト

### 6.1 災害・人道支援での先行事例

| プロジェクト | 概要 | 関連性 |
|-------------|------|--------|
| **ウクライナ戦場写真のタイムスタンプ** | OpenTimestamps を使い、写真が「少なくともX時点には存在した」証拠を記録 | 直接的な先行事例。痕跡データの存在証明と同じ構造 |
| **IoTセンサデータの真正性証明** (IEEE R10 HTC 2025) | ブロックチェーンベースでセンサデータの来歴と整合性を検証。データ欠損にも対応 | 災害時の不安定なネットワーク下でのデータ整合性確保に直接関連 |
| **人道支援のブロックチェーン活用** (SSRN, 2024) | データ管理、ID 検証、サプライチェーンでの改ざん耐性 | 災害対応における信頼性確保の一般論 |

**出典**: [Proof of Authenticity of General IoT Information](https://arxiv.org/pdf/2512.18560), [Blockchain Technology in Humanitarian Aid - SSRN](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=4961600), [Blockchain for humanitarian action - Journal of International Humanitarian Action](https://jhumanitarianaction.springeropen.com/articles/10.1186/s41018-018-0044-5)

### 6.2 監査ログの改ざん検出

| 手法 | 概要 | 関連性 |
|------|------|--------|
| **Hash chain ベースの監査ログ** (VLDB 2004) | 各ログエントリに hash chain を適用し、内部者による改ざんを検出 | 本プロジェクトの hash chain 設計の理論的基盤 |
| **Efficient Data Structures for Tamper-Evident Logging** (USENIX Security 2009) | Merkle tree ベースのログ構造。800MB の証跡が 3KB の proof に圧縮可能 | 大量の痕跡データに対する効率的な検証証明 |
| **Accountability of Things** (2023) | IoT デバイスが hash chain をローカル生成し、バッチで検証サーバに送信 | **オフライン生成→オンライン検証** のパターンが本プロジェクトと一致 |

**出典**: [Tamper Detection in Audit Logs - VLDB 2004](https://www.vldb.org/conf/2004/RS13P1.PDF), [Efficient Data Structures for Tamper-Evident Logging - USENIX](https://static.usenix.org/event/sec09/tech/full_papers/crosby.pdf), [Accountability of Things](https://arxiv.org/pdf/2308.05557), [Audit logs security - Cossack Labs](https://www.cossacklabs.com/blog/audit-logs-security/)

### 6.3 学んだ教訓

1. **ブロックチェーンは手段であって目的ではない**: 人道支援での実証研究の多くは「パイロット段階で止まっている」。インフラが不安定な環境では分散台帳の限界がある。オフライン完結を優先する本プロジェクトの設計方針は正しい
2. **バッチ送信パターンが現実的**: IoT/モバイルの先行事例は「ローカルで hash chain を生成し、接続回復時にバッチで外部検証」というパターンに収束している
3. **「改ざん防止」ではなく「改ざん検出」を目標とする**: 透明性ログの設計思想（tamper-evident, not tamper-proof）は本プロジェクトの現実的な目標と一致する

---

## 7. プライバシーと法的考慮

### 7.1 外部アンカーとプライバシー

| 方式 | 外部に出る情報 | プライバシーリスク |
|------|--------------|-------------------|
| OpenTimestamps | SHA-256 ハッシュのみ | **最小**。ハッシュから元データの復元は不可能 |
| RFC 3161 TSA | SHA-256 ハッシュのみ | **最小**。TSA はハッシュの意味を知らない |
| Sigstore Rekor | エントリ内容が公開ログに載る | **要注意**。ハッシュのみを記録する設計が必要 |
| Bitcoin/Ethereum 直接 | トランザクションデータが公開 | **最小**（OP_RETURN にはハッシュのみ） |

### 7.2 GDPR / 個人情報保護法との関係

- **ハッシュは擬名化（pseudonymization）であり匿名化ではない**: GDPR 上、ハッシュ化されたデータは依然として個人データとみなされる可能性がある（Article 29 Working Party, Opinion 05/2014）
- **ただし外部アンカーに載せるのは Merkle root のみ**: 個々の痕跡データのハッシュではなく、集約された root hash であるため、元データとの対応付けは実質的に不可能
- **データ最小化原則に適合**: GDPR の「必要最小限の情報のみを処理する」原則に、ハッシュのみの外部公開は合致する

**出典**: [EDPS: Hash Function as Pseudonymisation](https://edps.europa.eu/sites/edp/files/publication/19-10-30_aepd-edps_paper_hash_final_en.pdf), [The Role of Hashing in GDPR Compliance](https://www.hash.tools/115/cryptographic-algorithms/1999/the-role-of-hashing-in-gdpr-compliance), [Compliance Essentials: Why Hashed Data Isn't Anonymous](https://pandectes.io/blog/compliance-essentials-why-hashed-data-isnt-anonymous/)

### 7.3 日本の法的環境

- **電子署名法**（2001年施行）: 電子署名は「本人による署名」として法的効力を持つ。ただし「認定認証事業者」による証明書が前提
- **タイムスタンプ**: 日本では「認定タイムスタンプ」（e-Government 対応）が法的に認められている。Bitcoin ベースのタイムスタンプの法的位置づけは明確ではないが、「証拠」としての補助的な価値はある
- **個人情報保護法**: ハッシュのみの外部公開は「個人情報の第三者提供」には該当しにくいが、元データとの対応表を保持する場合は注意が必要

---

## 8. PoC 計画（IDEA-0003 の PoC を具体化）

### 8.1 PoC の目的

「ブロックチェーン抜き」で改ざん検知と第三者検証が成立するかを先に確定し、その後に外部アンカーの付加価値を検証する。

### 8.2 段階的 PoC

#### Phase 1: コア検証（オフライン完結）

| ステップ | 内容 | 成功判定 |
|---------|------|---------|
| 1-1 | 痕跡レコード列に対して hash chain を構成（SHA-256, `prev_hash` 含む） | chain_tail が計算可能 |
| 1-2 | Ed25519 鍵ペアを生成し、chain_tail に署名 | 署名・検証が成功 |
| 1-3 | 任意の 1 レコードを改変/削除し、検証で検出できることを確認 | 改変を 100% 検出 |
| 1-4 | envelope を別環境にコピーし、同じ検証結果が出ることを確認 | 第三者環境で検証再現 |

**実装**: Rust (`core/` crate) + CLI。IF-IMPL-001〜003 に準拠。

#### Phase 2: 外部アンカー検証（オンライン任意）

| ステップ | 内容 | 成功判定 |
|---------|------|---------|
| 2-1 | 複数 envelope の chain_tail から Merkle tree を構成し、root hash を算出 | root hash が算出可能 |
| 2-2 | OpenTimestamps で root hash をアンカー | .ots ファイルが取得可能 |
| 2-3 | .ots ファイルを upgrade し、Bitcoin ブロックとの紐付けを確認 | Bitcoin ブロックヘッダとの照合が成功 |
| 2-4 | アンカーなしの envelope と、アンカーありの envelope の検証フローを比較 | アンカーなしでもコア検証は成立。アンカーは追加の時刻証明 |

### 8.3 必要リソース

- Rust 開発環境（既存の `core/` crate）
- Ed25519 署名: `ed25519-dalek` クレート
- SHA-256: `sha2` クレート
- OpenTimestamps: Python CLI（`ots` コマンド）または Rust バインディング
- テスト端末: 1台（CLI で十分）

---

## 9. リスクと緩和策のまとめ

| リスク | 深刻度 | 緩和策 |
|--------|--------|--------|
| 端末侵害で「署名付きの嘘」が作られる | 高 | TEE/SE で鍵保護 + 外部アンカーで時刻固定 + 異常検知 |
| 鍵紛失・端末交換で検証不能になる | 高 | 鍵ローテーション手順 + バックアップ鍵 + 公開鍵の事前登録 |
| 外部アンカー依存先の停止 | 中 | 多層アンカー（OpenTimestamps + TSA）。コアはアンカーなしで成立 |
| 外部アンカーのコスト増大 | 低 | OpenTimestamps は無料。TSA も FreeTSA で無料枠あり |
| hash chain の性能ボトルネック | 低 | SHA-256 は十分高速。数万件/秒は問題なし |
| iOS Secure Enclave の Ed25519 非対応 | 中 | マルチアルゴリズム対応（iOS は P-256、Android は Ed25519）または Keychain フォールバック |

---

## 10. 結論と推奨

### 10.1 成立性の評価

| 成立条件（IDEA-0003 §4） | 評価 | 根拠 |
|--------------------------|------|------|
| 改ざん検知 | **成立** | hash chain + 署名で 1bit の改変・欠落も検出可能 |
| オフライン成立 | **成立** | コア設計は外部アンカー不要。端末内で完結 |
| 共有成立（第三者検証） | **成立** | verification_bundle を同梱すれば再現可能 |
| プライバシー | **成立** | Merkle root（ハッシュのみ）を外部に出す設計で最小開示 |
| 運用可能性 | **条件付き成立** | 鍵管理の設計次第。端末紛失・移行の手順が鍵 |

### 10.2 推奨する次の一手

1. **Phase 1 PoC を実施**: L1 の hash chain + Ed25519 署名を Rust CLI で実装し、改ざん検出と第三者検証の再現性を確認する
2. **鍵管理の設計文書を作成**: 鍵生成・ローテーション・紛失対応の具体的な手順を mini RFC として起案する
3. **Phase 2 は Phase 1 成功後**: OpenTimestamps 統合は「任意拡張」として Phase 1 の上に積む
4. **iOS の署名アルゴリズムは別途決定**: Secure Enclave (P-256) vs Keychain (Ed25519) のトレードオフを mini RFC で扱う
