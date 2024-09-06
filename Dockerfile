FROM rust:latest

# refresh
RUN apt update -y && apt upgrade -y

# install system utilities
RUN apt install -y git curl wget build-essential

# install system dependencies
RUN apt install libopencv-dev clang libclang-dev

WORKDIR /workspace

COPY . .