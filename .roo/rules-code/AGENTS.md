# Code Mode Rules (Non-Obvious Only)

- Always use the `repository!` macro in w-ddd instead of manual repository implementations (web/w-macro/src/lib.rs:156-316)
- Implement entities using `#[derive(Entity)]` and `#[entity_fields]` together for automatic audit field generation
- Use manual future polling pattern for RBatis operations: `std::pin::pin!(future).poll(&mut Context)` (web/w-macro/src/lib.rs:209-224)
- Database queries must use RBatis CRUD macros, not raw SQL (see generated code in repository! macro)
- Access repositories via global static functions: `*_repository()` returning `&'static dyn Repository`
- Configuration uses global static with atomic guards (web/blog/src/conf.rs:8-15)
- Blog CLI supports subcommands: `run config.yaml`, `hello <name>`, `hello2 <name>` (web/blog/src/main.rs:27-31)
