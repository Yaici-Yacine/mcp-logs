# MCP Logs - Système de capture de logs en temps réel

Système complet de capture et analyse de logs en temps réel pour projets de développement, avec communication via Unix socket entre un CLI Rust et un serveur MCP Bun.
`

## Installation

### 1. Compiler le CLI Rust

```bash
cd log-agent
cargo build --release
```

Le binaire sera dans `target/release/log-agent`

### 2. Installer les dépendances MCP

```bash
cd mcp-logs
bun install
```

## Utilisation

### Étape 1 : Démarrer le serveur MCP

Dans un terminal, lancez le serveur MCP :

```bash
cd mcp-logs
bun run index.ts
```

Vous devriez voir :
```
🚀 MCP Logs Server starting...
✓ Socket server listening on /tmp/log-agent.sock
✓ MCP server ready
ℹ Waiting for logs from log-agent CLI...
```

### Étape 2 : Lancer votre projet avec log-agent

Dans un autre terminal, utilisez le CLI pour capturer les logs :

```bash
# Exemple avec Bun
./log-agent/target/release/log-agent run --project my-app bun dev

# Exemple avec Node
./log-agent/target/release/log-agent run --project api-server npm start

# Exemple avec Rust
./log-agent/target/release/log-agent run --project rust-app cargo run

# Exemple avec Python
./log-agent/target/release/log-agent run --project python-app python main.py
```

### Étape 3 : Analyser les logs via OpenCode

Dans OpenCode/Claude, utilisez les outils MCP pour analyser les logs :

```
Montre-moi les 50 derniers logs
Recherche "error" dans les logs
Quels sont les logs du projet "my-app" ?
Montre-moi les statistiques des logs
```

## Outils MCP disponibles

### 1. `get_recent_logs`
Récupère les derniers logs (par défaut 50, max 500).

**Paramètres :**
- `count` (optionnel) : nombre de logs à récupérer

**Exemple :**
```json
{
  "count": 100
}
```

### 2. `get_logs`
Récupère les logs avec filtrage avancé.

**Paramètres :**
- `project` (optionnel) : nom du projet
- `level` (optionnel) : `info`, `warn`, `error`, `debug`
- `source` (optionnel) : `stdout`, `stderr`
- `search` (optionnel) : recherche textuelle
- `limit` (optionnel) : nombre max de résultats (défaut 100)

**Exemple :**
```json
{
  "project": "my-app",
  "level": "error",
  "limit": 50
}
```

### 3. `search_logs`
Recherche dans les logs par contenu textuel.

**Paramètres :**
- `query` (requis) : texte à rechercher
- `project` (optionnel) : filtrer par projet
- `limit` (optionnel) : nombre max de résultats (défaut 50)

**Exemple :**
```json
{
  "query": "database connection",
  "limit": 20
}
```

### 4. `get_errors`
Récupère uniquement les logs de niveau erreur.

**Paramètres :**
- `project` (optionnel) : filtrer par projet
- `limit` (optionnel) : nombre max d'erreurs (défaut 50)

### 5. `get_stats`
Statistiques globales : nombre total de logs, projets actifs, distribution par niveau.

### 6. `clear_logs`
Vide tous les logs en mémoire.

## Protocole JSON

Format des messages échangés via le socket :

```json
{
  "version": "1.0",
  "type": "log_entry",
  "data": {
    "timestamp": "2025-12-23T10:30:45.123Z",
    "level": "info",
    "source": "stdout",
    "project": "my-app",
    "message": "Server started on port 3000",
    "pid": 12345
  }
}
```

**Niveaux de log :**
- `info` : logs informatifs
- `warn` : avertissements
- `error` : erreurs
- `debug` : logs de debug

**Sources :**
- `stdout` : sortie standard
- `stderr` : sortie d'erreur

## Configuration

### Changer le chemin du socket

**Dans le CLI Rust** (`log-agent/src/socket.rs`) :
```rust
pub const SOCKET_PATH: &str = "/tmp/log-agent.sock";
```

**Dans le serveur MCP** (`mcp-logs/src/server/index.ts`) :
```typescript
export const SOCKET_PATH = "/tmp/log-agent.sock";
```

### Ajuster la limite de logs en mémoire

Dans `mcp-logs/index.ts` :
```typescript
const logStore = new LogStore(10000); // 10000 logs max
```

## Tester la connexion

Testez que le socket fonctionne :

```bash
./log-agent/target/release/log-agent test --message "Hello from CLI"
```

## Exemples pratiques

### Surveiller une application Next.js

```bash
log-agent run --project nextjs-app bun dev
```

### Capturer les tests

```bash
log-agent run --project tests npm test
```

### Plusieurs projets en parallèle

Terminal 1 :
```bash
log-agent run --project frontend bun dev
```

Terminal 2 :
```bash
log-agent run --project backend cargo run
```

Les logs des deux projets seront visibles dans le serveur MCP et différenciables par leur nom.

## Dépannage

### Le socket n'existe pas

Vérifiez que le serveur MCP est démarré en premier :
```bash
cd mcp-logs && bun run index.ts
```

### Permission denied sur le socket

Le socket est créé avec les permissions de l'utilisateur. Assurez-vous que les deux processus tournent sous le même utilisateur.

### Logs perdus

Le CLI continue de fonctionner même si le serveur MCP n'est pas disponible. Les logs sont affichés dans le terminal mais ne sont pas stockés. Démarrez le serveur MCP pour les capturer.

### Trop de logs en mémoire

Ajustez la limite dans `LogStore` ou utilisez `clear_logs` régulièrement.

## Limites actuelles

- Stockage en mémoire uniquement (les logs sont perdus au redémarrage du serveur MCP)
- Maximum 10000 logs en mémoire par défaut (FIFO : les plus anciens sont supprimés)
- Communication locale uniquement (Unix socket)
- Un seul serveur MCP à la fois sur un socket donné

## Structure du projet

```
mcp-log/
├── log-agent/              # CLI Rust
│   ├── src/
│   │   ├── cli/           # Arguments CLI
│   │   ├── capture/       # Capture stdout/stderr
│   │   ├── types/         # Types de données
│   │   ├── socket.rs      # Client Unix socket
│   │   └── main.rs        # Point d'entrée
│   └── Cargo.toml
│
└── mcp-logs/              # Serveur MCP Bun
    ├── src/
    │   ├── mcp/
    │   │   ├── handlers.ts  # Handlers des outils
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

## Contribuer

Ce projet est un POC. Améliorations possibles :
- Persistence sur disque (base de données)
- Support de multiples sockets
- Filtrage en temps réel
- Interface web
- Métriques et alertes
