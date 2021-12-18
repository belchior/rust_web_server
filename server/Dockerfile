FROM base_image as builder
WORKDIR /server
COPY . .
RUN ./scripts/build.sh

FROM alpine
WORKDIR /server
COPY --from=builder /server/release/rust_web_server ./release/rust_web_server
CMD ["./release/rust_web_server"]

# Build
# docker build -t rust_web_server .
