FROM rust:slim as build
WORKDIR /build
COPY . .
RUN cargo build --release

FROM rust:slim
WORKDIR /opt/resource
COPY --from=build /build/target/PLACEHOLDER/release/concourse-github-issue main
RUN ln -s main check && ln -s main in && ln -s main out
