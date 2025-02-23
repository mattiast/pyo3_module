# First build Rust code, create a Python wheel
FROM rust:latest AS builder

RUN rustup component add rustfmt clippy

# Create a non-root user
RUN useradd -m dev
USER dev
WORKDIR /home/dev

COPY --from=ghcr.io/astral-sh/uv:latest /uv /uvx /bin/
RUN uv python install cpython-3.12.9-linux-x86_64-gnu
RUN uv venv
ENV PATH="/home/dev/.venv/bin:$PATH"
ENV VIRTUAL_ENV="/home/dev/.venv"
ENV LC_ALL=C.UTF-8
RUN uv pip install maturin pytest numpy hypothesis sphinx sphinx-rtd-theme ruff

