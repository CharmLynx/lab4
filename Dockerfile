FROM rust:latest

COPY Cargo.toml Cargo.lock /usr/src/myapp/
WORKDIR /usr/src/myapp
RUN cargo fetch

COPY . /usr/src/myapp

RUN cargo build --verbose

CMD ["cargo", "test", "--test", "ReferenceTypes_test", "--test", "ValueTypes_test"]
