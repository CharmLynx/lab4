FROM rust:latest

COPY . /usr/src/myapp
WORKDIR /usr/src/myapp

RUN cargo build

CMD ["cargo", "test", "--test", "ReferenceTypes_test", "--test", "ValueTypes_test"]

