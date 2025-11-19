# AGENTS.md

This file provides guidance to agents when working with code in this repository.

## Build Commands

- **Workspace build**: `cargo build` (builds all crates)
- **Single crate build**: `cargo build -p <crate-name>` (e.g., `cargo build -p blog`)
- **Run blog app**: `cargo run -p blog -- run config.yaml`
- **Test single crate**: `cargo test -p <crate-name>`

## Project Structure

**Workspace contains multiple independent crates**:

- `algorithm/` - Algorithm library with divide & conquer, search, sort modules
- `game/chinese-chess/` - Desktop GUI game using eframe/egui
- `web/blog/` - Actix-web blog with SQLite database (main web app)
- `web/blog-spring/` - Spring-based blog implementation
- `web/w-*` - Web utilities: w-actix (Actix examples), w-ddd (DDD entities), w-macro (procedural macros)

## Critical Architecture Patterns

**Custom Procedural Macro System**:

- `w-macro` crate contains entity generation macros
- `#[entity_fields]` adds standard audit fields (id, created_by, updated_by, timestamps, version, deleted)
- `#[entity_field(name, ty)]` adds individual fields to structs
- `#[derive(Entity)]` implements Entity trait with getter/setter methods
- `repository!` macro generates full repository implementation with RBatis

**Non-Standard Async Handling**:

- Repository implementations use **manual future polling** instead of `.await`
- Pattern: `std::pin::pin!(future).poll(&mut Context)` (see w-macro lines 209-224)
- Required for RBatis async operations wrapped in sync interface

**Global Repository Pattern**:

- Repository instances stored as global statics using `LazyLock`
- Initialized via `#[ctor::ctor]` functions
- Access via `*_repository()` functions that return `&'static dyn Repository`

## Database Configuration

**SQLite Database Setup**:

- Database path configured in `web/blog/config.yaml` (default: `./deploy/blog.db`)
- Migration scripts in `web/blog/deploy/article.sql`
- RBatis ORM with custom repository layer

## Entry Points

- **Blog web app**: `web/blog/src/main.rs` - CLI with subcommands: `run`, `hello`, `hello2`
- **Chinese chess game**: `game/chinese-chess/src/main.rs` - Desktop GUI application  
- **Simple web examples**: `web/w-actix/src/main.rs`

## Dependencies

- **Web frameworks**: actix-web (primary), spring-web (secondary), axum (minimal)
- **Database**: rbatis + rbdc-sqlite for async DB operations
- **GUI**: eframe + egui for desktop game
- **Async runtime**: tokio across all async code
- **Configuration**: config crate with YAML support

## Common Patterns

- **Error handling**: `anyhow::Result<()>` with contextual error messages
- **Logging**: `simple_logger` with `log` crate
- **Configuration**: Global static config with atomic initialization guards
- **Repository CRUD**: Auto-generated via macros with manual async polling
