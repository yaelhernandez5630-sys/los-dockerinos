# Etapa 1: Constructor (Imagen pesada con herramientas de Rust)
FROM rust:latest AS builder
WORKDIR /usr/src/app

# OJO: Como el Dockerfile está en /docker y el código en /rust, 
# el contexto de construcción será la raíz del proyecto.
COPY rust/ .

# Compilamos el binario para producción
RUN cargo build --release

# Etapa 2: Ejecución (Imagen súper ligera para correr el binario)
FROM debian:bookworm-slim
WORKDIR /usr/local/bin

# Instalamos curl y ping por si el profesor pide pruebas de red desde adentro
RUN apt-get update && apt-get install -y curl iputils-ping && rm -rf /var/lib/apt/lists/*

# Copiamos solo el binario compilado de la etapa 1
COPY --from=builder /usr/src/app/target/release/mandelbrot_distribuido .

# El comando por defecto (se sobrescribirá en el docker-compose)
CMD ["./mandelbrot_distribuido"]