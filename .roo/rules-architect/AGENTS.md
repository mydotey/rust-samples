# Architect Mode Rules (Non-Obvious Only)

- Repository pattern implemented via procedural macros with global static access - not dependency injection
- Manual future polling required for RBatis async operations wrapped in sync interface (non-standard Rust async pattern)
- Multi-framework web architecture: Actix-web (main), Spring-web (alternative), Axum (examples)
- DDD entities automatically generate audit fields via `#[entity_fields]` macro (id, created_by, updated_by, timestamps, version, deleted)
- SQLite database with custom RBatis layer - migrations in web/blog/deploy/article.sql
- Cross-crate dependencies: blog → blog-client, blog → w-ddd, blog → w-macro, algorithm → lang-extension
- Desktop GUI game separated from web stack using eframe/egui
- Configuration system uses global statics with atomic guards instead of dependency injection
- Repository instances initialized via `#[ctor::ctor]` functions at program startup
- Async operations use manual polling pattern instead of standard `.await` syntax in generated repository code
