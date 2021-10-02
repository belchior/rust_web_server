FROM rust:1.54 as builder
WORKDIR /builder
COPY . .
RUN cargo build --release
CMD ["./target/release/rust_web_server"]

# Build
# docker build -t rust_web_server .

# Run
# docker run -it --rm -p 8080:8080 -p 27017:27017 --name rust_web_server rust_web_server
