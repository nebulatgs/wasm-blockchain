FROM rustlang/rust:nightly as wasm-builder
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
WORKDIR /app
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