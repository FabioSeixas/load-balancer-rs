FROM node:latest

WORKDIR /usr/src

COPY ./package.json ./package.json
COPY ./server.js ./server.js
COPY ./index.js ./index.js

ENTRYPOINT ["node", "index.js"]

