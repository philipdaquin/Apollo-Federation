FROM rust:1.59
ENV CARGO_TERM_COLOR always
# Update User's system 
RUN apt-get update && apt-get install -y libpq-dev clang
# Create an empty project for caching dependencies
RUN USER=root cargo new --bin /reviews/docker-build
# Set the Working directory to build
WORKDIR /reviews/docker-build 
# Copy Cargo.lock to build file
COPY /Cargo.lock ./
# Copy Cargo toml to the build file 
COPY ./reviews/Cargo.toml ./ 
# Cache Dependencies 
COPY ./reviews/ ./
RUN cargo install --path . --locked


FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=0 \ 
    /usr/local/cargo/bin/reviews \ 
    /usr/local/bin/reviews

CMD [ "reviews" ]
EXPOSE 4003