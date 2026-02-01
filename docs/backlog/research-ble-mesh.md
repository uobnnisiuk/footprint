# 調査レポート: BLEメッシュ（またはメッシュ的ストア＆フォワード）で通信断を越える

**対象アイデア**: IDEA-0002: 「BLEメッシュ（またはメッシュ的ストア＆フォワード）で通信断を越える」
**調査日**: 2026-02-01
**調査者**: Claude (AI Assistant)

---

## エグゼクティブサマリー

BLEを活用した端末間メッシュ・ストア＆フォワード通信により、通信インフラ断絶時の情報伝達を実現するアイデアについて、技術仕様、実装制約、既存事例、セキュリティ、実現可能性を調査した。

**主な結論**:
- ⚠️ **BLE Mesh規格の採用は困難**: iOS APIの制約によりADVベアラが使用不可、GATT Proxy必須
- ✅ **ストア＆フォワード（DTN）は実現可能**: BLE Mesh規格に依存せず、独自S&F実装が現実的
- ⚠️ **iOS制約が最大の壁**: バックグラウンド広告が独自形式、スキャン頻度制限、接続数制限（約8）
- ✅ **データ転送性能は十分**: 1回の接触（数秒）で数KB〜数十KBの転送が可能
- ❌ **既存実装に深刻な脆弱性**: Bridgefyは追跡・改ざん・DoS攻撃に脆弱（2020-2021年の監査）
- ⚠️ **電池消費は設計次第**: スキャン頻度を適切に制御すれば5%以下に抑制可能だが、常時スキャンは現実的でない
- ✅ **災害時の実績あり**: goTennaがプエルトリコで23マイルのメッシュ通信を実証

**推奨**: Bluetooth SIG BLE Meshは避け、**Android優先の独自S&F実装**でPoCを実施。iOS対応は後回しにするか、フォアグラウンド限定として割り切る。

---

## 1. BLE Mesh技術仕様

### 1.1 Bluetooth SIG BLE Mesh概要

Bluetooth SIG（Special Interest Group）が策定したBluetooth Low Energy向けメッシュネットワーク規格。

| 項目 | 仕様 |
|------|------|
| **策定時期** | 2017年7月公開（Mesh Profile 1.0） |
| **最新バージョン** | Mesh Protocol 1.1（2023年9月に機能強化） |
| **ネットワーク型** | マネージドフラッディング（Managed Flooding） |
| **最大ノード数** | 理論上32,767ノード |
| **メッセージ配送** | Publish/Subscribeモデル |
| **セキュリティ** | ネットワーク層暗号化 + アプリケーション層暗号化 |

**出典**: [Bluetooth mesh networking - Wikipedia](https://en.wikipedia.org/wiki/Bluetooth_mesh_networking), [Bluetooth Mesh Networking: The Ultimate Guide](https://novelbits.io/bluetooth-mesh-networking-the-ultimate-guide/)

### 1.2 2023年の主要機能強化

Bluetooth SIGは2023年9月に以下の機能を追加：

1. **Directed Forwarding（方向制御転送）**
   - ソースから宛先への直接経路を確立
   - 不要なメッセージフラッディングを削減し、消費電力を最小化

2. **Remote Provisioning（リモートプロビジョニング）**
   - デバイス登録時に物理的近接が不要
   - 複数ホップ経由でのプロビジョニングが可能

3. **Certificate-Based Provisioning（証明書ベースプロビジョニング）**
   - PKI（公開鍵基盤）を利用したデバイス認証
   - X.509形式のデジタル証明書を使用

4. **Private Beacons（プライベートビーコン）**
   - ビーコンデータをPrivateBeaconKeyと乱数で暗号化
   - 静的情報の露出を防止

**出典**: [Bluetooth® Mesh Feature Enhancements Summary](https://www.bluetooth.com/mesh-feature-enhancements-summary/), [What's New with the Latest Bluetooth Mesh Specification - Silicon Labs](https://www.silabs.com/blog/whats-new-with-the-latest-bluetooth-mesh-specification)

### 1.3 BLE Meshの通信ベアラ

BLE Meshは以下の2つの通信ベアラ（伝送路）を使用：

1. **ADVベアラ（Advertising Bearer）**
   - BLE広告パケットを使用
   - 全メッシュノードがこれを使用
   - **iOS API制約により実装不可**

2. **GATT Bearer（GATT Proxy）**
   - BLE GATT接続経由でメッシュメッセージを転送
   - スマートフォンなど、ADVベアラが使えないデバイス向け
   - **Proxy機能を持つ中継ノードが必須**

**出典**: [GitHub - NordicSemiconductor/IOS-nRF-Mesh-Library](https://github.com/NordicSemiconductor/IOS-nRF-Mesh-Library)

---

## 2. ストア＆フォワード（DTN）の基礎

### 2.1 遅延耐性ネットワーク（DTN）とは

DTN（Delay/Disruption Tolerant Networking）は、エンドツーエンドの経路が確立できない環境で、メッセージを中継しながら徐々に目的地へ届ける技術。

**基本原理**:
- **Store（保存）**: メッセージを端末のストレージに一時保存
- **Forward（転送）**: 他の端末と接触した際にメッセージを転送
- **Bundle Protocol**: メッセージ（バンドル）を管理するプロトコル

**出典**: [Delay/Disruption Tolerant Networking - NASA](https://www.nasa.gov/communicating-with-missions/delay-disruption-tolerant-networking/), [Delay-tolerant networking - Wikipedia](https://en.wikipedia.org/wiki/Delay-tolerant_networking)

### 2.2 DTNの特性

| 項目 | 説明 |
|------|------|
| **適用環境** | 宇宙通信、遠隔地、災害時、移動体ネットワーク |
| **遅延許容** | 数分〜数時間以上の遅延を許容 |
| **断続的接続** | 継続的な接続は不要、接触時のみ通信 |
| **ヘテロジニアス** | 異なるネットワーク技術を統合可能 |
| **信頼性** | 送達保証（必要に応じてACK/再送） |

**NASAの実績**:
- 2024年、NASAのPACEミッションがDTNを運用開始
- 3,400万バンドル以上を100%成功率で送信

**出典**: [All about Delay-Tolerant Networking (DTN) Contributions to Future Internet](https://www.mdpi.com/1999-5903/16/4/129), [Delay- and Disruption-Tolerant Networks (DTNs) A Tutorial](https://www.nasa.gov/wp-content/uploads/2023/09/dtn-tutorial-v3.2-0.pdf)

### 2.3 DTNのルーティングプロトコル

#### 2.3.1 Epidemic Routing（エピデミック・ルーティング）

**特徴**:
- フラッディング（flooding）方式
- 遭遇したすべての端末にメッセージをコピー
- 配送率と遅延のベンチマーク（上限）

**利点**:
- ✅ 最も高い配送率と最小遅延
- ✅ 実装が単純

**欠点**:
- ❌ メッセージ複製が膨大（リソース消費大）
- ❌ ストレージとネットワーク帯域を圧迫

**出典**: [Routing in delay-tolerant networking - Wikipedia](https://en.wikipedia.org/wiki/Routing_in_delay-tolerant_networking), [Epidemic Routing for Partially-Connected Ad Hoc Networks](https://www.researchgate.net/publication/2633330_Epidemic_Routing_for_Partially-Connected_Ad_Hoc_Networks)

#### 2.3.2 PRoPHET（Probabilistic Routing Protocol using History of Encounters and Transitivity）

**特徴**:
- 過去の遭遇履歴に基づいて配送確率を計算
- 確率が高い端末にのみメッセージを転送
- Epidemicより効率的

**出典**: [An Improved PRoPHET Routing Protocol in Delay Tolerant Network](https://pmc.ncbi.nlm.nih.gov/articles/PMC4306380/)

### 2.4 災害通信への応用

DTNは災害復旧・緊急対応ネットワークに適用可能：
- インフラ破壊または不可用な環境での通信
- 疎結合ネットワーク（大きなリンク遅延、低リンク可用性に耐性）
- アプリケーション層実装により、異種サブアーキテクチャを統合

**出典**: [Disaster Management through Delay Tolerant Networks - NADIA](http://article.nadiapub.com/IJFGCN/vol10_no3/7.pdf)

---

## 3. iOS/Android実装の制約

### 3.1 iOSのバックグラウンドBLE制約

#### 3.1.1 スキャン制約

| 項目 | フォアグラウンド | バックグラウンド |
|------|------------------|------------------|
| **スキャン頻度** | 連続的 | 数秒に1回（大幅に間引き） |
| **スキャンタイプ** | Active（追加情報取得可） | Passive（広告のみ） |
| **サービスフィルタ** | 任意 | 必須（指定UUIDのみ検出） |
| **重複通知** | 可能（`AllowDuplicates`） | 不可（1デバイス1イベントに統合） |

**出典**: [Core Bluetooth Background Processing for iOS Apps](https://developer.apple.com/library/archive/documentation/NetworkingInternetWeb/Conceptual/CoreBluetooth_concepts/CoreBluetoothBackgroundProcessingForIOSApps/PerformingTasksWhileYourAppIsInTheBackground.html), [iOS BLE Scanning: Building Reliable Core Bluetooth Discovery](https://punchthrough.com/ios-ble-scanning-guide/)

#### 3.1.2 広告制約

**バックグラウンド広告の制限**:
- iOSアプリがバックグラウンドで広告すると、**独自形式**で送信される
- この形式はBluetooth標準外で、**非iOS端末では読み取り不可**
- すべてのサービスUUIDは「オーバーフロー領域」に配置される
- オーバーフロー領域のスキャンは、**受信側デバイスの画面が点灯している場合のみ**動作

**出典**: [Hacking The Overflow Area](https://davidgyoungtech.com/2020/05/07/hacking-the-overflow-area), [iOS advertisements in the background - Crownstone](https://crownstone.rocks/2018/06/27/ios-advertisements-in-the-background)

#### 3.1.3 接続数制限

- iOS/Androidデバイスは**最大約8のBLE接続**に制限される
- これには他アプリやOSの接続も含まれる

**出典**: [KBA_BT_0504: Bluetooth Mesh Mobile - Silicon Labs](https://www.silabs.com/community/wireless/bluetooth/knowledge-base.entry.html/2020/11/12/kba_bt_0504_bluetoothmeshmobile-reductionof-EhIZ)

### 3.2 BLE Mesh実装に対するiOS制約の影響

**Nordic Semiconductor iOSライブラリの制約**:
- iOSでは**ADVベアラの実装が不可能**（API制限）
- **GATT Proxy必須**: iOS端末はGATT Proxyプロトコル経由でのみメッシュ通信可能
- **Proxy機能を持つノードが必須**: iOS端末だけではメッシュネットワークを構成できない

**出典**: [GitHub - NordicSemiconductor/IOS-nRF-Mesh-Library](https://github.com/NordicSemiconductor/IOS-nRF-Mesh-Library), [AN1200.1: iOS and Android ADK for Bluetooth® mesh SDK](https://www.silabs.com/documents/public/application-notes/an1200-1-bluetooth-mesh-2x-for-android-and-ios-adk.pdf)

### 3.3 Androidの実装優位性

Androidでは以下が可能：
- ✅ バックグラウンドでの広告（標準BLE形式）
- ✅ より頻繁なスキャン（最小7.5msの接続間隔）
- ✅ ADVベアラの実装（ただしバックグラウンド制約はあり）

**出典**: [GitHub - NordicSemiconductor/Android-nRF-Mesh-Library](https://github.com/NordicSemiconductor/Android-nRF-Mesh-Library)

---

## 4. データ転送性能

### 4.1 BLE GATTスループット

| 項目 | BLE 4.0/4.1 | BLE 4.2/5.0 |
|------|-------------|-------------|
| **最大ATTペイロード** | 20バイト | 244バイト（DLE使用時） |
| **接続間隔（Android）** | 7.5ms〜4000ms | 7.5ms〜4000ms |
| **接続間隔（iOS）** | 15ms〜4000ms | 15ms〜4000ms |
| **理論スループット** | 約1.2kB/s | 約97.6kB/s（244バイトPDU） |
| **実測スループット（Android）** | - | 16kB/s（接続間隔7.5ms、6パケット/間隔） |
| **実測スループット（iOS）** | - | 2.7kB/s（30ms間隔）〜7.1kB/s（11.25ms間隔） |

**出典**: [Maximizing BLE Throughput Part 4 - Punch Through](https://punchthrough.com/ble-throughput-part-4/), [Bluetooth 5 speed: How to achieve maximum throughput](https://novelbits.io/bluetooth-5-speed-maximum-throughput/)

### 4.2 転送時間の見積もり

**すれ違い時間を3秒と仮定**:

| データサイズ | BLE 4.2（Android、最適化） | BLE 4.2（iOS、標準） |
|--------------|----------------------------|----------------------|
| 1KB | 0.06秒 | 0.37秒 |
| 10KB | 0.63秒 | 3.7秒 ❌ |
| 100KB | 6.25秒 ❌ | 37秒 ❌ |

**結論**:
- ✅ **数KB程度のメッセージなら3秒で転送可能**（Android最適化時）
- ⚠️ iOS標準設定では10KB以上は困難
- ❌ 数十KB以上の転送は長時間接続が必要（すれ違いでは不可）

**出典**: [A Practical Guide to BLE Throughput - Interrupt](https://interrupt.memfault.com/blog/ble-throughput-primer)

---

## 5. 既存実装事例

### 5.1 FireChat（2014年香港デモで使用）

**概要**:
- Open Gardenが開発したメッシュメッセージングアプリ
- 2014年香港「雨傘革命」で約50万人が使用

**技術仕様**:
- BluetoothまたはWi-Fi経由でメッシュネットワークを構築
- インターネット不要の独自ネットワーク
- **範囲**: Bluetooth/Wi-Fiで約10m

**セキュリティ問題**:
- ❌ メッセージが平文で送信
- ❌ チャットルームが公開（当局も参加可能）

**現状**: サービス終了

**出典**: [FireChat in Hong Kong: How an app tapped its way into the protests - CNN](https://www.cnn.com/2014/10/16/tech/mobile/tomorrow-transformed-firechat/index.html), [How the Hong Kong Protests Forced Firechat to Evolve - Vice](https://www.vice.com/en/article/what-firechat-learned-in-hong-kong/)

### 5.2 Bridgefy（2019年香港デモ、2020年重大脆弱性発覚）

**概要**:
- メキシコのスタートアップが開発
- 2019年香港デモでFireChatの後継として使用
- Bluetoothメッシュネットワーク

**2020年8月の脆弱性発覚（Royal Holloway大学の監査）**:

| 脆弱性 | 内容 |
|--------|------|
| **ユーザ追跡** | 物理的に近くにいれば、どのIDが誰と通信しているかの社交グラフを構築可能 |
| **位置追跡** | 個々のユーザの移動を群衆内で追跡可能（メッセージ遅延からトポロジーを推測） |
| **暗号化脆弱性** | Bleichenbacher攻撃により約217個の選択暗号文で機密性を破壊可能 |
| **認証不在** | 暗号学的認証機構が存在しない |
| **DoS攻撃** | "Zip爆弾"で全ネットワークをクラッシュ可能（10KB→10MB解凍でアプリ強制終了） |

**2021年の再監査結果**:
- Bridgefyは2020年の指摘を受けてSignalプロトコルを採用したと主張
- しかし、**以前の脆弱性が修正されていない**ことが判明

**出典**: [Bridgefy FAIL: Insecure for Use in Protests](https://securityboulevard.com/2020/08/bridgefy-fail-insecure-for-use-in-protests/), [Breaking Bridgefy, again](https://eprint.iacr.org/2021/214.pdf), [Breaking Bridgefy, again: Adopting libsignal is not enough - USENIX](https://www.usenix.org/conference/usenixsecurity22/presentation/albrecht)

### 5.3 Briar（オープンソース、セキュア）

**概要**:
- 活動家・ジャーナリスト向けの安全なメッセージングアプリ
- 中央サーバ不要、P2P通信

**技術仕様**:
- **接続経路**: Bluetooth、Wi-Fi、Tor、USBメモリ
- **暗号化**: エンドツーエンド暗号化（すべての通信）
- **範囲**: Bluetooth/Wi-Fiで約10m
- **セキュリティ監査**: 独立監査を通過

**制約**:
- ⚠️ 範囲が10m程度と短い（都市全体や大規模建物には不十分）
- ⚠️ Android専用（iOS版は開発中だが未リリース）

**実用例**:
- Wi-Fiホットスポット（インターネット接続なし）経由で通信可能
- オンライン時はTor経由で匿名通信

**出典**: [How it works - Briar](https://briarproject.org/how-it-works/), [Chatting offline: an overview of mesh messaging apps - Kaspersky](https://www.kaspersky.com/blog/mesh-messengers/54192/)

### 5.4 goTenna（プエルトリコで実証済み）

**概要**:
- 専用ハードウェアを使用したメッシュネットワーク
- **技術**: BLEではなく、VHF帯（151-154MHz）の長距離無線

**プエルトリコ・ハリケーンマリア後の展開（2017年）**:
- 通信インフラの93%が破壊された状況で展開
- クラウドファンディングで資金調達、ソーラーパネル付き中継ノードを設置
- **実測結果**: 単一ホップで23.1マイル（約37km）の通信成功
- すべてのメッセージが**データ損失なし**で送信・受信成功

**出典**: [LCG Holdings Tests goTenna Pro in Puerto Rico and Achieves 23.1 Mile Hop](https://gotennapro.com/blogs/case-studies/lcg-holdings-tests-gotenna-pro-in-puerto-rico-and-achieves-23-1-mile-single-hop-range), [A mesh network spontaneously erupts in the US and helps connect Puerto Rico - TechCrunch](https://techcrunch.com/2017/11/14/a-mesh-network-spontaneously-erupts-in-the-us-and-helps-connect-puerto-rico/)

**注**: goTennaはBLE技術ではないが、災害時メッシュネットワークの成功事例として重要

---

## 6. セキュリティとプライバシーの課題

### 6.1 主要な攻撃ベクトル

#### 6.1.1 フラッディング攻撃（Flooding Attack）

**概要**:
- 悪意のあるノードが大量の偽メッセージを生成
- ネットワーク帯域、ストレージ、電池を消費させる

**影響**:
- ネットワーク全体の機能停止
- 正規メッセージの配送遅延・失敗
- 全端末の電池枯渇

**出典**: [Mitigation on Flood Attacks in DTN's - IJERT](https://www.ijert.org/mitigation-on-flood-attacks-in-dtns)

#### 6.1.2 署名フラッディング（Signature Flooding）

**概要**:
- 正規の署名付きメッセージを大量に送信
- 各ノードが署名検証でCPU・電力を消費
- DoS攻撃の一種

**出典**: [Flooding-Resilient Broadcast Authentication for VANETs](https://netsec.ethz.ch/publications/papers/hsiao_mobicom11.pdf)

#### 6.1.3 追跡・プライバシー侵害

**概要**:
- 固定IDの使用により、ユーザの移動を追跡可能
- メッセージの発信者・受信者の社交グラフを構築可能

**Bridgefyの事例**:
- 物理的に近くにいれば、すべての通信関係を監視可能

**出典**: [Breaking Bridgefy](https://martinralbrecht.wordpress.com/wp-content/uploads/2020/08/bridgefy-abridged.pdf)

### 6.2 緩和策

#### 6.2.1 レート制限（Rate Limiting）

**概要**:
- 各ノードが一定時間内に送信できるメッセージ数を制限
- パケット生成数とレプリカ生成数の両方を制限

**課題**:
- ⚠️ 正規のバースト通信（短期間の大量送信需要）を阻害

**出典**: [Mitigation on Flood Attacks in DTN's - IJERT](https://www.ijert.org/mitigation-on-flood-attacks-in-dtns), [Detecting Flooding Attack and Accommodating Burst Traffic in DTN - IEEE](https://ieeexplore.ieee.org/document/8024026/)

#### 6.2.2 軽量認証（DTN-Cookies）

**概要**:
- 従来のRSA署名より軽量な認証子（Cookie）を使用
- イングレスフィルタリング、テーブル検索、Cookie計算・検証のエネルギーは無視可能

**性能**:
- 平均エネルギー消費: 39,465mJ（ゲートウェイのエネルギー予備の0.001%）
- RSA-1024署名より低消費電力

**出典**: [A Security Scheme to Mitigate Denial of Service Attacks in Delay Tolerant Networks](https://pubs.sciepub.com/jcsa/5/2/2/index.html)

#### 6.2.3 Claim-Carry-and-Check

**概要**:
- 各ノードが自身の送信数をカウント（Claim）
- 他ノードがこの主張を検証（Check）
- 矛盾を検出してレート制限違反を発見

**出典**: [Mitigation on Flood Attacks in DTN's - IJERT](https://www.ijert.org/research/mitigation-on-flood-attacks-in-dtns-IJERTCONV2IS15031.pdf)

#### 6.2.4 FDER（Flooding Detection based on Encounter Records）

**概要**:
- 遭遇記録に基づいてフラッディング攻撃を検出
- 正規のバースト通信を許容しつつ、攻撃を検知

**出典**: [Detecting flooding attack while accommodating burst traffic in delay tolerant networks - IEEE](https://ieeexplore.ieee.org/document/7943536/)

#### 6.2.5 バッチ署名検証

**概要**:
- 複数パケットの署名を集約して一括検証
- 計算コストを削減

**出典**: [Flooding-Resilient Broadcast Authentication for VANETs](https://netsec.ethz.ch/publications/papers/hsiao_mobicom11.pdf)

### 6.3 プライバシー保護

**必須対策**:
- ✅ **IDローテーション**: 定期的に端末識別子を変更
- ✅ **エンドツーエンド暗号化**: メッセージ内容の保護
- ✅ **匿名化**: 発信者・受信者情報の保護
- ✅ **最小限の情報露出**: 中継ノードには内容を見せない

---

## 7. 電池消費と実用性

### 7.1 COVID-19接触通知アプリの事例

**Google/Apple Exposure Notification API**:
- BLEを使用した常時バックグラウンド動作
- 設計目標: **電池消費5%以下**（通常使用時）

**実測値**:
- ✅ 通常使用時: 5%以下（設計通り）
- ⚠️ アイドル時: より高い割合を占める（絶対値は低い）
- ❌ 初期実装の問題: 一部で60-100%の電池消費を報告

**出典**: [Will the NHS COVID-19 app drain my phone battery?](https://faq.covid19.nhs.uk/article/KA-01163/en-us), [COVID-19 and Your Smartphone: BLE-Based Smart Contact Tracing](https://pmc.ncbi.nlm.nih.gov/articles/PMC8843047/)

### 7.2 スキャン頻度と電池消費のトレードオフ

| スキャン頻度 | 検出遅延 | 電池消費 | 適用シナリオ |
|-------------|----------|----------|--------------|
| **常時スキャン** | 最小（数秒） | 高（10-20%/日） | 非現実的 |
| **1分に1回** | 中程度（1分） | 中（5-10%/日） | 災害時の短期運用 |
| **5分に1回** | 高（5分） | 低（2-5%/日） | 平時の長期運用 |
| **イベント駆動** | 可変 | 最小 | 理想的（実装困難） |

**推奨設定**:
- **平時**: 5分に1回スキャン（長期運用可能）
- **災害モード**: 1分に1回スキャン（数日間の運用）
- **緊急時**: 常時スキャン（数時間の運用）

### 7.3 実用性の評価

**メリット**:
- ✅ 適切な設計で、数日〜数週間の運用が可能
- ✅ ユーザ操作は最小限（アプリ起動のみ）
- ✅ スマホが普及しているため追加ハードウェア不要

**課題**:
- ⚠️ バックグラウンド動作がOSに依存（特にiOS）
- ⚠️ 電池残量が少ない場合、機能停止のリスク
- ⚠️ ユーザがアプリを強制終了すると動作しない

---

## 8. PoC（概念実証）推奨計画

### 8.1 目的

1. **端末間のBLE S&F転送が成立するか**を確認
2. **すれ違い時間（数秒）で転送可能なデータサイズ**を測定
3. **リレー（A→B→C）が動作するか**を検証
4. **電池消費が現実的か**を評価

### 8.2 準備物

| カテゴリ | 内容 |
|----------|------|
| **端末** | Android 3台（iOS対応は後回し） |
| **開発環境** | Android Studio、BLE開発経験 |
| **テスト場所** | 屋内（オフィス）、屋外（広場） |
| **測定ツール** | 電池監視アプリ、BLEスキャナ（nRF Connect等） |

### 8.3 実験手順

#### フェーズ1: 基本的なBLE広告・スキャン（1日）

1. **端末Aが広告、端末Bがスキャン**
   - 広告データに小メッセージ（100バイト程度）を含める
   - 検出率を測定（1m、5m、10m）

2. **成功基準**:
   - ✅ 10m以内で検出率90%以上
   - ✅ 広告データが正しく受信できる

#### フェーズ2: GATT接続とデータ転送（2-3日）

1. **端末Aが広告、端末Bが接続してデータ取得**
   - GATT Characteristicに1KB、10KB、100KBのデータを配置
   - 接続確立からデータ転送完了までの時間を測定

2. **成功基準**:
   - ✅ 3秒以内に1KB転送可能
   - ✅ 10秒以内に10KB転送可能

#### フェーズ3: 双方向転送とメッセージ交換（2-3日）

1. **端末AとBが相互にメッセージを交換**
   - 端末Aが保持するメッセージリストを端末Bに送信
   - 端末Bが保持するメッセージリストを端末Aに送信
   - 重複排除（既に持っているメッセージは転送しない）

2. **成功基準**:
   - ✅ 両方向の転送が成功
   - ✅ 重複排除が動作

#### フェーズ4: リレー動作の検証（2-3日）

1. **3台でのリレー**
   - 端末A → 端末B → 端末C
   - 端末Aで生成したメッセージが、端末Cに到達するか確認
   - 各転送のログを記録

2. **成功基準**:
   - ✅ メッセージが端末Cに到達
   - ✅ ログで転送経路が確認できる

#### フェーズ5: 電池消費測定（3-5日）

1. **長時間のバックグラウンド動作**
   - スキャン頻度を1分、5分、10分で変えて測定
   - 8時間の電池消費率を記録

2. **成功基準**:
   - ✅ 5分間隔で5%以下/日の消費
   - ✅ 1分間隔で10%以下/日の消費

#### フェーズ6: 模擬すれ違いテスト（1-2日）

1. **実際の移動を模擬**
   - 端末を持って歩き、すれ違い（3-5秒の接触）を再現
   - メッセージ転送の成功率を測定

2. **成功基準**:
   - ✅ 3秒の接触で1KB転送成功率80%以上

### 8.4 所要時間と予算

| 項目 | 期間 | 備考 |
|------|------|------|
| **準備** | 1週間 | Android端末調達、開発環境構築 |
| **フェーズ1-6** | 2-3週間 | 各フェーズ1-5日 |
| **分析・レポート** | 1週間 | 結果まとめ、判定 |
| **合計** | **4-7週間** | - |

**予算**:
- Android端末3台（既存端末を流用可能なら不要）
- 開発者人件費（1名、パートタイム可）

### 8.5 判定基準

| 結果 | 推奨アクション |
|------|----------------|
| ✅ **すべてのフェーズ成功** | 本格開発開始（Android優先） |
| ⚠️ **フェーズ5（電池）のみ失敗** | スキャン頻度を調整して再試行 |
| ⚠️ **フェーズ6（すれ違い）のみ失敗** | 接続確立時間の最適化が必要 |
| ❌ **フェーズ2-4失敗** | **Rejected** または技術変更（Wi-Fi Direct等） |

---

## 9. リスクと緩和策

### 9.1 技術的リスク

| リスク | 影響 | 確率 | 緩和策 |
|--------|------|------|--------|
| **iOS対応困難** | iOS端末が中継に参加できない | 高 | Android優先で開発、iOS対応は後回し |
| **バックグラウンド制約** | OSがアプリを強制終了 | 中 | フォアグラウンドサービス、ユーザ啓発 |
| **すれ違い時間不足** | 3秒では転送完了しない | 中 | メッセージサイズを1KB以下に制限 |
| **電池消費過大** | 数時間で電池切れ | 中 | スキャン頻度を動的調整 |
| **接続数制限** | 8接続を超えるとオーバーフロー | 低 | 接続を短時間で切断、順次処理 |

### 9.2 セキュリティリスク

| リスク | 影響 | 確率 | 緩和策 |
|--------|------|------|--------|
| **フラッディング攻撃** | ネットワーク機能停止 | 高 | レート制限、DTN-Cookies認証 |
| **なりすまし** | 偽メッセージの拡散 | 高 | メッセージ署名、PKI |
| **追跡** | ユーザ移動の監視 | 中 | IDローテーション、匿名化 |
| **DoS（Zip爆弾等）** | アプリクラッシュ | 中 | サイズ制限、圧縮禁止 |
| **改ざん** | メッセージ内容の変更 | 低 | 署名検証、ハッシュチェーン |

### 9.3 運用・倫理的リスク

| リスク | 影響 | 確率 | 緩和策 |
|--------|------|------|--------|
| **デマ拡散** | 混乱、誤誘導 | 高 | 発信者署名、信頼度スコア、レピュテーション |
| **スパム** | ネットワーク混雑 | 中 | レート制限、ブラックリスト、優先度付け |
| **プライバシー侵害** | 平時の追跡 | 中 | IDローテーション、オプトイン方式 |
| **期待値管理** | 「必ず届く」と誤解 | 高 | 明確な説明、限界の開示 |
| **悪用（犯罪通信）** | 違法活動への利用 | 低 | 法執行機関との連携、利用規約 |

---

## 10. 結論と推奨事項

### 10.1 総合評価

| 項目 | 評価 |
|------|------|
| **技術的実現可能性** | ⭐⭐⭐⭐☆（4/5） - Android限定なら高い |
| **有用性** | ⭐⭐⭐⭐☆（4/5） - 通信断時の情報伝播に有効 |
| **運用容易性** | ⭐⭐☆☆☆（2/5） - バックグラウンド制約、電池管理が課題 |
| **セキュリティ/プライバシー** | ⭐⭐☆☆☆（2/5） - 既存実装に深刻な脆弱性、慎重な設計必須 |
| **開発難易度** | ⭐⭐⭐☆☆（3/5） - BLE通信は標準的だが、DTNロジックが複雑 |
| **iOS対応** | ⭐☆☆☆☆（1/5） - API制約により極めて困難 |

### 10.2 推奨アクション

#### 短期（1-2ヶ月）

1. **PoCの実施（Android優先）**
   - 上記8.3の手順でBLE S&Fの成立性を検証
   - 特にフェーズ6（すれ違い）の成功が重要

2. **BLE Mesh規格の採用を見送る**
   - iOS制約により実用性が低い
   - 独自S&F実装が現実的

3. **セキュリティ設計の検討**
   - Bridgefyの失敗を教訓に、初期から署名・暗号化を組み込む
   - フラッディング攻撃対策（レート制限）を必須とする

#### 中期（2-6ヶ月）

1. **最小実装（MVP）**
   - Android専用で、基本的なS&F機能を実装
   - メッセージ種別: 救助要請、生存報告、目撃情報（各1KB以下）
   - TTL（生存時間）: 24時間
   - 暗号化: Ed25519署名 + AES-256暗号化

2. **フィールドテスト**
   - 避難訓練、防災イベントで実証実験
   - 10-50人規模での動作確認

3. **iOS対応の可否判断**
   - PoCでiOS-Android間の転送を試験
   - 成立しない場合は「Android専用」として割り切る

#### 長期（6ヶ月〜）

1. **標準化とオープンソース化**
   - プロトコル仕様を公開、第三者監査を実施
   - OSSとして公開し、コミュニティによる改善を促進

2. **他災害アプリとの連携**
   - 既存の防災アプリ（Yahoo!防災速報等）とのAPI連携
   - 自治体の避難情報配信システムとの統合

3. **専用ハードウェアの検討**
   - スマホ電池切れに備え、低消費電力のBLE中継専用機を開発
   - ソーラー充電、長寿命電池（数年）を搭載

### 10.3 Go/No-Go判定ゲート

以下の条件を**すべて満たす場合**、開発継続を推奨:

- ✅ PoCで3秒の接触で1KB転送が成功率80%以上
- ✅ リレー（A→B→C）が動作することを確認
- ✅ 電池消費が10%/日以下（スキャン頻度1分）
- ✅ セキュリティ設計（署名・暗号化・レート制限）が実装可能
- ✅ Android専用での運用が許容される（iOS対応は後回し）

**1つでも満たさない場合**: Rejected または大幅な設計変更（Wi-Fi Direct、専用ハードウェア等）

### 10.4 代替技術の検討

本アイデアが不成立の場合の代替案：

| 技術 | メリット | デメリット |
|------|----------|------------|
| **Wi-Fi Direct** | 高速（数Mbps）、長距離（100m） | 電池消費大、接続確立に時間 |
| **Wi-Fi Aware** | 低消費電力、自動接続 | 対応端末が少ない（Android 8以降） |
| **LoRa専用端末** | 超長距離（数km）、低消費電力 | 専用ハードウェア必須、高コスト |
| **Meshtastic** | オープンソース、長距離 | 専用ハードウェア必須 |

---

## 11. 参考文献

### BLE Mesh技術仕様
- [Bluetooth mesh networking - Wikipedia](https://en.wikipedia.org/wiki/Bluetooth_mesh_networking)
- [Bluetooth® Mesh Feature Enhancements Summary](https://www.bluetooth.com/mesh-feature-enhancements-summary/)
- [Bluetooth Mesh Networking: The Ultimate Guide](https://novelbits.io/bluetooth-mesh-networking-the-ultimate-guide/)
- [What's New with the Latest Bluetooth Mesh Specification - Silicon Labs](https://www.silabs.com/blog/whats-new-with-the-latest-bluetooth-mesh-specification)

### DTN（遅延耐性ネットワーク）
- [Delay/Disruption Tolerant Networking - NASA](https://www.nasa.gov/communicating-with-missions/delay-disruption-tolerant-networking/)
- [All about Delay-Tolerant Networking (DTN) Contributions to Future Internet](https://www.mdpi.com/1999-5903/16/4/129)
- [Delay-tolerant networking - Wikipedia](https://en.wikipedia.org/wiki/Delay-tolerant_networking)
- [Delay- and Disruption-Tolerant Networks (DTNs) A Tutorial](https://www.nasa.gov/wp-content/uploads/2023/09/dtn-tutorial-v3.2-0.pdf)
- [Disaster Management through Delay Tolerant Networks](http://article.nadiapub.com/IJFGCN/vol10_no3/7.pdf)

### ルーティングプロトコル
- [Routing in delay-tolerant networking - Wikipedia](https://en.wikipedia.org/wiki/Routing_in_delay-tolerant_networking)
- [Epidemic Routing for Partially-Connected Ad Hoc Networks](https://www.researchgate.net/publication/2633330_Epidemic_Routing_for_Partially-Connected_Ad_Hoc_Networks)
- [An Improved PRoPHET Routing Protocol in Delay Tolerant Network](https://pmc.ncbi.nlm.nih.gov/articles/PMC4306380/)

### iOS/Android BLE制約
- [Core Bluetooth Background Processing for iOS Apps](https://developer.apple.com/library/archive/documentation/NetworkingInternetWeb/Conceptual/CoreBluetooth_concepts/CoreBluetoothBackgroundProcessingForIOSApps/PerformingTasksWhileYourAppIsInTheBackground.html)
- [iOS BLE Scanning: Building Reliable Core Bluetooth Discovery](https://punchthrough.com/ios-ble-scanning-guide/)
- [Hacking The Overflow Area](https://davidgyoungtech.com/2020/05/07/hacking-the-overflow-area)
- [iOS advertisements in the background - Crownstone](https://crownstone.rocks/2018/06/27/ios-advertisements-in-the-background)
- [GitHub - NordicSemiconductor/IOS-nRF-Mesh-Library](https://github.com/NordicSemiconductor/IOS-nRF-Mesh-Library)
- [GitHub - NordicSemiconductor/Android-nRF-Mesh-Library](https://github.com/NordicSemiconductor/Android-nRF-Mesh-Library)

### BLEデータ転送性能
- [Maximizing BLE Throughput Part 4 - Punch Through](https://punchthrough.com/ble-throughput-part-4/)
- [Bluetooth 5 speed: How to achieve maximum throughput](https://novelbits.io/bluetooth-5-speed-maximum-throughput/)
- [A Practical Guide to BLE Throughput - Interrupt](https://interrupt.memfault.com/blog/ble-throughput-primer)

### 既存実装事例
- [FireChat in Hong Kong - CNN](https://www.cnn.com/2014/10/16/tech/mobile/tomorrow-transformed-firechat/index.html)
- [How the Hong Kong Protests Forced Firechat to Evolve - Vice](https://www.vice.com/en/article/what-firechat-learned-in-hong-kong/)
- [Bridgefy FAIL: Insecure for Use in Protests](https://securityboulevard.com/2020/08/bridgefy-fail-insecure-for-use-in-protests/)
- [Breaking Bridgefy, again](https://eprint.iacr.org/2021/214.pdf)
- [Breaking Bridgefy, again - USENIX](https://www.usenix.org/conference/usenixsecurity22/presentation/albrecht)
- [How it works - Briar](https://briarproject.org/how-it-works/)
- [Chatting offline: mesh messaging apps - Kaspersky](https://www.kaspersky.com/blog/mesh-messengers/54192/)
- [goTenna Pro in Puerto Rico - 23.1 Mile Hop](https://gotennapro.com/blogs/case-studies/lcg-holdings-tests-gotenna-pro-in-puerto-rico-and-achieves-23-1-mile-single-hop-range)
- [Mesh network in Puerto Rico - TechCrunch](https://techcrunch.com/2017/11/14/a-mesh-network-spontaneously-erupts-in-the-us-and-helps-connect-puerto-rico/)

### セキュリティ
- [Mitigation on Flood Attacks in DTN's - IJERT](https://www.ijert.org/mitigation-on-flood-attacks-in-dtns)
- [Detecting Flooding Attack and Accommodating Burst Traffic - IEEE](https://ieeexplore.ieee.org/document/8024026/)
- [A Security Scheme to Mitigate DoS Attacks in DTN](https://pubs.sciepub.com/jcsa/5/2/2/index.html)
- [Flooding-Resilient Broadcast Authentication for VANETs](https://netsec.ethz.ch/publications/papers/hsiao_mobicom11.pdf)

### 電池消費
- [Will the NHS COVID-19 app drain my phone battery?](https://faq.covid19.nhs.uk/article/KA-01163/en-us)
- [COVID-19 and Your Smartphone: BLE-Based Smart Contact Tracing](https://pmc.ncbi.nlm.nih.gov/articles/PMC8843047/)

---

**次のアクション**: 本レポートをベースに、PoC実施の可否判断とスケジュール策定を行う
