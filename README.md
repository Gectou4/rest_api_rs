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

## Docker

### Lancer avec Docker Compose

```bash
docker compose up --build
```

L'API est accessible sur `http://localhost:3000`.

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
