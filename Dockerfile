# Stage 1
FROM rust as build-step-rust
RUN mkdir -p /app
WORKDIR /app
COPY backend /app
RUN cargo build --release
RUN ls -la /app/target/release


# Stage 1
FROM node:22-slim as build-step-angular
RUN mkdir -p /app
WORKDIR /app
COPY frontend/looksyk/package.json /app
RUN npm install
COPY frontend/looksyk/ /app/
RUN npm run build -- --configuration=production
RUN ls -la /app/dist


# Assembly
FROM debian:stable-slim
RUN apt-get update && apt-get install openssl ca-certificates curl --yes && apt-get clean
RUN mkdir /graph

COPY --from=build-step-rust /app/target/release /app
COPY --from=build-step-angular /app/dist/looksyk/browser /app/static
WORKDIR /app
EXPOSE 11000

ENTRYPOINT ["./looksyk", "--graph-location=/graph", "--external-app=true", "--static-path=/app/static"]
