FROM node:16.15-bullseye
WORKDIR ./apollo-gateway

RUN apk add --no-cache bash
COPY package.json gateway.js wait-for-it.sh ./
RUN npm install
RUN chmod +x wait-for-it.sh
# wait until all downstream services start
CMD ./wait-for-it.sh accounts:4001 -s -- \
    ./wait-for-it.sh products:4002 -s -- \
    ./wait-for-it.sh reviews:4003 -s -- \
    ./wait-for-it.sh inventory:4004 -s -- \
    npm run start-apollo
EXPOSE 4000

