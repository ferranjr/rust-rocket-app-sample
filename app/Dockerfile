FROM rust:1.73.0-buster as builder
WORKDIR /app
COPY .. .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/rocket-mongo-api /usr/local/bin/rocket-mongo-api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["rocket-mongo-api"]