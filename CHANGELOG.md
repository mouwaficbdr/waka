# Changelog

All notable changes to `waka` are documented here.

Format: [Conventional Commits](https://www.conventionalcommits.org/)
Versioning: [Semantic Versioning](https://semver.org/)

## [2.0.1] - 2026-03-02

### 🐛 Bug Fixes

- **update**: Replace cargo install with GitHub Releases download ([`b1c7dbd`](https://github.com/mouwaficbdr/waka/commit/b1c7dbd08091d9c60247238e09f6f53d9d1e51e3))
- **render**: Correct box border width calculation to eliminate visual artifact on right side ([`d7b2856`](https://github.com/mouwaficbdr/waka/commit/d7b285667c815c85fabcd3ae370e8542491c8d1a))

### ⚙️ CI/CD

- Remove invalid [cargo] section from deny.toml ([`fbf32a8`](https://github.com/mouwaficbdr/waka/commit/fbf32a8073a8cab463e3e414a9a99f2dd611808f))

### 🔧 Miscellaneous

- **changelog**: Update CHANGELOG for v2.0.0 ([`6570ad3`](https://github.com/mouwaficbdr/waka/commit/6570ad361f1967bd8fc7debae970b682e2f26d9c))

## [2.0.0] - 2026-03-02

### 🚀 Features

- **stability**: Stabilize public API surface for v1.0 ([`e960c37`](https://github.com/mouwaficbdr/waka/commit/e960c37c2b1517c740414b47818b3ee4eb068848))
- **waka-render**: Add centralized Theme system and render utilities ([`1419853`](https://github.com/mouwaficbdr/waka/commit/1419853e4736493faf5ca6813b9c4043e673e1b0))
- **render**: Redesign stats output with rich visual layout ([`24c6a22`](https://github.com/mouwaficbdr/waka/commit/24c6a2217ccba130ae0244a8356478af2cec3ed5))
- Enrich projects and goals display ([`0e54507`](https://github.com/mouwaficbdr/waka/commit/0e54507613560f9f63c36771c550e3dbb74a3b87))
- **auth**: Redesign auth login with interactive prompt and welcome message ([`3635125`](https://github.com/mouwaficbdr/waka/commit/363512575a3adf3dbad4be432caa3121a384a130))
- **tui**: Improve dashboard loading state, navigation, and sparkline ([`5f3c9fa`](https://github.com/mouwaficbdr/waka/commit/5f3c9fa0de963fff5d453ffe3860f7f508c239f9))

### 🐛 Bug Fixes

- **cli**: Improve Windows path detection for cargo install ([`6923106`](https://github.com/mouwaficbdr/waka/commit/6923106234e6b7f86b5e6b7519f321badd56bbb8))
- **api**: Handle absent 'seconds' field in GrandTotal from WakaTime API ([`ca801d3`](https://github.com/mouwaficbdr/waka/commit/ca801d3cd3f16c29cc42d9840ed19948519411e9))
- **api**: Align types with official WakaTime API documentation ([`11a4d22`](https://github.com/mouwaficbdr/waka/commit/11a4d22e2cfcd4ca738fbe56a96f1c33be357fb3))
- **api**: Align all types with official WakaTime API documentation ([`1e427ab`](https://github.com/mouwaficbdr/waka/commit/1e427ab9e48e69ed1de5a2ce57e56d165e7f252e))
- **api**: Add #[serde(default)] to all optional API fields that may be absent ([`ab922c5`](https://github.com/mouwaficbdr/waka/commit/ab922c55b7075ff453a5342ed954e027588d7a4c))
- **waka-api**: Add crate-specific README, fix crates.io display ([`3ab532a`](https://github.com/mouwaficbdr/waka/commit/3ab532a8fd5fdd9bb350ab6ddecca644b786e1fa))

### 📚 Documentation

- Finalize README for v1.0.0 public release ([`01b244c`](https://github.com/mouwaficbdr/waka/commit/01b244cf9f4a4acc6124b4076105fcd30e87b4e7))
- Add VHS demo tape and recording ([`bd27190`](https://github.com/mouwaficbdr/waka/commit/bd27190d262c371f97391ada04a1bee3489f4b99))
- **cli**: Update example version numbers to v1.0.0/v1.1.0 ([`21d2fe1`](https://github.com/mouwaficbdr/waka/commit/21d2fe1e08b2b1e3abc8a8540067eecf20b5079f))
- Update report --output-format flag, regenerate completions and CHANGELOG ([`f233291`](https://github.com/mouwaficbdr/waka/commit/f2332917c4b44f4e177664a2697357af25781aa9))
- Add generated demo.gif ([`da2bdb7`](https://github.com/mouwaficbdr/waka/commit/da2bdb7e450f990093067d991f381ad896d7b62b))
- Fix crates.io badge URLs from waka-cli to waka-api ([`5676be8`](https://github.com/mouwaficbdr/waka/commit/5676be8b24a0095e8514b5704584560fdecb5e47))
- Fix README badges and remove invalid cargo install command ([`103bdbf`](https://github.com/mouwaficbdr/waka/commit/103bdbfa4253c2c2e960729658e873d0876d82e2))
- Remove non-existent cargo-dist installer, add direct binary download links ([`00fbccf`](https://github.com/mouwaficbdr/waka/commit/00fbccfcf1e33b1ea218db2b95a31335d7647dbc))
- Fix binary download filenames in README ([`ca5cbc2`](https://github.com/mouwaficbdr/waka/commit/ca5cbc22479384d0214d2fbf240497f61105a417))

### ⚙️ CI/CD

- Fix ci-success gate, scope RUSTFLAGS to clippy, remove unimplemented pages workflow ([`547015e`](https://github.com/mouwaficbdr/waka/commit/547015e6e93c202bd2840a84b1623d0f10fc3e72))
- Replace broken cargo-dist reusable workflow with direct multi-platform build ([`d67055b`](https://github.com/mouwaficbdr/waka/commit/d67055b24b05771a16c6608773db2c61f00c3e39))
- Fix publish-crates idempotency and changelog detached HEAD ([`9c1695f`](https://github.com/mouwaficbdr/waka/commit/9c1695fe817caae5fa77beefd307c4e05daa3111))
- Fix audit workflow — cargo-deny-action v2, fix deny.toml deprecated keys, replace rustsec action with direct cargo-audit ([`02f6111`](https://github.com/mouwaficbdr/waka/commit/02f6111562e1567ae8afe9ad510cc61a56b2e907))

### 🔧 Miscellaneous

- Finalize v1.0.0 — update README and mark Phase 4 complete ([`491b2b9`](https://github.com/mouwaficbdr/waka/commit/491b2b98a32b5a939f2aac47939d78b39085ad25))
- Bump version to 1.0.0 ([`135cd03`](https://github.com/mouwaficbdr/waka/commit/135cd0339763048919a2e6640b653c90da340971))
- **ci**: Remove nightly-only rustfmt options to fix fmt CI job ([`c301f4b`](https://github.com/mouwaficbdr/waka/commit/c301f4b6680d2d21aaa352e1aaae161d58b82a9b))
- **release**: Bump version to 2.0.0 ([`52747d2`](https://github.com/mouwaficbdr/waka/commit/52747d23154c0c5a98141bfe42bee4c1baff109a))

## [0.4.0] - 2026-03-01

### 🚀 Features

- **report**: Implement report generation in md/html/json/csv ([`89ef10c`](https://github.com/mouwaficbdr/waka/commit/89ef10c2fb6fa109be6a240403bb55a89f7385b3))
- **cli**: Implement update and changelog commands ([`bc0c368`](https://github.com/mouwaficbdr/waka/commit/bc0c368a100841dfe89ecf751807037f0a336b19))
- **docs**: Generate man pages with clap_mangen ([`1bbf77b`](https://github.com/mouwaficbdr/waka/commit/1bbf77b2506eb50109d11727c9875b606cd0216c))

### 📚 Documentation

- Update README to reflect Phase 2 completion and outline Phase 3 features ([`c47c639`](https://github.com/mouwaficbdr/waka/commit/c47c639cfdea49b5fe10d3e1ac698fbf9114de6b))
- Set up mdBook documentation site ([`9dfa286`](https://github.com/mouwaficbdr/waka/commit/9dfa2862f13980185f82f2638a18182464291967))

### ⚙️ CI/CD

- Configure cargo-dist for multi-platform releases ([`d035b1f`](https://github.com/mouwaficbdr/waka/commit/d035b1f128bf8c50c12bf179b5d110fb2c3d898f))

### 🔧 Miscellaneous

- **api**: Prepare waka-api for crates.io publication ([`bea6198`](https://github.com/mouwaficbdr/waka/commit/bea6198188fc6fc65cd95255ff5374db2b356a4f))
- Bump version to 0.4.0 and mark Phase 3 complete ([`a24013e`](https://github.com/mouwaficbdr/waka/commit/a24013e7bd5e9f7dd605e08cf250500ff1d9f37a))

## [0.3.0] - 2026-03-01

### 🚀 Features

- **prompt**: Implement shell prompt integration command ([`b65abff`](https://github.com/mouwaficbdr/waka/commit/b65abff7833aa5893e4f6618fac8e5ae932cc2ac))
- **cli**: Implement non-blocking update checker ([`3c27711`](https://github.com/mouwaficbdr/waka/commit/3c277117d1cad12541a520d747a8b22d109da16d))
- **goals**: Implement goals list and show commands ([`95d4682`](https://github.com/mouwaficbdr/waka/commit/95d468217b02bb3b287cf2ecda3e0dca3587b33f))
- **goals**: Implement goals watch with system notifications ([`a0e5d98`](https://github.com/mouwaficbdr/waka/commit/a0e5d98b5549848a4c92b024bf5e7123562daaee))
- **leaderboard**: Implement leaderboard command ([`8651e64`](https://github.com/mouwaficbdr/waka/commit/8651e6473b32eed7b208a22e1e5633d441fe5e6f))
- **tui**: Implement ratatui app skeleton with event loop and state management ([`c0aaf77`](https://github.com/mouwaficbdr/waka/commit/c0aaf7733e3e34219cac9b22fc20345e2d2e6e8c))
- **tui**: Implement main dashboard layout with all widgets ([`f874b44`](https://github.com/mouwaficbdr/waka/commit/f874b44cde71360166a99f08af3182d2f85d8ba1))
- **tui**: Implement all dashboard views and keyboard navigation ([`49a3177`](https://github.com/mouwaficbdr/waka/commit/49a317714df72fce86f2b1fc3c6f1392a017a874))
- **tui**: Add offline indicator, manual refresh, export, and resize handling ([`1427001`](https://github.com/mouwaficbdr/waka/commit/14270019e8830e4ffa24a8afa8d19b64c54d3dcf))

### 🐛 Bug Fixes

- **cli**: Standardize all error messages per spec format ([`6b703d9`](https://github.com/mouwaficbdr/waka/commit/6b703d997c8256ebe5b78ea42eacbf466a7d87c5))

### 📚 Documentation

- Update README for Phase 2 completion and add TUI documentation ([`d99ccea`](https://github.com/mouwaficbdr/waka/commit/d99ccea5345004631cde635b86a113d1f44c458c))

## [0.2.0] - 2026-03-01

### 🚀 Features

- **cache**: Implement local cache with sled, TTL, and graceful corruption handling ([`044ad0a`](https://github.com/mouwaficbdr/waka/commit/044ad0a877500a037e87bbe6fa2a9594c7904a50))
- **cache**: Integrate CacheStore into stats command with TTL, stale-if-error ([`9ff6410`](https://github.com/mouwaficbdr/waka/commit/9ff6410bfcef8c8654661a0dfbd99597b4ab73f7))
- **api**: Add projects, stats, goals, and leaderboard endpoints ([`397101d`](https://github.com/mouwaficbdr/waka/commit/397101d010af410486f95baa23698919bfed1152))
- **projects**: Implement projects, languages, and editors commands with BreakdownRenderer ([`7459e73`](https://github.com/mouwaficbdr/waka/commit/7459e73fd3e1f1a6d5e37ed8ea1ce71c1234f342))
- **render**: Add CSV and TSV output formats with --csv-bom support ([`9251d2b`](https://github.com/mouwaficbdr/waka/commit/9251d2b3dec54bc13268a148e8c3998694ed5d1b))
- **config**: Implement multi-profile support with waka auth switch ([`c1b9f8e`](https://github.com/mouwaficbdr/waka/commit/c1b9f8e672e6461a4942633cb5d2900205c25715))
- **render**: Implement TTY detection, NO_COLOR, and TERM=dumb support ([`75e2826`](https://github.com/mouwaficbdr/waka/commit/75e28266c4368d4dcec25172439b23c977290981))
- **completions**: Generate shell completions for bash/zsh/fish/powershell/elvish ([`ec8f2bb`](https://github.com/mouwaficbdr/waka/commit/ec8f2bb81dbdf8fd7d0cf71bb55eda7514207667))
- **cache**: Implement cache management commands ([`f57c2cf`](https://github.com/mouwaficbdr/waka/commit/f57c2cf87293d2b4c2801f3d0d536290c7bebce3))

### 📚 Documentation

- Corriger la mise en forme des exemples de comportement dans le CODE_OF_CONDUCT ([`b608a88`](https://github.com/mouwaficbdr/waka/commit/b608a886e5a2c1dd69a85823b7fbd808d7093f71))
- Update README for Phase 1 completeness and polish docs ([`58526e8`](https://github.com/mouwaficbdr/waka/commit/58526e8b24f6cf11cbd9f6ad34be39869e0f2caf))

## [0.1.0] - 2026-03-01

### 🚀 Features

- **api**: Define response types and error enum ([`7f1f005`](https://github.com/mouwaficbdr/waka/commit/7f1f005e91f7fd50e186a11bb0a5b9a417b3f614))
- **api**: Implement HTTP client with auth, retry, and error handling ([`b667cc6`](https://github.com/mouwaficbdr/waka/commit/b667cc682e22869e0d5806778814257e3de4fc1f))
- **api**: Implement SummaryParams builder and summaries() endpoint ([`3d7483d`](https://github.com/mouwaficbdr/waka/commit/3d7483d9fd2474cc5bccbda17063b7b3bff13c1c))
- **config**: Implement config file load/save with XDG paths ([`33f4f4c`](https://github.com/mouwaficbdr/waka/commit/33f4f4cc81d02fd110761a1d5a6f98049e4b674f))
- **config**: Implement credential store with keychain and env var priority chain ([`7868596`](https://github.com/mouwaficbdr/waka/commit/7868596a21b8c3585bc4eb0d5215bcfac912ccde))
- **cli**: Scaffold full command tree with clap derive ([`640efa0`](https://github.com/mouwaficbdr/waka/commit/640efa005856ef7a7a40dee85b5bb173da58ea60))
- **auth**: Implement auth login, logout, status, and show-key commands ([`00df3c4`](https://github.com/mouwaficbdr/waka/commit/00df3c48bcc4bebfb87a4b5b951bb9dddbdc54bf))
- **render**: Implement summary renderers (table, json, plain) with snapshot tests ([`89ee100`](https://github.com/mouwaficbdr/waka/commit/89ee100647fc829c6d618e97e1c637db63b38896))
- **dependencies**: Add waka-api to Cargo.lock dependencies ([`89a0e00`](https://github.com/mouwaficbdr/waka/commit/89a0e0003c245b9c070c6312c78e401fa5745e0c))
- **config**: Implement doctor diagnostic command ([`248ced8`](https://github.com/mouwaficbdr/waka/commit/248ced8eda3d68a402ca0eaa40d5f5cb2d4cc9d3))

### 📚 Documentation

- Add CODE_OF_CONDUCT and complete Phase 0 polish ([`9e6885f`](https://github.com/mouwaficbdr/waka/commit/9e6885fa06dd0ed2ff91dc4ffccf1ab8b5899956))

### ⚙️ CI/CD

- Add GitHub Actions CI and security audit workflows ([`dcb84d9`](https://github.com/mouwaficbdr/waka/commit/dcb84d95eb2c82d74e0b12ca9361b5045bc0ec46))

### 🔧 Miscellaneous

- Add configuration files for git-cliff, cargo-deny, cargo-dist, and rustfmt ([`050dcb8`](https://github.com/mouwaficbdr/waka/commit/050dcb8201fe921b4d4d60115e72eb647912b959))
- Bootstrap workspace with all crates and tooling config ([`446726b`](https://github.com/mouwaficbdr/waka/commit/446726b782dd78953c3a45cf31d68c46ec4d69d3))

<!-- generated by git-cliff -->
