FROM server_base_image_postgres AS builder
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
#   --env=DATABASE_DB=database \
#   --env=DATABASE_HOST=172.17.0.2 \
#   --env=DATABASE_PASSWORD=postgres \
#   --env=DATABASE_PORT=5432 \
#   --env=DATABASE_USER=postgres \
#   --env=DATABASE_CONNECTIONS_NUMBER=4 \
#   --env=CORS_ALLOWED_ORIGINS=http://localhost:3000 \
#   server
