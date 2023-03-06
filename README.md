# kiwi

Small project to create an app like Twitter with Axum, SQLite (PostgreSQL in the future) and Svelte.

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Maintainers](#maintainers)
- [License](#license)

## Background

I'm trying to take a simple approach when developping the project with an easy to read structure to help better understand the different layers created for most web apps:

```bash
.
├── migrations          # Contains SQL files to generate the database.
└── src
    ├── config.rs       # Retrieves CLI parameters / Global variables.
    ├── error.rs        # Create custom errors.
    ├── lib.rs          # Presents modules.
    ├── main.rs
    ├── router
    │   ├── mod.rs      
    │   └── server.rs   # Contains Router struct and its layer(s).
    ├── module
    │   ├── logical.rs  # Contains all the logic used for the module.
    │   ├── mod.rs
    │   └── routes.rs   # Contains the routes.
    └── ...
        ├── logical.rs
        ├── mod.rs
        └── routes.rs
```

## Install

This project is only available through *git clone* for the moment.

## Maintainers

[@antoinemarneur](https://github.com/antoinemarneur).

## License

[MIT](LICENSE) © Antoine Marneur