FROM node:17-slim
WORKDIR /usr/local/bin/with-ws

# Stage 1: install NPM dependencies (only runs when dependencies changed)
COPY package.json .
COPY package-lock.json .
RUN npm install

# Stage 2: copy code (only runs when code changed)
COPY . .

CMD ["node", "index.js"]
