
FROM client_base_image as builder
WORKDIR /client
COPY . .
ARG SERVER_URL
ENV REACT_APP_SERVER_URL=${SERVER_URL}
ENV CI=true
RUN npm run build

FROM nginx:alpine
WORKDIR /usr/share/nginx/html
COPY --from=builder /client/build/ .
COPY --from=builder /client/nginx.conf /etc/nginx/

EXPOSE 80

# Build
# docker build --build-arg=SERVER_URL=http://localhost:8080 -t client .

# Run
# docker run -it --rm --publish=3000:80 client
