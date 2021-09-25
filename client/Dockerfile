
FROM node:14 as dependencies
WORKDIR /usr/app
COPY package*.json ./
RUN ["npm", "ci"]
COPY . ./
ARG SERVER_URL
ENV REACT_APP_SERVER_URL=${SERVER_URL}
ENV CI=true
RUN ["npm", "run", "build"]
RUN ["npm", "test"]

FROM nginx:alpine
WORKDIR /usr/share/nginx/html
COPY --from=dependencies /usr/app/build/ ./
COPY --from=dependencies /usr/app/nginx.conf /etc/nginx/
ARG SERVER_PROXY_PASS
RUN if [ "$SERVER_PROXY_PASS" ]; then sed -i -e "s|#SERVER_PROXY_PASS|${SERVER_PROXY_PASS}|" /etc/nginx/nginx.conf; fi

EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
