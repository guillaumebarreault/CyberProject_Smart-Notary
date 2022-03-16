FROM rust:1.59.0 as builder

RUN mkdir /usr/src/rustwebservice
WORKDIR /usr/src/rustwebservice
COPY . .

RUN rustup default nightly
RUN cargo build --release

EXPOSE 8000

FROM gcr.io/distroless/cc-debian11


COPY --from=builder /usr/src/rustwebservice/target/release/rustwebservice /usr/src/rustwebservice/
COPY --from=builder /usr/src/rustwebservice/templates /usr/src/rustwebservice/templates/
COPY --from=builder /usr/src/rustwebservice/static /usr/src/rustwebservice/static/

WORKDIR /usr/src/rustwebservice

EXPOSE 8000

CMD ["./rustwebservice"]