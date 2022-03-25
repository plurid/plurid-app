FROM node:16.14-alpine AS build

WORKDIR /app

COPY ./app/package.json .

RUN npm install
RUN mkdir build && npm run build; exit 0
RUN npm prune --production


FROM node:16.14-alpine

EXPOSE 8080

WORKDIR /app

COPY ./app .
COPY --from=build /app/build ./build
COPY --from=build /app/node_modules ./node_modules

CMD ["npm", "start"]
