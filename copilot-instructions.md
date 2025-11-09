# Copilot Instructions

Keep these guidelines minimal and practical. Follow existing project patterns where present.

## General

- Make small, reviewable commits with clear commit messages.
- Run project tests and linters locally where available before opening a PR.
- If debugging an issue is taking a long time, write small unit tests to confirm your assumptions.
- Write short, general, independent, reusable helper functions when you can.
- Place helper functions at the bottom of the file or in separate modules.
- Follow the style in surrounding files.
- Avoid making changes that were not requested.

## Rust

- Use `cargo fmt` / `rustfmt` for formatting; prefer idiomatic Rust and clear ownership.
- Add small focused changes; avoid large refactors in the same PR.
- Include or update unit tests when behavior changes; run `cargo test` before submitting.
- Prefer descriptive error types (use `thiserror` or `anyhow` only where consistent with the repo).
- Don’t change public APIs unless necessary; bump semver-aware changes with a note.

## TypeScript (Deno)

- Use `deno` tooling for formatting and linting.
- Do not use `node`, `npm`, `npx`, or `yarn`.
- Prefer explicit return types for exported functions and keep helper functions small.
- Put helper functions at the top-level if they do not have dependencies on outer function scope.
- Use async/await, handle errors explicitly, and avoid swallowing exceptions.
- Keep changes focused and avoid reformatting unrelated files.

## Bash

- Add a shebang and keep permissions executable when needed (`chmod +x`).
- Keep scripts idempotent and document required environment variables.

If you need more detailed rules, ask for an expanded style guide.
