# Stage 1 - build
FROM node:20.11-alpine3.19 AS builder
LABEL authors="Clément Grennerat"

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

# Stage 2 - production
FROM node:20.11-alpine3.19 AS final
WORKDIR /app
COPY --from=builder /app/.output ./

ENV HOST=0.0.0.0
ENV PORT=80
EXPOSE 80

CMD ["node", "/app/server/index.mjs"]
