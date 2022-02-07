FROM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /sushibot/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY . .

WORKDIR /app/wasm
ENV WGPU_BACKEND=web
ENV RUSTFLAGS=--cfg=web_sys_unstable_apis
RUN wasm-pack build --target web --out-name wasm --out-dir static/wasm

FROM node:lts as node-builder
WORKDIR /app
COPY --from=wasm-builder /app .
RUN yarn install --frozen-lockfile
RUN yarn build

FROM node:lts as runner
WORKDIR /app
COPY --from=wasm-builder /app .
RUN yarn install --frozen-lockfile

CMD ["yarn", "start"]