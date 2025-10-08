# Contributing

## Version validation: GitHub Actions

GitHub repository [peter-lyons-kehl/ndd/](https://github.com/peter-lyons-kehl/ndd/) validates the
crate's version with a [GitHub action](.github/workflows/main.yml). See [README.md](README.md) about
acceptable `stable` and `nightly` versions.

## Version validation: Local GIT pre-commit hook

Set a local GIT pre-commit hook:
```bash
cd .git/hooks
ln -s ../../pre-commit
```

## GIT branches

`nightly` functionality is on [`nightly`
branch](https://github.com/peter-lyons-kehl/ndd/tree/nightly).

### Warning about nightly

`nightly` branch is subject to continuous rebase (and forced push). If you need to extend/modify
`nightly`-specific functionality, communicate first.

## File formatting

- Use `cargo fmt` for Rust source.
- Leave one empty line at the end of Rust, Markdown and any other source files.
