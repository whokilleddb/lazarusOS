FROM rustlang/rust:nightly-buster as base
WORKDIR /minicupcake

# Extra Metadata
LABEL version = "0.1.0"
LABEL description = "minicupcake"
LABEL author = "whokilleddb"

FROM base as setup
COPY Cargo.toml /minicupcake
COPY Cargo.lock /minicupcake
COPY src /minicupcake/src

FROM setup