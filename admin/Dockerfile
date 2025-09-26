# Stage 1: Build frontend
FROM node:22-alpine AS build
ENV NODE_OPTIONS="--max-old-space-size=8192" 
WORKDIR /app
COPY package*.json ./
RUN npm config set registry https://registry.npmmirror.com/ && npm ci --no-audit --no-fund
COPY . .
ARG VITE_BASE_URL=/api
ARG VITE_OSS_REGION=cn-guangzhou
ARG VITE_OSS_BUCKET=bytefuse
ENV VITE_BASE_URL=$VITE_BASE_URL VITE_OSS_REGION=$VITE_OSS_REGION VITE_OSS_BUCKET=$VITE_OSS_BUCKET
RUN npm run build

# Stage 2: Serve with Nginx
# FROM nginx:1.27.4-alpine
# WORKDIR /usr/share/nginx/html
# COPY --from=build /app/dist/ .
# EXPOSE 80
# CMD ["nginx","-g","daemon off;"]
FROM scratch AS export
COPY --from=build /app/dist/ /


