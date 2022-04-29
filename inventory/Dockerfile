FROM rust:1.59
ENV CARGO_TERM_COLOR always
# Update User's system 
RUN apt-get update && apt-get install -y libpq-dev clang
# Create an empty project for caching dependencies
RUN USER=root cargo new --bin /inventory/docker-build
# Set the Working directory to build
WORKDIR /inventory/docker-build 
# Copy Cargo.lock to build file
COPY /Cargo.lock ./
# Copy Cargo toml to the build file 
COPY ./inventory/Cargo.toml ./ 
# Cache Dependencies 
COPY ./inventory/ ./
RUN cargo install --path . --locked


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=0 \ 
    /usr/local/cargo/bin/inventory \ 
    /usr/local/bin/inventory

CMD [ "inventory" ]
EXPOSE 4004