FROM rust:latest
ENV CARGO_TERM_COLOR always
# Update User's system 
RUN apt-get update && apt-get-install -y libpq-dev cmake 
# Create an empty project for caching dependencies
RUN USER=root cargo new --bin /reviews/build
# Set the Working directory to build
WORKDIR /reviews/build 
# Copy Cargo.lock to build file
COPY /Cargo.lock ./
# Copy Cargo toml to the build file 
COPY ./reviews/Cargo.toml ./ 
# Cache Dependencies 
COPY ./reviews/ ./
RUN cargo install --path . --locked


FROM debian:bookworm-slim
RUN apt-get update && apt-get-install -y libpq-dev cmake

COPY --from=0 \ 
    /usr/local/cargo/bin/reviews \ 
    /usr/local/bin/reviews

CMD [ "reviews" ]
EXPOSE 4003