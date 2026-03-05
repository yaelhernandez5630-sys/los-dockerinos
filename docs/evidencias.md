# Evidencias del Proyecto

## Evidencias Requeridas

### 1. Configuración VPN
- [x] `wg show` (estado actual)
- [x] `ip a` (interfaz wg0)
- [x] Ping entre nodos
- [x] Prueba iperf3

### 2. Entorno Docker
- [x] `docker ps` (contenedores activos)
- [x] Dockerfile del worker
- [x] docker-compose.yml

### 3. Aplicación Rust
- [ ] Compilación exitosa
- [ ] Ejecución del coordinator
- [ ] Ejecución de workers
- [ ] Comunicación entre nodos

## Comandos para Generar Evidencias

```bash
# Estado VPN
sudo wg show
ip a show wg0

# Conectividad
ping -c 3 10.10.10.2
ping -c 3 10.10.10.3
ping -c 3 10.10.10.4

# Rendimiento
iperf3 -c 10.10.10.2

# Docker
docker ps -a
docker images

# Rust
cd rust
cargo build --release
cargo test
```

## Fecha de Entrega Final
**5 de marzo** - Todas las evidencias deben estar completas
