FROM rust:latest
ENV CARGO_TERM_COLOR always
# Update User's system 
RUN apt-get update && apt-get-install -y libpq-dev cmake 
# Create an empty project for caching dependencies
RUN USER=root cargo new --bin /products/build
# Set the Working directory to build
WORKDIR /products/build 
# Copy Cargo.lock to build file
COPY /Cargo.lock ./
# Copy Cargo toml to the build file 
COPY /products/Cargo.toml ./ 
# Cache Dependencies 
COPY /products/ ./
RUN cargo install --path . --locked
RUN cargo build --release 

# FROM debian:bookworm-slim
RUN apt-get update && apt-get-install -y libpq-dev cmake

# COPY --from=0 \ 
#     /usr/local/cargo/bin/products \ 
#     /usr/local/bin/products

CMD [ "products" ]
EXPOSE 4002