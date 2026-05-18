# G4Api - Mini API REST (Rust)

API REST légère en Rust, avec [axum](https://github.com/tokio-rs/axum), répondant en JSON.

Elle gère deux types d'objets et leurs relations :

| Objet  | Champs |
|--------|--------|
| `User` | `user_id`, `name`, `email` |
| `Task` | `task_id`, `title`, `description`, `creation_date`, `status` |

Les statuts de tâche (`status`) sont des entiers : `1` Backlog · `2` Todo · `3` In Progress · `4` Done · `5` Closed.

### Endpoints

| Méthode | URI | Description |
|---------|-----|-------------|
| `GET` | `/user/{id}` | Données d'un utilisateur |
| `GET` | `/user/{id}/task` | Liste des tâches d'un utilisateur |
| `POST` | `/task` | Créer une nouvelle tâche |
| `POST` / `PUT` | `/task/{id}` | Modifier une tâche existante |
| `DELETE` | `/task/{id}` | Supprimer une tâche |
| `POST` / `PUT` | `/user/{id}/task/{taskId}` | Associer une tâche à un utilisateur |
| `DELETE` | `/user/{id}/task/{taskId}` | Retirer l'association tâche ↔ utilisateur |

---

## Configuration

### Base de données

Créer une base MySQL et importer le schéma disponible dans `share/sql/init.sql`.

Configurer la connexion via la variable d'environnement `DATABASE_URL` :

```
DATABASE_URL=mysql://root:root@localhost:3306/rest_api
```

### Routes

Les routes sont déclarées dans `src/routes/mod.rs` avec le router axum.

---

## Développement

### Prérequis

- Rust 1.75+
- MySQL 8.0+

### Installation

```bash
cargo build
```

### Lancer le serveur

```bash
cargo run
```

Le serveur écoute sur `http://0.0.0.0:3000`.

### Tests

Les tests nécessitent une connexion base de données valide.

```bash
cargo test
```

### Linter

```bash
cargo clippy -- -D warnings
```

### Formatter

```bash
cargo fmt
```

---

## Tests

### Tests unitaires (Rust)

Nécessitent une base MySQL locale.

```bash
cargo test
```

### Tests d'intégration (Docker)

Lance l'API + MySQL + script de test curl dans des conteneurs isolés :

```bash
docker compose run --rm test
```

Le script teste **10 scénarios** :

| # | Test | Vérification |
|---|------|-------------|
| 1 | `GET /user/1` | 200 + JSON avec `user_id`, `name`, `email` |
| 2 | `GET /user/999` | 404 (not found) |
| 3 | `GET /user/1/task` | 200 + JSON avec `user_id`, `tasks` |
| 4 | `POST /task` | 201 + tâche créée avec `task_id` |
| 5 | `POST /task/{id}` | 200 (mise à jour) |
| 6 | `POST /user/1/task/{id}` | 200 (association) |
| 7 | `GET /user/1/task` | tâche associée présente |
| 8 | `DELETE /user/1/task/{id}` | 200 (désassociation) |
| 9 | `DELETE /task/{id}` | 200 (suppression) |
| 10 | `GET /task/{id}` | 404/405 (vérification suppression) |

### Tests manuels (PowerShell)

Si l'API tourne déjà en local :

```powershell
.\scripts\test.ps1
```

Ou avec une URL custom :

```powershell
.\scripts\test.ps1 -ApiUrl http://localhost:3000
```

### Tests manuels (bash)

```bash
API_URL=http://localhost:3000 bash scripts/test.sh
```

---

## Docker

### Lancer avec Docker Compose

```bash
docker compose up --build
```

L'API est accessible sur `http://localhost:3000`.

### Lancer les tests d'intégration

```bash
docker compose run --rm test
```

### Variables d'environnement

| Variable | Description | Par défaut |
|----------|-------------|------------|
| `DATABASE_URL` | URL de connexion MySQL | `mysql://root:root@localhost:3306/rest_api` |

---

## Stack technique

- **HTTP** : [axum](https://github.com/tokio-rs/axum)
- **Base de données** : [sqlx](https://github.com/launchbadge/sqlx) (MySQL)
- **Sérialisation** : [serde](https://serde.rs/) + [serde_json](https://github.com/serde-rs/json)
- **Runtime** : [tokio](https://tokio.rs/)
- **Linter** : clippy (built-in)
- **Formatter** : rustfmt (built-in)
