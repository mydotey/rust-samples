# Debug Mode Rules (Non-Obvious Only)

- Blog web server logs to console via `simple_logger`, level controlled by environment
- Database file location: `web/blog/deploy/blog.db` (SQLite)
- Run blog server: `cargo run -p blog -- run config.yaml` then access <http://localhost:8080>
- Use `web/blog/test.http` for HTTP request testing with compatible tools
- Chinese chess game: `cargo run -p chinese-chess` launches desktop GUI application
- RBatis async operations use manual polling - breakpoints may not work as expected in repository methods
- Global configuration uses atomic initialization - debug config loading issues in web/blog/src/conf.rs:11-15
- SQLite database must exist before running - check `web/blog/deploy/` directory
