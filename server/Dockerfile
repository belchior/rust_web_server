FROM server_base_image_postgres as builder
WORKDIR /server
COPY . .
RUN ./scripts/build.sh

FROM alpine
WORKDIR /server
COPY --from=builder /server/release/rust_web_server ./release/rust_web_server

EXPOSE 8080
CMD ["./release/rust_web_server"]

# Build
# docker build -t server .

# Run
# docker run -it --rm \
#   --name server \
#   --publish=8080:8080 \
#   --env=SERVER_PORT=8080 \
#   --env=SERVER_HOST=0.0.0.0 \
#   --env=POSTGRES_DB=database \
#   --env=POSTGRES_HOST=172.17.0.2 \
#   --env=POSTGRES_PASSWORD=postgres \
#   --env=POSTGRES_PORT=5432 \
#   --env=POSTGRES_USER=postgres \
#   --env=CORS_ALLOWED_ORIGINS=http://localhost:3000 \
#   server
