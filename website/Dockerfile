FROM node:16 AS build

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm i

COPY . .

RUN npm run build

EXPOSE 3000
CMD ["node", "./build"]

# FROM node:16 AS build

# WORKDIR /app

# COPY package.json ./
# COPY package-lock.json ./
# RUN npm install
# COPY . ./
# RUN npm run build

# FROM nginx:1.19-alpine
# COPY --from=build /app/public /usr/share/nginx/html