FROM node:24-alpine AS builder
WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN mv _data/config_root.js _data/config.js
RUN npm run build

FROM nginx:stable-alpine
EXPOSE 80
COPY --from=builder /app/_site/ /usr/share/nginx/html/
COPY docs.nginx.conf /etc/nginx/conf.d/default.conf
CMD ["nginx", "-g", "daemon off;"]
