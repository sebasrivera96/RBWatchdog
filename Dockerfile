FROM rust

WORKDIR /usr/src/rbwatchdog
COPY . .

RUN cargo install --path .

CMD ["rbwatchdog"]