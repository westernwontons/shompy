set positional-arguments
set shell := ["zsh", "-uc"]

alias r := run

default: run

@run:
  cargo run --bin ui

@prisma-generate:
  cargo prisma generate --schema=./prisma/schema.prisma