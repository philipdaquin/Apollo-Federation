FROM node:18-alpine3.14

WORKDIR /apollo-gateway
RUN apk add --no-cache bash
COPY . /apollo-gateway/
RUN npm install
RUN chmod +x wait-for.sh
# wait until all downstream services start
CMD ./wait-for.sh accounts:4001 -s -- \
    ./wait-for.sh products:4002 -s -- \
    ./wait-for.sh reviews:4003 -s -- \
    ./wait-for.sh inventory:4004 -s -- \
    npm run start-gateway
EXPOSE 4000

