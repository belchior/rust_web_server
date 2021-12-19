FROM server_base_image as builder
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
#   --env=DATABASE_URI=host.docker.internal \
#   --env=SERVER_HOST=0.0.0.0 \
#   --env=SERVER_PORT=8080 \
#   --env=DATABASE_URI=mongodb://host.docker.internal:27017 \
#   --env=DATABASE_NAME=learning \
#   --env=CORS_ALLOWED_ORIGINS=http://localhost:3000 \
#   --publish=8080:8080 \
#   server
