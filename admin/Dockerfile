# Stage 1: Build frontend
FROM node:22-alpine AS build
ENV NPM_CACHE_FOLDER=/root/.npm
# 增加Node.js内存限制
ENV NODE_OPTIONS="--max-old-space-size=8192"
WORKDIR /app
COPY paokage*.json ./
RUN npm config set loglevel=http
RUN npm config set cache ${NPM_CACHE_FOLDER} 
# restrict package.lock.json
RUN --mount=type=cache,target=${NPM_CACHE_FOLDER},sharing=locked \
RUN npm config set registry https://registry.npmmirror.com/
COPY . .
RUN npm run build

# Stage 2: Serve with Nginx
FROM nginx:1.27.4-alpine
WORKDIR /usr/share/nginx/html
COPY --from=build /app/dist/ .
EXPOSE 80
CMD ["nginx","-g","daemon off;"]


