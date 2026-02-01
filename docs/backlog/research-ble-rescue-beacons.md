# 調査レポート: BLEタグを救助ビーコンとして活用

**対象アイデア**: IDEA-0001: 「落とし物タグ（BLEタグ）を救助ビーコンとして活用」
**調査日**: 2026-02-01
**調査者**: Claude (AI Assistant)

---

## エグゼクティブサマリー

BLE（Bluetooth Low Energy）タグを災害時の救助ビーコンとして活用するアイデアについて、技術仕様、既存製品、実現可能性、セキュリティ/プライバシー、法的・倫理的側面を調査した。

**主な結論**:
- ✅ **技術的には実現可能**: BLEビーコンは10m〜60m程度の検出範囲を持ち、救助用途での観測は可能
- ⚠️ **ルート選択が重要**: ルートA（ID解釈型）とルートB（電波源検出型）では実装難易度と制約が大きく異なる
- ⚠️ **プライバシー対策必須**: アンチストーキング機能との競合を慎重に設計する必要がある
- ✅ **電池寿命は十分**: 適切な設定で数ヶ月〜数年の動作が可能
- ⚠️ **精度には限界**: 屋内や複雑環境では位置推定精度が大幅に低下（10cm〜数m以上の誤差）

**推奨**: PoCで「市販タグの安定検出」と「RSSI精度」を先に検証し、ルートA/Bの選択を確定すべき

---

## 1. BLEビーコン技術の基礎仕様

### 1.1 通信規格

| 項目 | 仕様 |
|------|------|
| 周波数帯 | 2.400–2.4835 GHz ISM帯 |
| チャネル数 | 40チャネル（2MHz幅）、広告専用3チャネル（37, 38, 39） |
| 広告間隔 | 20ms〜10.24秒（典型的には100ms〜1秒） |
| 送信出力 | −30dBm〜+4dBm以上（BLE 5.0以降）|
| 通信距離 | 屋外見通し環境で最大60m以上、実用的には10〜30m |
| PDUタイプ | `ADV_NONCONN_IND`（接続不要の広告）が一般的 |

**出典**: [Bluetooth Low Energy beacon - Wikipedia](https://en.wikipedia.org/wiki/Bluetooth_Low_Energy_beacon), [Bluetooth Low Energy: It starts with advertising | Bluetooth® Technology Website](https://www.bluetooth.com/blog/bluetooth-low-energy-it-starts-with-advertising/)

### 1.2 主要ビーコンフォーマット

Bluetooth SIGには公式のビーコン標準が存在せず、以下の疑似標準が乱立:

- **iBeacon** (Apple): UUIDベースの識別
- **Eddystone** (Google): URL/UID/TLMフレーム対応
- **AltBeacon**: オープン標準
- **各社独自仕様**: Tile, Chipolo, Samsung SmartTag等

**出典**: [Understanding the different types of BLE Beacons | Mbed](https://os.mbed.com/blog/entry/BLE-Beacons-URIBeacon-AltBeacons-iBeacon/)

---

## 2. 既存BLEタグ製品の仕様

### 2.1 Apple AirTag

| 項目 | 仕様 |
|------|------|
| チップセット | Nordic nRF52832 (BLE 5.2) + Apple U1 (UWB) |
| 広告間隔 | 通常2秒、未登録時33ms |
| 暗号化 | NIST楕円曲線P-224公開鍵暗号 |
| IDローテーション | 1日1回（午前4時、または電源サイクル後24時間） |
| アドレスタイプ | ランダム静的（公開鍵の最初6バイト） |
| 検出範囲 | BLEで数十m、UWB（iPhone 11以降）で精密測位 |

**セキュリティ特性**:
- 公開鍵を含むBLE広告を送信
- Find Myネットワークを利用した位置報告（暗号化済み）
- 所有者から離れた状態で一定時間経過後、音声アラート発生

**出典**: [Apple AirTag Reverse Engineering - Adam Catley](https://adamcatley.com/AirTag.html), [The technology behind Apple's AirTag | by Prashanth Basappa | Nerd For Tech | Medium](https://medium.com/nerd-for-tech/the-technology-behind-apples-airtag-c7983f9322b5)

### 2.2 Tile

| 項目 | 仕様 |
|------|------|
| 検出範囲 | Tile Pro 2018で最大15m（安定距離） |
| プロトコル | 独自BLEビーコンフォーマット |
| クラウドサーチ | Tileアプリユーザーネットワーク経由で位置報告 |
| 未追跡検出遅延 | 中央値4,413分（約3日）※アンチストーキング機能 |

**出典**: [What Is the Tile Tracker Range? | Tile Bluetooth Range](https://www.life360.com/blog/how-far-can-you-track-a-tile-bluetooth-tracker-range), [(PDF) Please Unstalk Me: Understanding Stalking with Bluetooth Trackers and Democratizing Anti-Stalking Protection](https://www.researchgate.net/publication/381874576_Please_Unstalk_Me_Understanding_Stalking_with_Bluetooth_Trackers_and_Democratizing_Anti-Stalking_Protection)

### 2.3 Samsung SmartTag

| 項目 | 仕様 |
|------|------|
| 検出範囲 | 約120m（見通し環境） |
| 未追跡検出遅延 | 中央値70分（最速クラス） |
| クラウドサーチ | Galaxy端末のSmartThings Findネットワーク |

**出典**: [BLE-Doubt: Smartphone-Based Detection of Malicious Bluetooth Trackers](https://safe-things-2022.github.io/accepted_papers/safethings2022-final1.pdf)

---

## 3. 救助ビーコンとしての技術的実現可能性

### 3.1 検出範囲と精度

#### 3.1.1 RSSI（受信信号強度）による距離推定

**精度特性**:
- **1m以内**: 10cm未満の誤差（理想環境）
- **1〜3m**: 0.27m程度の誤差（フィルタリング使用時）
- **3m超**: 1m以上の誤差が一般的
- **屋外**: 地面や建物からの反射の影響
- **屋内**: 壁、天井、家具からの反射、マルチパス、フェージングにより精度が大幅低下

**出典**: [A Practice of BLE RSSI Measurement for Indoor Positioning - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8347277/), [Improving BLE Beacon Proximity Estimation Accuracy through Bayesian Filtering](https://arxiv.org/pdf/2001.02396)

#### 3.1.2 救助現場での実用例

**AirFlare（荒野安全アプリ）**:
- 端末は救助チームの信号を待ち受け、ユーザ操作不要で検出可能
- **検出範囲**: 開放地で0.5マイル（約800m）以上
- **制約**: 森林地帯や起伏の多い地形では大幅に範囲が縮小

**MOB（Man Overboard）システム**:
- BLEリストバンドとフロートタグで海難事故時の救助を支援
- セルラー通信不要で動作
- **検出遅延**: 8秒以内

**出典**: [Have a phone? Now you have a rescue locator. - Airflare - Wilderness Safety App](https://airflare.com/), [Safety-promoting mobile application](https://onix-systems.com/marine-safety-case)

### 3.2 ルート別の実現可能性評価

#### ルートA: 広告ID解釈型

**前提条件**:
- タグが固定IDまたは予測可能なIDローテーションを行う
- 広告データから識別子を抽出可能

**課題**:
- ❌ **AirTag**: 公開鍵が1日1回変わり、Find Myネットワーク外では識別困難
- ❌ **Tile**: 独自プロトコルで公開仕様なし
- ⚠️ **汎用iBeacon/Eddystone**: UUID/UIDが固定なら識別可能だが、市販品の多くは非公開

**メリット**:
- タグ所有者の特定が可能
- 事前登録により、家族情報等と紐付け可能

**実現可能性**: **中〜低** （市販品の仕様に強く依存）

#### ルートB: 電波源検出型

**前提条件**:
- BLE広告が存在し、RSSIが取得できればOK
- IDは解釈しない（「何かあるだけ」を検出）

**メリット**:
- ✅ ほぼすべてのBLEタグで動作
- ✅ 実装が単純（スキャン + RSSI測定のみ）

**課題**:
- ❌ タグ所有者の特定不可（目撃情報・家族証言との組み合わせ必須）
- ❌ 大量の無関係なBLE機器（スマートウォッチ、イヤホン等）との区別が困難

**実現可能性**: **高** （技術的ハードルは低いが、運用上の有用性に課題）

---

## 4. セキュリティとプライバシーの課題

### 4.1 追跡・ストーキングリスク

BLEタグは**小型・安価・長寿命**で、悪用されやすい特性を持つ:
- 被害者の衣服、所持品、車両に短時間でタグを設置
- 数ヶ月〜数年にわたり精密な追跡が可能

**出典**: [Security and Privacy of Wireless Beacon Systems](https://arxiv.org/pdf/2107.05868), [BLE-Doubt: Smartphone-Based Detection of Malicious Bluetooth Trackers](https://safe-things-2022.github.io/accepted_papers/safethings2022-final1.pdf)

### 4.2 アンチストーキング機能との競合

#### iOSの保護機能
- **自動検出**: 未知のAirTagが一定期間同行している場合、通知を送信
- **音声アラート**: 所有者から離れた後、一定時間で音を鳴らす
- **Precision Finding**: iPhone 11以降でUWBによる精密位置特定

#### Androidの保護機能
- **Google標準機能**: 未知のBluetoothトラッカーを自動検出（2023年7月〜）
- **手動スキャン**: 設定 > 安全と緊急 > 未知のトラッカー警告 > 今すぐスキャン
- **サードパーティアプリ**: AirGuard等（AirTag、SmartTag、Find My Deviceトラッカーを検出）

#### Apple-Google共同標準（DULT）
- iOS 17.5およびAndroid 6以降で相互運用可能
- Chipolo, eufy, Jio, Motorola, Pebblebeeが対応表明

**出典**: [Apple and Google team up on anti-stalking feature for AirTag, Tile, more](https://9to5mac.com/2023/05/02/apple-google-unwanted-tracking-abuse-technology/), [AirGuard - AirTag protection | F-Droid - Free and Open Source Android App Repository](https://f-droid.org/en/packages/de.seemoo.at_tracking_detection/)

### 4.3 救助用途とアンチストーキング機能の衝突

**問題点**:
1. **音声アラート**: 所有者から離れた状態（災害時を含む）で音が鳴り、タグ位置が露呈
2. **検出遅延**: 救助側が「未知のトラッカー」として警告を受け、タグを無視/除外する可能性
3. **プライバシー設計**: IDローテーションにより、救助側が識別できない

**緩和策**:
- 「緊急モード」の設計（アンチストーキング機能を一時停止）
  - トリガー: 地震検知、手動起動、長時間の静止検出等
  - 懸念: 悪用（偽装緊急モード）のリスク
- 救助者側での「許可されたスキャン」権限管理
  - 公的機関（消防、警察）のみがスキャン可能
  - 懸念: 権限管理の複雑さ、民間ボランティアの排除

---

## 5. 電池寿命と運用性

### 5.1 電池寿命の実測値

| 広告間隔 | 送信電力 | 電池種類 | 寿命 |
|----------|----------|----------|------|
| 100ms | 標準 | コイン電池 | 1〜3ヶ月 |
| 900ms | 標準 | コイン電池 | 2〜3年 |
| - | 標準 | リチウムコイン電池 | 3〜5年 |
| - | 標準 | 亜鉛マンガン電池 | 約1年 |

**省電力化の手法**:
- スリープモードの活用
- 広告間隔の延長（ただし検出性とトレードオフ）
- 送信電力の低減（ただし検出範囲が縮小）

**出典**: [Bluetooth Beacons Compared, Part 1: Battery Life and Type – Reelables](https://reelables.com/bluetooth-beacons-battery-compared/), [BLE Beacon: Battery monitoring, low power operation and power save functionalities](https://community.infineon.com/t5/Knowledge-Base-Articles/CYW20706-BLE-Low-Power-Beacon/ta-p/246355)

### 5.2 災害時の運用適性

**メリット**:
- ✅ 広告間隔を1秒程度に設定すれば、数ヶ月〜1年以上の動作が可能
- ✅ 平時の「持ち歩くだけ」運用が現実的
- ✅ スマートフォンと異なり、充電不要（電池交換は年単位）

**課題**:
- ⚠️ 電池切れのリスク（ユーザーが交換を忘れる）
- ⚠️ 災害時に既に電池切れの可能性

**推奨設定**:
- 平時: 広告間隔 500ms〜1秒（電池寿命2〜3年）
- 緊急モード: 広告間隔 100ms（検出速度優先、短期間のみ）

---

## 6. 法的・倫理的考慮事項

### 6.1 日本における法的枠組み

**災害救助関連法**:
- **災害対策基本法**: 災害時の捜索・救助活動の法的根拠
- **電波法**: BLE機器は技適認証が必要（2.4GHz帯は比較的規制が緩い）

**PLB（Personal Locator Beacon）規制**:
- 日本の技適適合PLB: ACR ResQLink+ (日本版) のみ
- **使用範囲**: 海上のみ（陸上・内水面は不可）
- GPS衛星メッセンジャー（SPOT, Garmin inReach）は日本国内どこでも使用可

**出典**: [Can I use a PLB in Japan? | HokkaidoWilds.org](https://hokkaidowilds.org/can-use-plb-japan), [Basic Act on Disaster Management - English - Japanese Law Translation](https://www.japaneselawtranslation.go.jp/en/laws/view/3322/en)

**BLEタグの法的位置づけ**:
- 現状、「救助ビーコン」としての公式規格・規制は存在しない
- 技適認証済みBLE機器であれば、一般的な使用は合法
- ただし、救助機関が「公式救助ツール」として採用する場合、別途基準が必要

### 6.2 倫理的課題

#### 期待値管理
- ❌ **過信リスク**: 「タグがあれば必ず見つかる」との誤解
- ✅ **現実的説明**: 「補助的手段の一つ」として位置づけ

#### プライバシーと救助のバランス
- アンチストーキング機能を緩和すれば、悪用リスク増大
- 厳格な保護を維持すれば、救助時の有用性が低下

#### インフォームドコンセント
- ユーザーが「災害時に第三者から検出される」ことを理解した上での登録が必要

---

## 7. PoC（概念実証）推奨計画

### 7.1 目的

1. 市販BLEタグの安定検出可能性を確認
2. RSSI精度と距離推定の実測
3. ルートA/ルートBの実現可能性を判定

### 7.2 準備物

- **タグ**: 2〜3種類（例: 汎用iBeacon、AirTag、Tile）
- **端末**: Android 1台、iOS 1台
- **アプリ**:
  - 自作スキャンアプリ（推奨: Android Beacon Library使用）
  - 既存アプリ（参考: nRF Connect, Beacon Scanner）

**出典**: [Android Beacon Library](https://altbeacon.github.io/android-beacon-library/)

### 7.3 実験手順

#### フェーズ1: 基本検出確認（1〜2時間）
1. 各タグをスキャンし、広告データを記録
2. 広告間隔、IDの固定/ローテーション、データフォーマットを確認
3. iOS/Androidでの検出可否を確認

**成功基準**:
- ✅ 広告が1分以内に検出できる
- ✅ 広告データが解析可能（ルートAの場合）

#### フェーズ2: 距離別RSSI測定（2〜3時間）
1. 距離を1m, 3m, 5m, 10m, 20mで固定
2. 各距離で30秒間スキャンし、RSSI値を記録
3. 屋内（部屋）と屋外（広場）で測定

**成功基準**:
- ✅ 10m以内で検出率90%以上
- ✅ RSSI値が距離と相関している（±5dBmのばらつき以内）

#### フェーズ3: 模擬捜索（2〜3時間）
1. タグを「被災者役」が持ち、建物内/屋外に隠れる
2. 「救助者役」がスキャンしながら捜索
3. RSSIの変化を見ながら、タグに近づけるかを検証

**成功基準**:
- ✅ 5m以内に近づけることができる
- ⚠️ 誤検出（他のBLE機器）の頻度が許容範囲

### 7.4 判定基準

| 結果 | 推奨アクション |
|------|----------------|
| ✅ ルートA実現可能（ID解釈可、安定検出） | ルートAで開発開始 |
| ✅ ルートB実現可能（安定検出のみ） | ルートBで開発開始（運用設計に注力） |
| ❌ 検出不安定（検出率50%以下） | **Rejected** または送信電力の高いタグを検討 |

---

## 8. リスクと緩和策

### 8.1 技術的リスク

| リスク | 影響 | 緩和策 |
|--------|------|--------|
| タグのIDローテーション | 識別不可（ルートA破綻） | ルートBへ移行、または固定IDタグを採用 |
| 検出不安定（屋内、瓦礫下） | 見逃しリスク | 複数タグ所持、送信電力を最大化 |
| 誤検出（他のBLE機器） | 捜索効率低下 | フィルタリング（既知デバイスリスト、信号パターン） |
| 電池切れ | 救助時に機能せず | 電池残量通知、年次交換推奨 |

### 8.2 運用・倫理的リスク

| リスク | 影響 | 緩和策 |
|--------|------|--------|
| 過信（「タグで必ず見つかる」） | 他の安全対策が疎かに | 明確な期待値設定、限界の説明 |
| アンチストーキング機能との競合 | 検出遅延、音声アラート | 緊急モードの設計（慎重な検討必要） |
| プライバシー侵害 | 平時の追跡リスク | IDローテーション維持、スキャン権限管理 |
| 悪用（偽タグ設置） | 誤誘導、混乱 | 署名検証（ルートAの場合）、登録済みタグのみ検出 |

---

## 9. 結論と推奨事項

### 9.1 総合評価

| 項目 | 評価 |
|------|------|
| **技術的実現可能性** | ⭐⭐⭐⭐☆（4/5） - ルートBなら高い実現性 |
| **有用性** | ⭐⭐⭐☆☆（3/5） - スマホ死亡ケースでは有効だが、精度に限界 |
| **運用容易性** | ⭐⭐⭐⭐☆（4/5） - 「持つだけ」で準備完了 |
| **セキュリティ/プライバシー** | ⭐⭐☆☆☆（2/5） - アンチストーキング機能との競合が課題 |
| **コスト** | ⭐⭐⭐⭐⭐（5/5） - 市販タグ活用で低コスト |

### 9.2 推奨アクション

#### 短期（1〜2週間）
1. **PoCの実施**: 上記7.3の手順で、市販タグの検出可能性を検証
2. **ルート選択**: PoCの結果に基づき、ルートA/Bを確定
3. **リスク評価**: プライバシー/セキュリティチームと連携し、アンチストーキング機能との共存方針を決定

#### 中期（1〜3ヶ月）
1. **最小実装**: 選択したルートでのプロトタイプ開発
2. **フィールドテスト**: 実際の建物/屋外環境で検出テスト
3. **法的検討**: 救助機関との協議、法的位置づけの確認

#### 長期（3ヶ月〜）
1. **標準化検討**: 救助用BLEタグの仕様を策定（業界標準として提案）
2. **緊急モード設計**: アンチストーキング機能との共存方法を確立
3. **社会実装**: 自治体、消防との連携、啓発活動

### 9.3 Go/No-Go判定ゲート

以下の条件を**すべて満たす場合**、開発継続を推奨:

- ✅ PoCで10m以内での検出率が80%以上
- ✅ アンチストーキング機能との競合に対する緩和策が設計可能
- ✅ ルートBでの運用（ID解釈なし）でも有用性が認められる

**1つでも満たさない場合**: Rejected または再設計

---

## 10. 参考文献

### BLE技術仕様
- [Bluetooth Low Energy beacon - Wikipedia](https://en.wikipedia.org/wiki/Bluetooth_Low_Energy_beacon)
- [Bluetooth Low Energy: It starts with advertising | Bluetooth® Technology Website](https://www.bluetooth.com/blog/bluetooth-low-energy-it-starts-with-advertising/)
- [Understanding the different types of BLE Beacons | Mbed](https://os.mbed.com/blog/entry/BLE-Beacons-URIBeacon-AltBeacons-iBeacon/)

### 既存製品
- [Apple AirTag Reverse Engineering - Adam Catley](https://adamcatley.com/AirTag.html)
- [The technology behind Apple's AirTag | Medium](https://medium.com/nerd-for-tech/the-technology-behind-apples-airtag-c7983f9322b5)
- [What Is the Tile Tracker Range? | Tile Bluetooth Range](https://www.life360.com/blog/how-far-can-you-track-a-tile-bluetooth-tracker-range)

### 救助用途
- [Have a phone? Now you have a rescue locator. - Airflare](https://airflare.com/)
- [Safety-promoting mobile application](https://onix-systems.com/marine-safety-case)

### セキュリティ/プライバシー
- [Security and Privacy of Wireless Beacon Systems](https://arxiv.org/pdf/2107.05868)
- [BLE-Doubt: Smartphone-Based Detection of Malicious Bluetooth Trackers](https://safe-things-2022.github.io/accepted_papers/safethings2022-final1.pdf)
- [Apple and Google team up on anti-stalking feature for AirTag, Tile, more](https://9to5mac.com/2023/05/02/apple-google-unwanted-tracking-abuse-technology/)
- [Please Unstalk Me: Understanding Stalking with Bluetooth Trackers](https://www.researchgate.net/publication/381874576_Please_Unstalk_Me_Understanding_Stalking_with_Bluetooth_Trackers_and_Democratizing_Anti-Stalking_Protection)

### RSSI精度
- [A Practice of BLE RSSI Measurement for Indoor Positioning - PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8347277/)
- [Improving BLE Beacon Proximity Estimation Accuracy through Bayesian Filtering](https://arxiv.org/pdf/2001.02396)

### 電池寿命
- [Bluetooth Beacons Compared, Part 1: Battery Life and Type – Reelables](https://reelables.com/bluetooth-beacons-battery-compared/)
- [BLE Beacon: Battery monitoring, low power operation and power save functionalities](https://community.infineon.com/t5/Knowledge-Base-Articles/CYW20706-BLE-Low-Power-Beacon/ta-p/246355)

### 法的枠組み
- [Can I use a PLB in Japan? | HokkaidoWilds.org](https://hokkaidowilds.org/can-use-plb-japan)
- [Basic Act on Disaster Management - Japanese Law Translation](https://www.japaneselawtranslation.go.jp/en/laws/view/3322/en)

### 開発ツール
- [Android Beacon Library](https://altbeacon.github.io/android-beacon-library/)
- [AirGuard - AirTag protection | F-Droid](https://f-droid.org/en/packages/de.seemoo.at_tracking_detection/)

---

**次のアクション**: 本レポートをベースに、PoC計画の詳細設計と実施スケジュールを策定
