# Made with Rust! (Tera + Pulldown-cmark + Axum + Argon2 + Ammonia + jsonwebtoken + tokio)
That is a simple stack that I assembled, while thinking about making something big and production grade
## What is already here
- All main routes (index, post creation, blog view, login and register)
- Cool styles (thanks to 98.css styles)
- Basic authentication (middleware protection, XSS protection, JWT, password hashing with Argon2)
- SQLite integration (plan to migrate on MySQL/Postgres for better security)
- XSS security (using Ammonia sanitizing and disabling raw HTML in pulldown-cmark parser)
## What I want to do next
- Search posts by slugs
- More advanced schema (support for tags, categories etc.)
- Email confirmation
- Support for TOTP authentication and 2FA
- CSP blocking of all JavaScript with minor exceptions
- Dockerize the service (for more advanced infrastructure)
- Optional K8s and more robust infrastructure
## Have fun, people! This repo is always open for expanding
