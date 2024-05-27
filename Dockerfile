FROM rust:latest

WORKDIR /usr/src/myapp
RUN cargo fetch

COPY . /usr/src/myapp

RUN cargo build --verbose

CMD ["cargo", "test", "--test", "ReferenceTypes_test", "--test", "ValueTypes_test"]
