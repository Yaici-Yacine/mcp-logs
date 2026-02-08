# MCP Logs - Système de capture de logs en temps réel

Système complet de capture et analyse de logs en temps réel pour projets de développement, avec communication via Unix socket entre un CLI Rust et un serveur MCP Bun.

## ✨ Fonctionnalités

- 🎨 **Logs colorisés** : Erreurs en rouge, warnings en jaune, debug en bleu
- 🔕 **Mode silencieux** : Logs verbeux désactivés par défaut
- 🚀 **Multi-agents** : Lancez plusieurs agents simultanément pour monitorer plusieurs projets
- 📊 **Outils MCP** : 9 outils pour interroger et analyser vos logs
- 🔄 **Restart à distance** : Redémarrez vos processus via l'IA (nouveau !)
- 📈 **Analytics avancées** : Distribution, timeline, messages fréquents, taux d'erreurs
- ⏰ **Filtrage temporel** : Filtrez par plage de temps (ISO 8601, timestamps, temps relatifs)
- 🔍 **Recherche regex** : Patterns avancés pour requêtes complexes
- 🔌 **Unix Socket** : Communication rapide et locale

---

## 📦 Installation

### Méthode 1 : Installation depuis les registres officiels (recommandé)

#### 1. Installer le CLI Rust

```bash
# Via Cargo (crates.io)
cargo install mcp-log-agent
```

Le binaire `mcp-log-agent` sera installé dans `~/.cargo/bin/` (assurez-vous que ce chemin est dans votre `$PATH`).

#### 2. Installer le serveur MCP

```bash
# Via NPM (npm registry)
npm install -g mcp-logs

# Ou avec Bun
bun install -g mcp-logs

# Ou avec pnpm
pnpm install -g mcp-logs
```

Le serveur sera installé globalement et accessible via la commande `mcp-logs`.

---

### Méthode 2 : Installation depuis les sources

#### 1. Installer le CLI Rust

```bash
# Depuis le dossier du projet
cd log-agent
cargo install --path .
```

**Alternative : Build sans installation**
```bash
cd log-agent
cargo build --release
# Le binaire sera dans ./target/release/mcp-log-agent
```

#### 2. Installer le serveur MCP

```bash
cd mcp-logs
npm install -g .
# ou avec bun
bun install -g .
```

**Alternative : Utilisation sans installation**
```bash
cd mcp-logs
bun install
# Puis lancer avec: bun run index.ts
```

---

## 🚀 Utilisation

### Démarrage rapide

#### 1. Configurer le serveur MCP dans votre client

Le serveur MCP doit être configuré dans votre client MCP (OpenCode, Claude Desktop, Cline, etc.). Choisissez votre client ci-dessous :

##### Pour OpenCode

Éditez `~/.config/opencode/mcp.json` :

**Si installé globalement (recommandé) :**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```
ou
```json
{
  "mcpServers": {
    "mcp-logs": {
      "type": "local",
      "enabled": true,
      "command": ["bun","x","mcp-logs@latest"]
    }
  }
}
```

**Si utilisé depuis les sources :**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/chemin/absolu/vers/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

##### Pour Claude Desktop

Éditez `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS) ou `%APPDATA%\Claude\claude_desktop_config.json` (Windows) :

**Si installé globalement (recommandé) :**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**Si utilisé depuis les sources :**
```json
{
  "mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/chemin/absolu/vers/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

##### Pour Cline (VSCode)

Éditez les paramètres Cline dans VSCode (`settings.json`) :

**Si installé globalement (recommandé) :**
```json
{
  "cline.mcpServers": {
    "mcp-logs": {
      "command": "mcp-logs"
    }
  }
}
```

**Si utilisé depuis les sources :**
```json
{
  "cline.mcpServers": {
    "mcp-logs": {
      "command": "bun",
      "args": ["run", "/chemin/absolu/vers/mcp-log/mcp-logs/index.ts"],
      "env": {
        "VERBOSE": "false"
      }
    }
  }
}
```

> **Note :** Assurez-vous que `bun` est installé et accessible dans votre PATH, car le package `mcp-logs` nécessite Bun pour fonctionner.

#### 2. Redémarrer votre client MCP

Après avoir modifié la configuration, redémarrez votre client (OpenCode, Claude Desktop, Cline, etc.) pour que le serveur MCP soit chargé.

#### 3. Vérifier que le serveur MCP est connecté

Dans votre client MCP, vous devriez maintenant voir les outils suivants disponibles :
- `get_recent_logs`
- `get_logs`
- `search_logs`
- `get_errors`
- `get_stats`
- `list_projects`
- `clear_logs`

#### 4. Configurer votre projet (Recommandé)

Créez une configuration locale dans le répertoire de votre projet :

```bash
cd votre-projet
mcp-log-agent config init --local
```

Éditez `.mcp-log-agent.toml` pour définir votre commande par défaut :

```toml
[agent]
default_project = "mon-app"
# Décommentez et configurez votre commande :
default_command = ["npm", "start"]
# Ou : default_command = ["bun", "dev"]
# Ou : default_command = ["cargo", "run"]
```

#### 5. Lancer votre application avec l'agent

**Méthode simple (avec config locale) :**
```bash
# Lancez simplement sans arguments !
mcp-log-agent run
```

**Méthode traditionnelle (sans config) :**
```bash
# Spécifiez la commande directement
mcp-log-agent run --project my-app -- npm run dev
```

> **Note:** Le séparateur `--` est nécessaire pour séparer les options de mcp-log-agent de votre commande.

Vos logs s'affichent maintenant dans le terminal ET sont capturés par le serveur MCP.

#### 6. Analyser les logs via MCP

Dans votre client MCP (OpenCode, Claude, Cline), utilisez les outils disponibles :

**Exemples de requêtes :**
```
Montre-moi les projets connectés
Montre les 100 derniers logs
Recherche "error" dans les logs du projet "my-app"
Quelles sont les dernières erreurs ?
```

Le client MCP appellera automatiquement les outils appropriés (`list_projects`, `get_recent_logs`, `search_logs`, `get_errors`, etc.).

---

## 📖 Exemples d'utilisation

### Workflow simple avec configuration locale

```bash
# 1. Configuration initiale (une fois par projet)
cd mon-projet
mcp-log-agent config init --local

# 2. Éditez .mcp-log-agent.toml
# Décommentez: default_command = ["npm", "start"]

# 3. Lancez simplement (à chaque fois)
mcp-log-agent run
```

### Surveiller une application Next.js

**Avec config:**
```bash
cd nextjs-app
mcp-log-agent config init --local
# Configurez: default_command = ["npm", "run", "dev"]
mcp-log-agent run
```

**Sans config:**
```bash
mcp-log-agent run --project nextjs-app -- npm run dev
```

### Capturer les logs de plusieurs projets

**Terminal 1 - Frontend :**
```bash
cd frontend
mcp-log-agent run  # utilise default_command de la config locale
```

**Terminal 2 - Backend :**
```bash
cd backend
mcp-log-agent run  # utilise default_command de la config locale
```

**Terminal 3 - API :**
```bash
cd api
mcp-log-agent run  # utilise default_command de la config locale
```

Les logs de tous les projets seront capturés simultanément et différenciables par leur nom.

### Analyser les logs via MCP

Dans votre client MCP, vous pouvez poser des questions en langage naturel :

```bash
# Exemples de requêtes en langage naturel
"Montre-moi les derniers logs"
"Quelles sont les erreurs dans le projet frontend ?"
"Recherche 'database' dans tous les logs"
"Affiche les statistiques des logs"
"Liste tous les projets connectés"
```

Ou utiliser directement les outils MCP avec leurs paramètres :

```bash
list_projects                              # Voir tous les agents connectés
get_recent_logs { "count": 50 }           # Derniers 50 logs
get_logs { "project": "frontend" }        # Logs du frontend uniquement
get_errors { "project": "backend" }       # Erreurs du backend
search_logs { "query": "database" }       # Rechercher "database"
get_stats                                  # Statistiques globales
```

---

## 🛠️ Outils MCP disponibles

| Outil | Description | Paramètres |
|-------|-------------|------------|
| `get_recent_logs` | Récupère les derniers logs | `count` (optionnel, max 500) |
| `get_logs` | Filtrage avancé avec plages temporelles | `project`, `level`, `source`, `search`, `startTime`, `endTime`, `limit` |
| `search_logs` | Recherche textuelle ou regex | `query` (requis), `regex`, `project`, `limit` |
| `get_errors` | Logs de niveau erreur uniquement | `project`, `limit` |
| `get_stats` | Statistiques globales | - |
| `get_analytics` | Analytics avancées | `timeRange`, `project`, `groupBy`, `startTime`, `endTime` |
| `list_projects` | Liste des agents connectés | - |
| `clear_logs` | Vide la mémoire | - |
| `restart_process` | **🔄 NOUVEAU** - Redémarre un processus surveillé | `project` (requis) |

### 🔄 Nouveau : Restart à distance via IA

L'IA peut maintenant redémarrer vos processus directement via MCP !

**Cas d'usage :**
- Détection automatique d'erreurs → restart
- Application de changements de config
- Gestion de fuites mémoire
- Orchestration multi-projets

**Exemple :**
```
User: J'ai modifié le fichier .env
AI: Je vais redémarrer l'application pour appliquer les changements.
    → Appelle restart_process { "project": "my-app" }
    ✓ Process restarted successfully (PID: 12345)
```

**Documentation complète :** Voir [RESTART-FEATURE.md](./RESTART-FEATURE.md)

### Exemples de requêtes

```json
// Récupérer les 100 derniers logs
{
  "count": 100
}

// Filtrer par projet et niveau
{
  "project": "frontend",
  "level": "error",
  "limit": 50
}

// Filtrer par plage temporelle
{
  "startTime": "last 1h",
  "level": "error"
}

// Recherche avec regex
{
  "query": "error.*timeout",
  "regex": true
}

// Analytics avancées
{
  "timeRange": "24h",
  "groupBy": "hour"
}

// Rechercher dans tous les projets
{
  "query": "connection timeout",
  "limit": 20
}
```

---

## 🎨 Colorisation des logs

Les logs sont automatiquement colorisés dans le terminal selon leur niveau :

- 🔴 **Error** : Rouge gras
- 🟡 **Warning** : Jaune
- 🔵 **Debug** : Bleu
- ⚪ **Info** : Blanc (normal)

Le niveau est inféré automatiquement depuis le contenu du message (détection de mots-clés comme "error", "warning", "debug").

---

## ⚙️ Configuration

### Système de Configuration v0.1.1

Les deux composants (`mcp-log-agent` et `mcp-logs`) supportent maintenant une configuration complète via fichiers et variables d'environnement.

#### log-agent (CLI Rust)

**Créer un fichier de configuration :**
```bash
# Local (projet actuel)
mcp-log-agent config init --local

# Global (utilisateur)
mcp-log-agent config init --global
```

**Fichier généré** : `.mcp-log-agent.toml` avec commentaires détaillés ligne par ligne

**Exemple de configuration simple :**
```toml
[agent]
default_project = "mon-app"
default_command = ["npm", "start"]  # Lancez avec juste "mcp-log-agent run"
```

**Commandes disponibles :**
```bash
mcp-log-agent config show              # Afficher la config actuelle
mcp-log-agent config get <key>         # Obtenir une valeur spécifique
mcp-log-agent config set <key> <value> # Modifier une valeur
mcp-log-agent config detect            # Détecter les sources de config
mcp-log-agent config list              # Lister toutes les clés disponibles
mcp-log-agent config colors list       # Lister les schémas de couleurs
mcp-log-agent config colors set <nom>  # Appliquer un schéma
```

**Exemples config set :**
```bash
# Modifier des valeurs directement
mcp-log-agent config set agent.verbose true
mcp-log-agent config set agent.connection_timeout 10
mcp-log-agent config set output.format plain
mcp-log-agent config set filters.min_level warn
mcp-log-agent config set agent.default_command '["npm", "run", "dev"]'
```

**Schémas de couleurs prédéfinis :**
- `default` - Couleurs par défaut (rouge/jaune/bleu)
- `solarized-dark` - Thème Solarized Dark
- `high-contrast` - Contraste élevé pour l'accessibilité
- `minimal` - Couleurs minimales
- `monochrome` - Nuances de gris uniquement

**Variables d'environnement :**
```bash
# Agent settings
export MCP_LOG_AGENT_SOCKET_PATH="/custom/path.sock"
export MCP_LOG_AGENT_DEFAULT_PROJECT="my-project"
export MCP_LOG_AGENT_VERBOSE=true
export MCP_LOG_AGENT_CONNECTION_TIMEOUT=10

# Output settings
export MCP_LOG_AGENT_COLORS=false
export MCP_LOG_AGENT_FORMAT=json
export MCP_LOG_AGENT_SHOW_TIMESTAMPS=true

# Color customization
export MCP_LOG_COLOR_ERROR_FG=bright_red
export MCP_LOG_COLOR_WARN_FG=bright_yellow

# Filters
export MCP_LOG_FILTER_MIN_LEVEL=warn

# Performance
export MCP_LOG_AGENT_BUFFER_SIZE=2000
```

#### mcp-logs (Serveur MCP)

**Créer un fichier de configuration :**
```bash
cd mcp-logs

# Local avec commentaires détaillés
bun run index.ts config init

# Global
bun run index.ts config init --global

# Minimal sans commentaires
bun run index.ts config init --minimal
```

**Fichier généré** : `.mcp-logs.json` avec commentaires inline (`_comment` fields)

**Commandes disponibles :**
```bash
bun run index.ts config show                    # Afficher la config actuelle
bun run index.ts config get <key>               # Obtenir une valeur spécifique
bun run index.ts config set <key> <value>       # Modifier une valeur
bun run index.ts config list                    # Lister toutes les clés
bun run index.ts config help                    # Aide
```

**Exemples config set :**
```bash
bun run index.ts config set server.verbose true
bun run index.ts config set storage.max_logs 20000
bun run index.ts config set logging.log_level debug --global
```

**Variables d'environnement :**
```bash
export MCP_LOGS_SOCKET_PATH="/custom/path.sock"
export MCP_LOGS_MAX_LOGS=20000
export MCP_LOGS_VERBOSE=true
export MCP_LOGS_LOG_LEVEL=debug
```

### Hiérarchie de Configuration

**Priorité (du plus haut au plus bas) :**
1. Arguments CLI
2. Variables d'environnement (`MCP_LOG_*` / `MCP_LOGS_*`)
3. Config locale (`.mcp-log-agent.toml` / `.mcp-logs.json`)
4. Config globale (`~/.config/*/config.*`)
5. Valeurs par défaut

### Configuration Rapide

**Exemple : Changer le chemin du socket pour les deux composants**

```bash
# log-agent
echo 'MCP_LOG_AGENT_SOCKET_PATH="/custom/path.sock"' >> ~/.bashrc

# mcp-logs
echo 'MCP_LOGS_SOCKET_PATH="/custom/path.sock"' >> ~/.bashrc

# Ou dans les fichiers de config
mcp-log-agent config init --local
# Modifier: agent.socket_path = "/custom/path.sock"

cd mcp-logs && bun run config.ts init
# Modifier: server.socket_path = "/custom/path.sock"
```

### Mode verbose

Par défaut, le serveur MCP est en mode silencieux. Pour activer les logs détaillés :

Via config :
```bash
# mcp-logs
bun run config.ts init
# Modifier: server.verbose = true
```

Via environnement :
```bash
VERBOSE=true mcp-logs
# ou
MCP_LOGS_VERBOSE=true mcp-logs
```

### Limite de logs en mémoire

Via config (`mcp-logs`):
```bash
bun run config.ts init
# Modifier: storage.max_logs = 20000
```

Via environnement :
```bash
MCP_LOGS_MAX_LOGS=20000 mcp-logs
```

---

## 🧪 Test de connexion

Vérifiez que tout fonctionne :

```bash
mcp-log-agent test --message "Hello from CLI"
```

Sortie attendue :
```
✓ Successfully sent test message to MCP server
```

---

## 🐛 Dépannage

### Le socket n'existe pas

**Cause** : Le serveur MCP n'est pas démarré.

**Solution** :
```bash
mcp-logs-server
# ou
cd mcp-logs && bun run index.ts
```

### Permission denied sur le socket

**Cause** : Problème de permissions utilisateur.

**Solution** : Vérifiez que le CLI et le serveur tournent sous le même utilisateur.

### Logs perdus

**Cause** : Le serveur MCP n'est pas accessible.

**Solution** : Les logs s'affichent quand même dans le terminal du CLI, mais ne sont pas stockés. Démarrez le serveur MCP pour les capturer.

### Trop de logs en mémoire

**Solution** : Utilisez `clear_logs` ou ajustez la limite dans `LogStore`.

---

## 📊 Format des messages

Les logs sont échangés via Unix socket au format JSON :

```json
{
  "version": "1.0",
  "type": "log_entry",
  "data": {
    "timestamp": "2025-12-28T14:30:45.123Z",
    "level": "info",
    "source": "stdout",
    "project": "my-app",
    "message": "Server started on port 3000",
    "pid": 12345
  }
}
```

**Niveaux** : `info`, `warn`, `error`, `debug`  
**Sources** : `stdout`, `stderr`

---

## 🏗️ Structure du projet

```
mcp-log/
├── log-agent/              # CLI Rust
│   ├── src/
│   │   ├── cli/           # Arguments CLI (clap)
│   │   ├── capture/       # Capture stdout/stderr (tokio)
│   │   ├── types/         # Types de données
│   │   ├── socket.rs      # Client Unix socket
│   │   └── main.rs        # Point d'entrée
│   └── Cargo.toml
│
└── mcp-logs/              # Serveur MCP (Bun/TypeScript)
    ├── src/
    │   ├── mcp/
    │   │   ├── handlers.ts  # Handlers des outils MCP
    │   │   └── tools.ts     # Définitions des outils
    │   ├── server/
    │   │   └── index.ts     # Serveur Unix socket
    │   ├── store/
    │   │   └── index.ts     # Store en mémoire
    │   └── types/
    │       └── index.ts     # Types TypeScript
    ├── index.ts             # Point d'entrée MCP
    └── package.json
```

---

## 🚧 Limites actuelles

- ⚠️ Stockage en mémoire uniquement (logs perdus au redémarrage)
- ⚠️ Maximum 10000 logs en mémoire (FIFO)
- ⚠️ Communication locale uniquement (Unix socket)
- ⚠️ Linux/macOS uniquement (pas de support Windows)

---

## 🤝 Contribuer

Améliorations possibles :
- [ ] Persistence sur disque (SQLite, PostgreSQL)
- [ ] Support Windows (Named Pipes)
- [ ] Interface web de visualisation
- [ ] Métriques et alertes
- [ ] Filtrage en temps réel côté serveur
- [ ] Export des logs (JSON, CSV)

---

## 📄 Licence

MIT © 2025 Yacine Yaici
