FROM rust:1.75-bullseye as builder
WORKDIR /usr/src/rustservice
COPY . .
RUN apt-get install -y curl
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs \
    build-essential && \
    node --version && \
    npm --version \
RUN npm i -g quasar
RUN npm i -g @quasar/cli
RUN cd /usr/src/rustservice/web && npm install && quasar build
RUN cd /usr/src/rustservice
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y curl
RUN apt-get install -y extra-runtime-dependencies pkg-config & apt-get clean & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/universal-bottle-tracker /usr/local/bin/universal-bottle-tracker
COPY --from=builder /usr/src/rustservice/web/dist/spa /static
EXPOSE 3000
HEALTHCHECK --start-period=60s  CMD curl --fail http://localhost:3000/health || exit 1
CMD ["universal-bottle-tracker"]