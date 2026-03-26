# CI/CD & Release

## Included Workflows

| Workflow | File | Trigger | Purpose |
|---|---|---|---|
| CI | `ci.yml` | PRs | cargo check, test, clippy |
| Lint | `lint.yml` | PRs | cargo fmt check |
| Build | `build-binaries.yml` | Release | Cross-platform binary builds via cargo-dist |
| npm Publish | `publish-npm.yml` | Release | Publish platform packages to npm |
| Pages | `pages.yml` | Push to main | Deploy `web/` to GitHub Pages |

## Conventional Commits

All commits should follow the format:

```
type(scope): description (#N)
```

Accepted types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `ci`, `perf`.

Examples:

```
feat(download): add retry logic (#42)
fix(config): handle missing file gracefully (#51)
chore(deps): bump serde to 1.0.200
```

## Release Flow

1. Merge your PR into `main`.
2. **release-plz** automatically opens a release PR that bumps the version and updates the changelog.
3. Merge the release PR.
4. **cargo-dist** builds binaries for all platforms and creates a GitHub release.
5. **publish-npm.yml** publishes the npm wrapper packages.

No manual version bumping or tagging required.

## justfile

Run the full pre-commit suite locally:

```bash
just pre-commit
```

This runs, in order: `fmt --check`, `clippy`, `doc`, and `test`.
