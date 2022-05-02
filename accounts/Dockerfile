FROM rust:1.59
ENV CARGO_TERM_COLOR always
# Update User's system 
RUN apt-get update && apt-get install -y libpq-dev clang
# Create an empty project for caching dependencies
RUN USER=root cargo new --bin /accounts/docker-build
COPY common_utils ./
# Set the Working directory to build
WORKDIR /accounts/docker-build 
# Copy Cargo.lock to build file
COPY /.env ./
COPY /Cargo.lock ./
# Copy Cargo toml to the build file 
COPY ./accounts/Cargo.toml ./ 
# Cache Dependencies 
COPY ./accounts/ .
RUN cargo install --path . --locked


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=0 \ 
    /usr/local/cargo/bin/accounts \ 
    /usr/local/bin/accounts

CMD ["accounts"]
EXPOSE 4001