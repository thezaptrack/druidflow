FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

# refresh
RUN apt update -y && apt upgrade -y

# refresh rust toolchain
RUN rustup self update

WORKDIR /workspace
COPY .. .

USER root