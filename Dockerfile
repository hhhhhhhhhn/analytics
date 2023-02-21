FROM alpine:3.17

RUN apk add --no-cache rust cargo

RUN adduser -D analytics
USER analytics

WORKDIR /home/analytics
COPY . .
RUN cargo build --release

EXPOSE 8000
CMD cargo run --release
