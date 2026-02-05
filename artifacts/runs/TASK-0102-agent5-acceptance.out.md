`docs/constitution/30_testplan.md:24` に **TST-0012** を最小Diffで追加しました。  
内容は指定どおりで、`docs/rfc/DEC-0005-notification-minimal-set.md` に対して:

- 最小セット `occurred_at / action_kind / target_ref / accountability_token` の明記
- 禁止事項（個人情報・`payload` 等の内容・精密位置・自由記述を含めない）の明記

を観測する acceptance（`grep` 1行コマンド付き）です。

既存TSTは弱体化していません。`./ci.sh` は green です。