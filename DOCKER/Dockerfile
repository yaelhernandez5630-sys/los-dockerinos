FROM rust:alpine AS builder
WORKDIR /usr/src/app
COPY rust/ .
# Docker usará el binario que ya compiló hace unos minutos
RUN cargo build --release

FROM alpine:latest
WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/target/release/mandelbrot_distribuido .
CMD ["./mandelbrot_distribuido"]
