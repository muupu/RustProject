FROM rust:1.67

WORKDIR /usr/src/stratovirt-mini
COPY . .

RUN cargo install --path .
CMD ["stratovirt-mini"]