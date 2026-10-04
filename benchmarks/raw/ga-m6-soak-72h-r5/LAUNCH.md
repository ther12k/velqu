# r5 launch record — ga-m6-soak-72h-rc1 (8b3dce23) on Halotec

Assignment: the RC1 qualification program (OD-011, #1415). Host and
operator per the standing r4 assignment — the owner's 2026-09-20
directive assigned Halotec as the stable soak host; r5 continues on it
(53 days uptime at launch, boot 2026-08-11, docker 24.0.2). The
original host remains barred (owner boundary 2026-09-15).

## Identity

- Candidate: `8b3dce23db884b11e3a17543825d839453c6e0a4` (RC1 = the live
  beta.2 content; nomination record `docs/production/evidence/rc1-nomination.md`).
- q-soak built at the candidate SHA via host build with the canonical
  remap flags (worktree at the exact SHA); sha256
  `24bfc207008ac6cc79a9a5c5c5978368ad5c35861039bb1c371fa70ec4bc78b5`,
  **verified bit-identical on the launch host after copy**.
- Image: `velqu-bench:multihost` **e1de7b18613f — bit-identical to
  r4's image** (re-shipped from the qualification workstation via
  `docker save | ssh docker load` after Halotec pruned it; loaded ID
  matched r4's recorded image exactly).
- Container: `velqu-soak-rc1-r5` (b4223c19f79c), containment identical
  to r4: `--network none --cpus 3 --memory 4g --restart no` (no
  auto-restart by design — a dead run must surface as dead).
- Args identical to r4: `--workers 2 --duration-secs 259200
  --window-secs 120`, chaos disabled.
- Launched **2026-10-02T18:19:36Z**; expected completion
  **2026-10-05T18:19:36Z** (259200 s).

## Interim health checks (qualification session)

- 21.35 h: 80.9 M verified, RSS 6.79 MiB, terminal-quarter slope
  +69.8 KiB/h.
- 36.57 h: 138.4 M verified, RSS 7.75 MiB, slope +59.7 KiB/h
  (decelerating, r4 shape).
- 47.51 h: 179.9 M verified, RSS 8.55 MiB, slope +69.6 KiB/h — inside
  r4's passing envelope throughout (bound: 250 KiB/h; max window step
  observed +264 KiB vs bound 2048).
