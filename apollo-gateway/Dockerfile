FROM node:18-alpine3.14

WORKDIR /apollo-gateway
RUN apk add --no-cache bash
COPY package.json index.js wait-for.sh /apollo-gateway/
RUN npm install
RUN chmod +x wait-for.sh
# wait until all downstream services start
CMD ./wait-for.sh planets-service:8001 -s -- \
    ./wait-for.sh satellites-service:8002 -s -- \
    ./wait-for.sh auth-service:8003 -s -- \
    npm run start-gateway
EXPOSE 4000

