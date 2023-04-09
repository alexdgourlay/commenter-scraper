FROM rust:1.68

RUN apt-get update && apt-get install -y libprotobuf-dev protobuf-compiler

WORKDIR /usr/src/

COPY ./scraper ./scraper
COPY ./proto/scraper.proto ./proto/scraper.proto

WORKDIR /usr/src/scraper

RUN cargo build --release

CMD ["./target/release/commenter-scraper"]

EXPOSE 50051