# Cahier des Charges — `waka`
### The Ultimate WakaTime CLI — Specification v1.0

> *"The developer tool you always deserved."*

---

## Table des matières

1. [Vision & Philosophie](#1-vision--philosophie)
2. [Contexte & Positionnement](#2-contexte--positionnement)
3. [Stack Technique](#3-stack-technique)
4. [Architecture du Projet](#4-architecture-du-projet)
5. [Authentification & Configuration](#5-authentification--configuration)
6. [Commandes & Interface CLI](#6-commandes--interface-cli)
7. [Dashboard TUI Interactif](#7-dashboard-tui-interactif)
8. [Système de Cache & Mode Offline](#8-système-de-cache--mode-offline)
9. [Formats de Sortie & Composabilité Unix](#9-formats-de-sortie--composabilité-unix)
10. [Expérience Utilisateur (UX)](#10-expérience-utilisateur-ux)
11. [Intégrations & Écosystème](#11-intégrations--écosystème)
12. [Standards Open Source](#12-standards-open-source)
13. [Distribution & Packaging](#13-distribution--packaging)
14. [CI/CD & Qualité](#14-cicd--qualité)
15. [Sécurité](#15-sécurité)
16. [Performance & Contraintes Techniques](#16-performance--contraintes-techniques)
17. [Roadmap & Versioning](#17-roadmap--versioning)
18. [Annexes](#18-annexes)

---

## 1. Vision & Philosophie

### 1.1 Énoncé de vision

`waka` est un outil CLI open source, écrit en Rust, qui permet aux développeurs d'interagir avec leur compte WakaTime depuis leur terminal avec une expérience utilisateur irréprochable. Il est conçu pour être **rapide**, **composable**, **agréable à utiliser**, et **hackable** par la communauté.

### 1.2 Principes fondateurs

**Unix philosophy.** Chaque commande fait une chose, la fait bien, et peut être chaînée. `waka stats today --format=json | jq .projects` doit marcher sans friction.

**Zero configuration par défaut.** Après `waka auth login`, tout fonctionne immédiatement sans aucune configuration supplémentaire. Les options avancées sont disponibles mais jamais imposées.

**Respect du terminal de l'utilisateur.** Détection automatique de la couleur, du thème clair/sombre, de la largeur du terminal, de l'environnement CI/pipe. L'outil s'adapte à son contexte — il ne le force pas.

**Offline first.** Un cache local intelligent fait que `waka stats today` fonctionne même sans connexion. Les données périmées sont affichées avec un indicateur clair, jamais en silence.

**Confiance.** L'outil ne collecte aucune donnée. Ne contacte que l'API WakaTime officielle. Est auditables ligne par ligne.

### 1.3 Ce que `waka` n'est pas

- Un remplacement du client WakaTime natif (heartbeats, tracking)
- Une interface web dans le terminal
- Un outil qui nécessite Docker, Node.js ou toute autre dépendance externe
- Un outil propriétaire ou freemium

---

## 2. Contexte & Positionnement

### 2.1 WakaTime API

WakaTime expose une API REST documentée sur `https://wakatime.com/developers`. Les endpoints principaux utilisés :

| Endpoint | Description |
|---|---|
| `GET /users/current` | Profil utilisateur |
| `GET /users/current/summaries` | Résumé sur une période |
| `GET /users/current/stats/{range}` | Stats agrégées (last_7_days, last_30_days…) |
| `GET /users/current/projects` | Liste des projets |
| `GET /users/current/goals` | Objectifs personnels |
| `GET /users/current/leaderboards` | Classements |
| `GET /users/current/durations` | Durées par projet/jour |
| `GET /users/current/heartbeats` | Heartbeats bruts |

Authentification : Basic Auth avec la clé API en base64, ou OAuth2 pour les intégrations tierces.

### 2.2 Outils existants & lacunes

Les outils existants sont soit abandonnés, soit limités à 1-2 commandes, soit ont une UX médiocre (pas de couleur, pas de cache, pas de mode offline, pas d'autocomplétion). `waka` comble ce vide.

---

## 3. Stack Technique

### 3.1 Langage

**Rust (édition 2021, MSRV : 1.82.0)**

Justification : binaire statique, performances natives, pas de garbage collector, excellent support cross-compilation, distribution simplifiée, écosystème CLI mature.

### 3.2 Crates essentielles

```toml
# CLI
clap = { version = "4", features = ["derive", "env", "wrap_help", "color"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP — TLS natif, pas de dépendance OpenSSL
reqwest = { version = "0.13", features = ["json", "gzip", "stream"] }

# Sérialisation
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Gestion des erreurs
anyhow = "1"
thiserror = "2"

# Config & paths XDG
directories = "5"
toml = "0.8"

# TUI
ratatui = "0.30"
crossterm = { version = "0.29", features = ["event-stream"] }

# Tables & output
comfy-table = "7"
indicatif = "0.17"
console = "0.15"
owo-colors = "4"

# Cache
sled = "0.34"

# Dates
chrono = { version = "0.4", features = ["serde"] }

# Markdown (pour les rapports)
termimad = "0.23"

# Notifications système
notify-rust = "4"
```

### 3.3 Toolchain

- **Formatter** : `rustfmt` avec configuration custom
- **Linter** : `clippy` en mode `--deny warnings` en CI
- **Release** : `cargo-dist` pour la distribution multi-plateforme
- **Tests** : `cargo nextest` (plus rapide que `cargo test`)
- **Benchmarks** : `criterion`
- **Audit sécurité** : `cargo audit` en CI

---

## 4. Architecture du Projet

### 4.1 Structure du workspace

```
waka/
├── Cargo.toml                  # Workspace root
├── Cargo.lock
├── README.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── CHANGELOG.md
├── LICENSE                     # MIT
├── dist-workspace.toml         # cargo-dist
├── .cargo/
│   └── config.toml             # target/profile configs
├── .github/
│   ├── workflows/
│   │   ├── ci.yml
│   │   ├── release.yml
│   │   └── audit.yml
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml
│   │   └── feature_request.yml
│   └── PULL_REQUEST_TEMPLATE.md
├── crates/
│   ├── waka/                   # Binaire principal
│   ├── waka-api/               # Client API WakaTime (lib publique)
│   ├── waka-config/            # Config & credentials management
│   ├── waka-cache/             # Cache local
│   ├── waka-tui/               # Dashboard interactif ratatui
│   └── waka-render/            # Renderers (table, json, csv, plain)
├── completions/                # Shell completions générées
│   ├── waka.bash
│   ├── waka.zsh
│   ├── waka.fish
│   └── _waka.ps1
├── man/                        # Man pages
│   └── waka.1
├── docs/                       # Documentation site (mdBook)
│   ├── book.toml
│   └── src/
└── tests/                      # Tests d'intégration workspace
    ├── integration/
    └── fixtures/               # Réponses API mockées (JSON)
```

### 4.2 Responsabilités des crates

#### `waka` (binaire)
Point d'entrée. Parse les arguments clap, dispatche vers les handlers. Ne contient aucune logique métier.

#### `waka-api` (lib publique)
Client HTTP WakaTime. Gère : authentification, retry avec backoff exponentiel, rate limiting (429), pagination, désérialisation des réponses. Publiée sur crates.io pour que d'autres projets puissent l'utiliser.

Interface publique minimale et stable :
```rust
pub struct WakaClient { /* ... */ }

impl WakaClient {
    pub fn new(api_key: &str) -> Self;
    pub async fn summaries(&self, params: SummaryParams) -> Result<SummaryResponse>;
    pub async fn stats(&self, range: Range) -> Result<StatsResponse>;
    pub async fn projects(&self) -> Result<Vec<Project>>;
    pub async fn goals(&self) -> Result<Vec<Goal>>;
    pub async fn me(&self) -> Result<User>;
    // ...
}
```

#### `waka-config`
Gère le fichier de config (`~/.config/waka/config.toml`), le stockage sécurisé des credentials (keychain OS via `keyring` crate ou fallback fichier chiffré), la validation, et les profils multiples.

#### `waka-cache`
Abstraction sur `sled` (embedded key-value store). TTL par entrée. Invalidation manuelle. Serialize/deserialize les réponses API. Mode lecture seule si le store est corrompu (fallback gracieux).

#### `waka-render`
Tous les renderers. Chaque renderer implémente le trait `Render<T>` :
```rust
pub trait Render<T> {
    fn render_table(&self, data: &T, opts: &RenderOptions) -> String;
    fn render_json(&self, data: &T) -> String;
    fn render_csv(&self, data: &T) -> String;
    fn render_plain(&self, data: &T) -> String;
}
```

#### `waka-tui`
Application ratatui autonome. Lancée par `waka dashboard`. Communique avec `waka-api` et `waka-cache` via des channels tokio. Indépendante du reste pour pouvoir être testée et développée isolément.

---

## 5. Authentification & Configuration

### 5.1 Fichier de configuration

Emplacement : `~/.config/waka/config.toml` (XDG Base Directory Specification)

```toml
# ~/.config/waka/config.toml

[core]
default_profile = "default"
update_check = true
telemetry = false           # Toujours false — jamais de collecte de données

[output]
color = "auto"              # auto | always | never
format = "table"            # table | json | csv | plain
date_format = "%Y-%m-%d"
time_format_24h = true

[cache]
enabled = true
ttl_seconds = 300           # 5 minutes par défaut
path = "~/.cache/waka/"     # Override optionnel

[display]
show_progress_bar = true
show_sparklines = true
week_start = "monday"       # monday | sunday

[profiles.default]
# La clé API n'est jamais stockée ici en clair
# Elle est dans le keychain OS ou dans un fichier séparé chiffré
api_url = "https://wakatime.com/api/v1"  # Override pour self-hosted

[profiles.work]
api_url = "https://wakatime.mycompany.com/api/v1"
```

### 5.2 Stockage des credentials

Priorité de résolution de la clé API (ordre de précédence) :

1. `--api-key` flag (CLI)
2. Variable d'environnement `WAKATIME_API_KEY`
3. Variable d'environnement `WAKA_API_KEY`
4. Keychain système (via crate `keyring`) — méthode recommandée
5. Fichier `~/.config/waka/credentials` (permissions 0600, obfusqué en base64)
6. Fichier `~/.wakatime.cfg` (compatibilité avec la config officielle WakaTime)

Le keychain système est utilisé par défaut sur macOS (Keychain), Linux (Secret Service / libsecret), Windows (Credential Manager).

### 5.3 Commandes d'authentification

```
waka auth login             # Prompt interactif pour la clé API, test + sauvegarde
waka auth login --api-key <KEY>  # Non-interactif (scripts, CI)
waka auth logout            # Supprime les credentials du profil actif
waka auth status            # Affiche l'état de connexion sans révéler la clé
waka auth show-key          # Affiche la clé (avec confirmation)
waka auth switch <profile>  # Change de profil actif
```

Flow de `waka auth login` :
1. Prompt pour la clé API (masquée à l'affichage)
2. Test de connexion sur `GET /users/current`
3. Affichage du profil utilisateur confirmant le succès
4. Sauvegarde dans le keychain
5. Message de succès avec next steps

---

## 6. Commandes & Interface CLI

### 6.1 Arborescence complète

```
waka [OPTIONS]
├── auth
│   ├── login [--api-key <KEY>] [--profile <NAME>]
│   ├── logout [--profile <NAME>]
│   ├── status
│   ├── show-key
│   └── switch <PROFILE>
│
├── stats
│   ├── today [OPTIONS]
│   ├── yesterday [OPTIONS]
│   ├── week [OPTIONS]
│   ├── month [OPTIONS]
│   ├── year [OPTIONS]
│   └── range --from <DATE> --to <DATE> [OPTIONS]
│       OPTIONS: --project <NAME> --language <LANG> --format <FMT> --no-cache
│
├── projects
│   ├── list [--sort-by <time|name>] [--limit <N>]
│   ├── top [--period <7d|30d|1y>]
│   └── show <PROJECT_NAME> [--from <DATE> --to <DATE>]
│
├── languages
│   ├── list [--period <7d|30d|1y>]
│   └── top [--limit <N>]
│
├── editors
│   ├── list [--period <7d|30d|1y>]
│   └── top [--limit <N>]
│
├── goals
│   ├── list
│   ├── show <GOAL_ID>
│   └── watch [--notify] [--interval <SECONDS>]
│
├── leaderboard
│   └── show [--page <N>]
│
├── report
│   ├── generate --from <DATE> --to <DATE> [--output <FILE>] [--format <md|html|json|csv>]
│   └── summary [--period <week|month>]
│
├── dashboard                   # Lance le TUI interactif
│   └── [--refresh <SECONDS>]
│
├── prompt                      # Pour intégration dans les prompts shell
│   └── [--format <simple|detailed>]
│
└── config
    ├── get <KEY>
    ├── set <KEY> <VALUE>
    ├── edit                    # Ouvre config dans $EDITOR
    ├── path                    # Affiche le chemin du fichier de config
    ├── reset [--confirm]
    └── doctor                  # Diagnostic complet
```

### 6.2 Options globales (disponibles sur toutes les commandes)

```
-p, --profile <PROFILE>   Utilise un profil spécifique
-f, --format <FORMAT>     Format de sortie : table | json | csv | plain
    --no-cache            Ignore le cache, force une requête fraîche
    --no-color            Désactive les couleurs (équivalent NO_COLOR=1)
    --quiet               Supprime les messages non-essentiels
    --verbose             Mode verbeux (affiche les requêtes HTTP)
    --help                Affiche l'aide contextuelle
    --version             Affiche la version
```

### 6.3 Spécification détaillée des commandes principales

#### `waka stats today`

Affiche un résumé de la journée en cours.

**Comportement :**
- Données récupérées depuis le cache si TTL non expiré
- Si le cache est vide ou expiré : requête API avec spinner
- Indicateur discret si les données viennent du cache : `(cached 3m ago)`

**Sortie par défaut (table) :**
```
┌──────────────────────────────────────────────────────┐
│  Today — Monday, January 13, 2025                    │
│  Total: 6h 42m  ▓▓▓▓▓▓▓▓▓░  Goal: 8h (84%)           │
├──────────────────┬───────────┬──────────────────────┤
│  Project         │  Time     │  Share               │
├──────────────────┼───────────┼──────────────────────┤
│  my-saas         │  3h 12m   │  ███████████ 48%     │
│  wakatime-cli    │  2h 01m   │  ███████     30%     │
│  dotfiles        │    29m    │  ██           7%     │
│  (other)         │  1h 00m   │  ████        15%     │
├──────────────────┴───────────┴──────────────────────┤
│  Languages:  Go (52%)  TypeScript (28%)  Bash (12%)  │
│  Editors:    Neovim (89%)  VS Code (11%)             │
└──────────────────────────────────────────────────────┘
```

#### `waka stats week`

Identique à `today` mais sur 7 jours, avec en plus :
- Sparkline des 7 derniers jours
- Comparaison avec la semaine précédente (+/-%)
- Meilleur jour de la semaine

**Sortie (extrait) :**
```
  Week of Jan 13 – Jan 19, 2025
  Total: 32h 14m   Avg/day: 4h 36m   Best day: Wed (7h 02m)

  Daily activity (last 7 days):
  Mon  ██████░░░░  3h 12m
  Tue  ████████░░  5h 01m
  Wed  ██████████  7h 02m
  Thu  ███░░░░░░░  1h 44m
  Fri  ████████░░  5h 59m
  Sat  ██░░░░░░░░  1h 02m
  Sun  ████████░░  4h 10m (ongoing)
  
  vs last week: +2h 18m (+7.7%) ↑
```

#### `waka stats range --from 2025-01-01 --to 2025-01-31`

Stats sur une plage de dates arbitraire. Même format que `week` mais adapté.

#### `waka goals list`

```
  Goals (3)
  
  ✓  Daily coding    8h / day     Today: 6h 42m  ████████░░  84%
  ✗  Weekly Python   10h / week   This week: 3h  ███░░░░░░░  30%  ← 3 days left
  ✓  Streak          30 days      Current streak: 12 days
```

#### `waka goals watch`

Mode watch interactif (non-TUI) : affiche les objectifs et se rafraîchit périodiquement. Envoie une notification système quand un objectif est atteint.

```
  Watching goals... (refreshing every 5m, Ctrl+C to stop)
  [14:32] Daily: 6h 42m / 8h (84%) ████████░░
  [14:37] Daily: 6h 48m / 8h (85%) ████████░░
```

#### `waka report generate`

Génère un rapport de productivité exportable, idéal pour les freelances et les stand-ups.

**Formats supportés :**
- Markdown (`.md`) — par défaut
- HTML (`.html`) — avec CSS inline, responsive
- JSON (`.json`) — structuré pour intégrations
- CSV (`.csv`) — pour tableurs

**Contenu du rapport :**
- En-tête : période, total, comparaison
- Breakdown par projet (table + graphique ASCII)
- Breakdown par langage
- Breakdown par éditeur
- Activité journalière (sparkline ou table selon format)
- Objectifs atteints sur la période

#### `waka config doctor`

Diagnostic exhaustif qui vérifie :

```
  waka config doctor
  
  ✓  Config file found at ~/.config/waka/config.toml
  ✓  API key found in system keychain
  ✓  API key is valid (authenticated as john@example.com)
  ✓  API reachable (ping: 124ms)
  ✓  Cache directory writable at ~/.cache/waka/
  ✓  Cache database healthy (42 entries, last write: 3m ago)
  ✓  Shell completions installed (zsh)
  ⚠  waka v0.3.1 installed — v0.4.0 available (run: waka update)
  ✓  No known issues
```

#### `waka prompt`

Conçu pour être appelé dans un prompt shell ou Starship. Sortie minimale et rapide (utilise uniquement le cache, ne fait jamais de requête réseau).

```bash
$ waka prompt
⏱ 6h 42m

$ waka prompt --format detailed
⏱ 6h 42m | my-saas
```

---

## 7. Dashboard TUI Interactif

### 7.1 Vue d'ensemble

`waka dashboard` lance une application TUI complète basée sur ratatui/crossterm. Elle remplace avantageusement la consultation répétée de `waka stats today`.

### 7.2 Layout

```
┌─ waka dashboard ──────────────────────────────────────── q:quit r:refresh ?:help ─┐
│                                                                                     │
│  ┌─ Today ──────────────────────────┐  ┌─ This Week ──────────────────────────┐   │
│  │  6h 42m  ███████████░░░  84%     │  │  32h 14m   Avg: 4h 36m              │   │
│  │  Goal: 8h   Streak: 12 days 🔥   │  │  ▁▃▅▇▄▂█  Mon-Sun                  │   │
│  └──────────────────────────────────┘  └─────────────────────────────────────┘   │
│                                                                                     │
│  ┌─ Top Projects ────────────────────────────────────────────────────────────────┐ │
│  │  my-saas         ████████████████████░░░░░░░░░░  3h 12m  48%               │ │
│  │  wakatime-cli    █████████████░░░░░░░░░░░░░░░░░  2h 01m  30%               │ │
│  │  dotfiles        ████░░░░░░░░░░░░░░░░░░░░░░░░░░    29m   7%                │ │
│  └────────────────────────────────────────────────────────────────────────────┘ │
│                                                                                     │
│  ┌─ Languages ────────────────────┐  ┌─ Goals ─────────────────────────────────┐  │
│  │  Go          52%  ██████████  │  │  ✓ Daily    6h 42m / 8h   ████████░░   │  │
│  │  TypeScript  28%  █████░░░░░  │  │  ✗ Python   3h / 10h      ███░░░░░░░   │  │
│  │  Bash        12%  ██░░░░░░░░  │  │  ✓ Streak   12 days                    │  │
│  │  YAML         8%  █░░░░░░░░░  │  └────────────────────────────────────────┘  │
│  └────────────────────────────────┘                                               │
│                                                                                     │
│  ┌─ Activity (last 30 days) ──────────────────────────────────────────────────────┐│
│  │  ░▁▂▃▄▅▆▇█▇▆▅▄▃▂▁░▁▂▃▄▅▆▇█▇▆▅  Dec 13 ──────────────────────── Jan 13      ││
│  └────────────────────────────────────────────────────────────────────────────────┘│
│                                                                                     │
│  Last updated: 14:32:01  ·  Auto-refresh in 4m 23s  ·  Tab: switch view            │
└─────────────────────────────────────────────────────────────────────────────────────┘
```

### 7.3 Vues disponibles (navigation par Tab)

| Touche | Vue |
|---|---|
| `Tab` / `1` | Vue principale (layout ci-dessus) |
| `2` | Vue projets (détail + historique 30j) |
| `3` | Vue langages (breakdown complet) |
| `4` | Vue objectifs (goals détaillés) |
| `5` | Vue activité (calendrier de contribution style GitHub) |

### 7.4 Raccourcis clavier

| Touche | Action |
|---|---|
| `q` / `Esc` | Quitter |
| `r` | Rafraîchir immédiatement |
| `Tab` | Changer de vue |
| `?` | Afficher l'aide |
| `↑↓` | Naviguer dans les listes |
| `Enter` | Détail de l'élément sélectionné |
| `c` | Vider le cache |
| `e` | Exporter la vue courante |

### 7.5 Comportement de rafraîchissement

- Intervalle configurable (`--refresh <SECONDS>`, défaut : 300)
- Timer visible dans la barre de statut
- Le rafraîchissement se fait en background (pas de freeze de l'UI)
- Indicateur de loading pendant la requête
- Si la requête échoue : garde les données précédentes + affiche un badge `⚠ offline`

---

## 8. Système de Cache & Mode Offline

### 8.1 Architecture du cache

Basé sur `sled` (embedded B-tree database). Une seule base de données par profil, stockée dans `~/.cache/waka/<profile>/db`.

**Clés de cache :**
```
summaries:2025-01-13          → SummaryResponse (TTL: 5min)
summaries:2025-01-13:my-saas  → SummaryResponse filtrée (TTL: 5min)
stats:last_7_days             → StatsResponse (TTL: 15min)
stats:last_30_days            → StatsResponse (TTL: 1h)
projects                      → Vec<Project> (TTL: 1h)
goals                         → Vec<Goal> (TTL: 5min)
me                            → User (TTL: 24h)
leaderboard:0                 → Leaderboard page 0 (TTL: 30min)
```

### 8.2 Comportement

**Cache hit (TTL valide) :** Retourne les données immédiatement. Affiche `(cached Xm ago)` en gris si `--verbose` ou dans le TUI.

**Cache hit (TTL expiré) :** Lance une requête en background, affiche les données périmées immédiatement avec un badge `⟳`, met à jour quand la réponse arrive.

**Cache miss :** Affiche un spinner, fait la requête, retourne les données et les met en cache.

**Mode `--no-cache` :** Ignore complètement le cache, force une requête fraîche, ne met pas en cache le résultat.

**Erreur réseau avec cache disponible :** Affiche les données en cache avec un badge `⚠ offline — showing cached data from <date>`. Ne retourne jamais d'erreur si des données cachées existent.

**Erreur réseau sans cache :** Message d'erreur clair avec suggestion (`waka auth doctor`).

### 8.3 Gestion du cache

```bash
waka cache clear              # Vide tout le cache
waka cache clear --older 7d   # Vide les entrées > 7 jours
waka cache info               # Affiche taille, nombre d'entrées, dernière écriture
waka cache path               # Affiche le chemin du cache
```

---

## 9. Formats de Sortie & Composabilité Unix

### 9.1 Principe

Chaque commande supportant un output supporte `--format <FORMAT>`. La détection automatique du contexte s'applique :

- Si stdout est un TTY → format `table` avec couleurs par défaut
- Si stdout est un pipe ou un fichier → format `plain` sans couleurs par défaut
- La variable `NO_COLOR` est respectée (standard https://no-color.org)
- `--format` override toujours la détection automatique

### 9.2 Formats disponibles

**`table`** : Tableau ASCII enrichi avec couleurs, barres de progression, sparklines. Uniquement pour affichage humain.

**`plain`** : Texte brut, tabulé, sans couleurs. Pour grep, awk, etc.

**`json`** : JSON complet de la réponse API normalisée. Stable entre versions (champs additionnels possibles, champs existants jamais supprimés). Avec `--pretty` pour indentation.

**`csv`** : CSV avec en-têtes. Encodage UTF-8 avec BOM optionnel (`--csv-bom`).

**`tsv`** : Variante TSV du CSV, pour les outils qui préfèrent les tabulations.

### 9.3 Exemples de composabilité

```bash
# Total en secondes pour un script
waka stats today --format=json | jq .data.grand_total.total_seconds

# Export CSV d'une semaine pour un tableur
waka stats week --format=csv > rapport_semaine.csv

# Utilisation dans un prompt Zsh
WAKA=$(waka prompt 2>/dev/null) && echo "$WAKA"

# Intégration dans un Makefile
@echo "Coding time today: $(waka stats today --format=plain | grep Total)"

# Filtrer les projets > 1h
waka projects list --format=json | jq '[.[] | select(.total_seconds > 3600)]'
```

---

## 10. Expérience Utilisateur (UX)

### 10.1 Premier lancement

Si `waka` est lancé sans configuration, le message d'accueil guide l'utilisateur :

```
  Welcome to waka! 👋

  It looks like you haven't set up your WakaTime API key yet.
  Let's get you started in 30 seconds.

  Run: waka auth login

  Your API key can be found at: https://wakatime.com/settings/api-key
```

### 10.2 Messages d'erreur

Les erreurs sont claires, actionnables, et ne contiennent jamais de stack trace en mode normal.

**Format des erreurs :**
```
  Error: Could not connect to WakaTime API
  
  Reason: Request timed out after 10 seconds
  
  Try:
    · Check your internet connection
    · Run `waka config doctor` for a full diagnostic
    · Use `--no-cache` to bypass the cache
  
  If the problem persists: https://github.com/you/waka/issues
```

Mode `--verbose` : affiche la trace complète, les en-têtes HTTP, les timings.

### 10.3 Indicateurs de progression

Toute opération > 200ms affiche un spinner animé :
```
  ⠋ Fetching stats from WakaTime API...
```

Remplacé par le résultat ou un message d'erreur. Jamais de commande qui "raccroche" sans feedback.

### 10.4 Aide contextuelle

**`--help`** sur chaque commande est soigné, avec exemples :

```
waka stats today — Show today's coding activity

USAGE:
    waka stats today [OPTIONS]

OPTIONS:
    -f, --format <FORMAT>     Output format [table|json|csv|plain] [default: table]
        --no-cache            Bypass cache and fetch fresh data
    -p, --project <PROJECT>   Filter by project name
        --help                Print help information

EXAMPLES:
    waka stats today
    waka stats today --format json | jq .grand_total
    waka stats today --project my-saas
```

### 10.5 Autocomplétion shell

Des completions sont générées et distribuées pour bash, zsh, fish, et PowerShell. Elles couvrent :
- Les sous-commandes
- Les flags et leurs valeurs possibles
- Les noms de projets (chargés dynamiquement depuis le cache)
- Les profils configurés

Installation :
```bash
waka completions bash >> ~/.bash_completion
waka completions zsh > ~/.zsh/completions/_waka
waka completions fish > ~/.config/fish/completions/waka.fish
```

### 10.6 Mise à jour automatique

`waka` vérifie une fois par jour si une nouvelle version est disponible (requête sur GitHub Releases API, sans telemetry). Si c'est le cas, il affiche un message discret en bas de l'output :

```
  ─────────────────────────────────────────────
  ⬆  waka v0.4.0 is available (you have v0.3.1)
     Update: waka update  ·  Changelog: waka changelog
```

La vérification est désactivable via `update_check = false` dans la config ou `WAKA_NO_UPDATE_CHECK=1`.

```bash
waka update        # Met à jour waka via cargo ou le gestionnaire de paquets détecté
waka changelog     # Affiche le CHANGELOG depuis la version installée
```

### 10.7 Respect des conventions système

| Convention | Comportement |
|---|---|
| `NO_COLOR=1` | Désactive toutes les couleurs |
| `TERM=dumb` | Mode plain automatique |
| Redirection stdout | Détecté, colors off, format plain |
| `PAGER` | Utilisé pour les outputs longs |
| `EDITOR` | Utilisé par `waka config edit` |
| `XDG_CONFIG_HOME` | Respecté pour le chemin de config |
| `XDG_CACHE_HOME` | Respecté pour le chemin du cache |
| Signal `SIGTERM`/`SIGINT` | TUI fermé proprement, cursor restauré |

---

## 11. Intégrations & Écosystème

### 11.1 Intégration Starship

Le prompt `waka prompt` est conçu pour s'intégrer dans [Starship](https://starship.rs/) via un module custom :

```toml
# ~/.config/starship.toml
[custom.waka]
command = "waka prompt --format simple"
when = "true"
format = "[$output]($style) "
style = "dimmed yellow"
```

### 11.2 Intégration tmux

Afficher le temps de coding dans la barre tmux :

```bash
# ~/.tmux.conf
set -g status-right "#(waka prompt 2>/dev/null) | %H:%M"
```

### 11.3 Intégration VS Code / Neovim / autres

Documentation et snippet pour afficher les stats dans la barre de statut via un terminal intégré ou un plugin dédié.

### 11.4 Webhooks & Automatisation

```bash
# Rapport automatique chaque vendredi (cron)
0 18 * * 5 waka report generate --from $(date -d "last monday" +%Y-%m-%d) \
            --to $(date +%Y-%m-%d) --format md \
            --output ~/reports/week-$(date +%V).md
```

### 11.5 API publique (`waka-api` crate)

La crate `waka-api` est publiée séparément sur crates.io avec :
- Documentation complète sur docs.rs
- Exemples dans `examples/`
- CHANGELOG propre
- Versioning sémantique strict

---

## 12. Standards Open Source

### 12.1 Licence

**MIT License** — la plus permissive et compatible avec l'écosystème Rust.

### 12.2 Documentation

**README.md** doit contenir :
- Badge : version, CI status, downloads, license
- Screenshot/GIF du dashboard TUI
- Installation (toutes méthodes)
- Quick start (3 commandes pour être opérationnel)
- Lien vers la documentation complète
- Section Contributing
- Code of Conduct

**CONTRIBUTING.md** doit couvrir :
- Prérequis de développement
- Architecture du projet
- Comment lancer les tests
- Convention de commits (Conventional Commits)
- Processus de PR
- Comment ajouter une commande
- Comment mettre à jour les completions
- Comment mettre à jour la documentation

**Documentation complète** : mdBook hébergé sur GitHub Pages. Couvre :
- Installation
- Configuration complète
- Toutes les commandes avec exemples
- Intégrations
- FAQ

### 12.3 Gestion des issues et PRs

Templates d'issue :
- `bug_report.yml` : version, OS, commande exécutée, output attendu, output obtenu
- `feature_request.yml` : problème rencontré, solution proposée, alternatives considérées

Template de PR :
- Description du changement
- Type : bug fix / feature / refactor / docs
- Tests ajoutés/modifiés
- Breaking change (oui/non)
- Checklist qualité

### 12.4 Convention de commits

[Conventional Commits](https://www.conventionalcommits.org/) obligatoire :

```
feat(stats): add sparkline to weekly view
fix(cache): handle corrupted sled database gracefully
docs(contributing): add section on adding new commands
chore(deps): update reqwest to 0.13.2
refactor(render): extract bar chart logic into module
test(api): add integration tests for summaries endpoint
```

Enforced par un hook pre-commit et en CI.

### 12.5 Versioning

[Semantic Versioning 2.0](https://semver.org/) strict.

- **PATCH** : bug fixes, corrections de typos, mises à jour de dépendances non-breaking
- **MINOR** : nouvelles commandes, nouvelles options, nouvelles features sans breaking change
- **MAJOR** : changements de l'interface CLI, du format de config, ou de l'API publique `waka-api`

Un fichier `CHANGELOG.md` est maintenu automatiquement via `git-cliff` ou `release-please`, basé sur les conventional commits.

### 12.6 Code of Conduct

[Contributor Covenant v2.1](https://www.contributor-covenant.org/) adopté tel quel.

Contact pour les incidents : maintenu dans `CODE_OF_CONDUCT.md`.

---

## 13. Distribution & Packaging

### 13.1 Méthodes d'installation

**Cargo (développeurs Rust) :**
```bash
cargo install waka-cli
```

**Homebrew (macOS/Linux) :**
```bash
brew tap <author>/waka
brew install waka
```

**Script d'installation universel :**
```bash
curl -sSfL https://raw.githubusercontent.com/<author>/waka/main/install.sh | sh
```

**Packages système (via GoReleaser/cargo-dist) :**
```bash
# Debian/Ubuntu
sudo apt install waka              # si le PPA est configuré
sudo dpkg -i waka_0.4.0_amd64.deb

# Arch Linux
yay -S waka                        # AUR
```

**Scoop (Windows) :**
```bash
scoop bucket add <author> https://github.com/<author>/scoop-bucket
scoop install waka
```

**Nix :**
```nix
# flake.nix
inputs.waka.url = "github:<author>/waka";
```

**GitHub Releases :** Binaires pré-compilés pour toutes les plateformes.

### 13.2 Plateformes cibles

| Cible | Support |
|---|---|
| `x86_64-unknown-linux-gnu` | ✓ Tier 1 |
| `x86_64-unknown-linux-musl` | ✓ Tier 1 (statique) |
| `aarch64-unknown-linux-gnu` | ✓ Tier 1 (ARM64) |
| `x86_64-apple-darwin` | ✓ Tier 1 |
| `aarch64-apple-darwin` | ✓ Tier 1 (Apple Silicon) |
| `x86_64-pc-windows-msvc` | ✓ Tier 1 |
| `x86_64-pc-windows-gnu` | ✓ Tier 2 |
| FreeBSD | ✓ Tier 2 |

### 13.3 Taille des binaires

Cible : binaire < 10 MB, idéalement < 5 MB (strip + LTO en release).

Configuration `Cargo.toml` :
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"
```

---

## 14. CI/CD & Qualité

### 14.1 Pipeline CI (GitHub Actions)

**`ci.yml`** — déclenché sur chaque push et PR :

```yaml
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - cargo fmt --check
      - cargo clippy -- -D warnings
      - cargo nextest run
      - cargo test --doc

  audit:
    steps:
      - cargo audit
      - cargo deny check

  coverage:
    steps:
      - cargo llvm-cov --lcov → codecov
```

**`release.yml`** — déclenché sur un tag `v*.*.*` :

```yaml
jobs:
  build:
    uses: cargo-dist
    # Génère les binaires pour toutes les plateformes
    # Crée la GitHub Release
    # Publie la Homebrew formula
    # Publie sur crates.io
```

### 14.2 Qualité du code

- `clippy` en `--deny warnings` : zéro warning toléré en CI
- `rustfmt` : format uniforme, pas de débat de style
- `cargo deny` : audit des licences des dépendances et des advisories de sécurité
- Coverage minimal : 70% de couverture de code (indicatif, pas bloquant)

### 14.3 Tests

**Tests unitaires** : dans chaque module, `#[cfg(test)]`. Couvrent la logique de parsing, rendu, cache, config.

**Tests d'intégration** : dans `tests/integration/`. Utilisent un serveur HTTP mock (crate `wiremock`) avec des fixtures JSON réelles de l'API WakaTime anonymisées.

**Tests de snapshot** : pour les renderers (table, JSON), utiliser `insta` pour détecter les régressions visuelles.

**Tests de bout en bout** : sur les platforms CI, valident que le binaire compilé répond correctement à des commandes simples.

---

## 15. Sécurité

### 15.1 Stockage des credentials

- Jamais de clé API en clair dans les logs ou les messages d'erreur
- Masquage de la clé dans `--verbose` : `Authorization: Basic ****...****`
- Préférence pour le keychain système
- Fichier de credentials avec permissions 0600 enforced au démarrage

### 15.2 Réseau

- TLS exclusivement via rustls (pas d'OpenSSL, pas de dépendance système)
- Vérification du certificat TLS activée et non contournable
- Timeout réseau : 10 secondes par requête
- Retry limité : 3 tentatives maximum avec backoff exponentiel
- Seul `wakatime.com` (ou le `api_url` configuré) est contacté

### 15.3 `SECURITY.md`

Politique de divulgation responsable claire :
- Canal de contact (email ou GitHub private advisory)
- Délai de réponse garanti : 72h
- Processus de correction et publication du fix
- Hall of fame des contributeurs sécurité

### 15.4 Dépendances

- `cargo audit` en CI bloque en cas d'advisory RUSTSEC non résolu
- `cargo deny` vérifie les licences (MIT, Apache-2.0, BSD autorisés)
- Mises à jour via Dependabot (PR automatiques hebdomadaires)

---

## 16. Performance & Contraintes Techniques

### 16.1 Temps de démarrage

Cible : **< 50ms** au démarrage à froid sur un CPU moderne.
Cible : **< 100ms** pour une commande avec cache hit.
Cible : **< 2s** pour une commande nécessitant une requête réseau (hors latence réseau).

Profiling régulier avec `hyperfine` :
```bash
hyperfine 'waka stats today --no-cache' 'waka stats today'
```

### 16.2 Consommation mémoire

Cible : **< 20 MB RSS** pour les commandes simples.
Cible : **< 50 MB RSS** pour le dashboard TUI en fonctionnement.

### 16.3 Comportement réseau

- Une seule requête HTTP par commande sauf cas justifiés
- Pas de requête en arrière-plan sans consentement explicite de l'utilisateur
- Pas de telemetry, analytics, ni collecte de données de quelque nature que ce soit

---

## 17. Roadmap & Versioning

### Phase 0 — Fondations (v0.1.0)

- [ ] Bootstrap workspace Cargo multi-crates
- [ ] `waka auth login/logout/status`
- [ ] `waka-api` : endpoint summaries et stats basiques
- [ ] `waka stats today/week/month` avec format table
- [ ] `waka config get/set/doctor`
- [ ] Cache basique (sled)
- [ ] Tests unitaires et d'intégration basiques
- [ ] CI GitHub Actions
- [ ] README fonctionnel

### Phase 1 — Complétude (v0.2.0)

- [ ] `waka stats yesterday/year/range`
- [ ] `waka projects list/top/show`
- [ ] `waka languages list/top`
- [ ] `waka editors list/top`
- [ ] Formats `json`, `csv`, `plain`
- [ ] Détection TTY / pipe
- [ ] Cache TTL avancé et mode offline
- [ ] Autocomplétion bash/zsh/fish
- [ ] Gestion des profils multiples

### Phase 2 — UX premium (v0.3.0)

- [ ] Dashboard TUI (`waka dashboard`)
- [ ] `waka goals list/watch` avec notifications
- [ ] `waka leaderboard`
- [ ] Messages d'erreur enrichis
- [ ] `waka prompt` pour intégration shell
- [ ] Vérification de mise à jour
- [ ] Profiling et optimisation démarrage
- [ ] Documentation complète (mdBook)

### Phase 3 — Écosystème (v0.4.0)

- [ ] `waka report generate` (md/html/json/csv)
- [ ] `waka update` et `waka changelog`
- [ ] Man pages
- [ ] Homebrew tap + Scoop bucket
- [ ] Publication `waka-api` sur crates.io
- [ ] Intégrations documentées (Starship, tmux, etc.)
- [ ] Package Debian/RPM via cargo-dist
- [ ] Nix flake

### Phase 4 — Stabilité v1.0 (v1.0.0)

- [ ] Stabilisation de l'interface CLI (aucun breaking change futur sans major bump)
- [ ] Stabilisation de l'API publique `waka-api`
- [ ] 80%+ de couverture de tests
- [ ] Audit de sécurité externe
- [ ] Support Windows testé et validé
- [ ] Site de documentation

---

## 18. Annexes

### A. Variables d'environnement

| Variable | Description |
|---|---|
| `WAKATIME_API_KEY` | Clé API (priorité haute) |
| `WAKA_API_KEY` | Alias de WAKATIME_API_KEY |
| `WAKA_PROFILE` | Profil à utiliser |
| `WAKA_FORMAT` | Format de sortie par défaut |
| `WAKA_NO_CACHE` | Désactive le cache si `1` |
| `WAKA_NO_UPDATE_CHECK` | Désactive la vérification de mise à jour |
| `WAKA_CONFIG_DIR` | Override du répertoire de config |
| `WAKA_CACHE_DIR` | Override du répertoire de cache |
| `NO_COLOR` | Standard, désactive les couleurs |

### B. Codes de sortie (exit codes)

| Code | Signification |
|---|---|
| `0` | Succès |
| `1` | Erreur générique |
| `2` | Erreur d'utilisation (mauvais argument) |
| `3` | Erreur d'authentification |
| `4` | Erreur réseau |
| `5` | Erreur de configuration |
| `6` | Données introuvables |

### C. Fichiers créés par `waka`

```
~/.config/waka/
├── config.toml         # Configuration principale
└── credentials         # Fallback credentials (si pas de keychain)

~/.cache/waka/
└── <profile>/
    └── db/             # Base sled (plusieurs fichiers)
```

### D. Compatibilité WakaTime

`waka` lit le fichier `~/.wakatime.cfg` (config officielle du client WakaTime) pour en extraire la clé API si aucune autre source n'est disponible. Il ne modifie jamais ce fichier.

### E. Nom du binaire et nom du crate

- Binaire publié : `waka`
- Crate Cargo : `waka-cli` (car `waka` est potentiellement déjà pris sur crates.io)
- Repository GitHub : `waka` ou `waka-cli`

---

*Document rédigé pour `waka` v0.1.0 — maintenu avec le code source dans `docs/SPEC.md`*
*Toute modification de l'interface CLI ou de l'API publique doit être reflétée ici avant implémentation.*
