No edits needed; `docs/constitution/30_testplan.md` already includes TST-0012 with the required minimal set fields and prohibited items, so there’s no diff to apply. I ran `./ci.sh` and it’s green.

If you want, I can:
1) verify `docs/rfc/DEC-0005-notification-minimal-set.md` content against TST-0012, or  
2) wire the grep check into `./ci.sh` as an always-on gate.