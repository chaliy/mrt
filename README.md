# MRT - MonoRepo Tool

cli to serve polyglot monorepo

## Features

- [x] List package in the monorepo - `mrt list`
- [x] Ability to execute "scripts" across monorepo: start, build, test, fmt, etc - `mrt run [script]`
- [x] Support for nodejs/npm packages and apps
- [ ] Support for python/poetry packages and apps
- [ ] Support for rust/cargo packages and apps
- [ ] Support for custom packages and apps
- ...

## How to install

If you have rust/cargo installed, you can install `mrt` with:

```
cargo install --git https://github.com/chaliy/mrt.git
mrt --help
```

### ZSH

```
mrt completion zsh > $ZSH/cache/completions/_mrt
compinit
mrt <TAB>
```

## Idea

Build tool that will help with common operations on polyglot monorepo. Run "scripts" across all packages, do release management, etc. 

- Run "scripts" across monorepo like build, test, fmt, etc - `mrt run [script]`
- Polyglot packages, mrt should run any. Initially nodejs/npm, python/poetry, Makefile and custom
- Dependency graph, if packages depends on each other, mrt should know
- Be aware of changes, detect changes and run scripts against changes
- Allow filter / group / list packages. So for example need to find a way to run "test" script only on "ui" packages

### More?

- Help with CI, for example command to proxy package command only when something has been changed
- Handle initialization of dev environment
- Handle running monorepo (e.g. microservices docker-compose)
- Version / release management. Single repo version, or per package version

## Design

### CLI

`mrt list` - List all monorepo packages
`mrt list [packages_spec]` - List specified packages

`mrt run [packages_spec] [script]` - Run script for specified packages
`mrt run [script]` - Run script for all specified packages


`[packages_spec]` - package | package1,package2 | pa* | packages/pack* 

## Inspiration

- Lerna / Nx - NodeJS monorepo tools
- Bazel - monorepo build tool