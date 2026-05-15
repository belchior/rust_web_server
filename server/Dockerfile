FROM server_base_image_postgres AS builder
WORKDIR /server
COPY . .
RUN ./scripts/build.sh

FROM alpine
WORKDIR /server
COPY --from=builder /server/release/rust_web_server ./release/rust_web_server

EXPOSE 4000
CMD ["./release/rust_web_server"]

# Build
# docker build -t server .

# Run
# docker run -it --rm \
#   --name server \
#   --publish=4000:80 \
#   --network=rust_web_server_mynetwork \
#   --env=CORS_ALLOWED_ORIGINS=http://localhost:5173 \
#   --env=DATABASE_CONNECTIONS_NUMBER=4 \
#   --env=DATABASE_DB=database \
#   --env=DATABASE_HOST=database \
#   --env=DATABASE_PASSWORD=postgres \
#   --env=DATABASE_PORT=5432 \
#   --env=DATABASE_TIMEOUT=300000 \
#   --env=DATABASE_USER=postgres \
#   --env=RUST_ENV=production \
#   --env=RUST_LOG=info \
#   --env=SERVER_HOST=0.0.0.0 \
#   --env=SERVER_NAME=server \
#   --env=SERVER_PORT=80 \
#   server
