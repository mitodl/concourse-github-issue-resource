FROM rust:slim-bookworm@sha256:cf9dd0ec73e75f827fe59123fff9dc65af1a1c8363c3c31ee8d7f8ad0b6a5fb2 as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim@sha256:4724b8cc51e33e398f0e2e15e18d5ec2851ff0c2280647e1310bc1642182655d
WORKDIR /opt/resource
COPY --from=build /build/target/release/concourse-github-issue main
RUN ln -s main check && ln -s main in && ln -s main out
