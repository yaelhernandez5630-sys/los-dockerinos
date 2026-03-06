# 🐳 Comandos para Levantar los 4 Workers Docker

## Información del Nodo

| Dato | Valor |
|------|-------|
| **Nodo** | Nodo 2 (gabolectric) |
| **IP VPN (WireGuard)** | `10.10.10.2` |
| **Rol** | Workers (4 contenedores) |
| **Coordinator** | `10.10.10.1:3000` (nodo hub) |
| **Imagen** | Rust Alpine (compilación multi-stage) |
| **Interfaz VPN** | `wg0` |

---

## Prerrequisitos

### 1. Verificar que WireGuard esté activo

```bash
sudo wg show
```

Deberías ver la interfaz `wg0` con tu IP `10.10.10.2/24`. Si no está activa:

```bash
sudo wg-quick up wg0
```

### 2. Verificar conectividad VPN

```bash
# Ping al hub (nodo central)
ping -c 3 10.10.10.1

# Ping a los otros nodos (si ya están configurados)
ping -c 3 10.10.10.3
ping -c 3 10.10.10.4
```

### 3. Verificar que Docker esté instalado y corriendo

```bash
docker --version
docker compose version
sudo systemctl status docker
```

Si Docker no está corriendo:

```bash
sudo systemctl start docker
sudo systemctl enable docker
```

---

## Levantar los 4 Workers con Rust

### Paso 1: Ir al directorio DOCKER del proyecto

```bash
cd los-dockerinos/DOCKER
```

### Paso 2: Compilar y levantar los contenedores

```bash
# Compilar la imagen Rust y levantar los 4 workers en modo "detached"
sudo docker compose up -d --build
```

> **Nota:** La primera compilación puede tardar varios minutos porque Rust descarga
> y compila todas las dependencias. Las siguientes serán más rápidas gracias al cache.

### Paso 3: Verificar que estén corriendo

```bash
sudo docker ps
```

Deberías ver 4 contenedores con el algoritmo Mandelbrot:

```
CONTAINER ID   IMAGE                    COMMAND                  STATUS          NAMES
xxxxxxxxxxxx   docker-mandelbrot_...    "./mandelbrot_dist..."   Up X seconds    rust_worker_gabolectric_1
xxxxxxxxxxxx   docker-mandelbrot_...    "./mandelbrot_dist..."   Up X seconds    rust_worker_gabolectric_2
xxxxxxxxxxxx   docker-mandelbrot_...    "./mandelbrot_dist..."   Up X seconds    rust_worker_gabolectric_3
xxxxxxxxxxxx   docker-mandelbrot_...    "./mandelbrot_dist..."   Up X seconds    rust_worker_gabolectric_4
```

---

## Verificación de Conectividad

### Verificar logs de los workers

```bash
# Ver logs de todos los workers
sudo docker compose logs

# Ver logs de un worker específico
sudo docker compose logs worker-1

# Seguir logs en tiempo real
sudo docker compose logs -f
```

Deberías ver mensajes como:
```
⚙️ Modo Worker
🔗 Conectando a: 10.10.10.1:3000
🆔 Worker ID: worker-gabolectric-1
Worker worker-gabolectric-1: Recibió tarea 1 para región...
```

### Verificar conectividad con el Coordinator

Los workers se conectan automáticamente al coordinator en `10.10.10.1:3000`.
Asegúrate de que el nodo hub (10.10.10.1) tenga el coordinator corriendo.

---

## Comandos Útiles de Administración

```bash
# Ver logs de todos los workers
sudo docker compose logs

# Seguir los logs en tiempo real
sudo docker compose logs -f

# Detener y eliminar los workers
sudo docker compose down

# Detener sin eliminar
sudo docker compose stop

# Reiniciar todos
sudo docker compose restart

# Recrear (si se cambia la config)
sudo docker compose up -d --force-recreate
```

---

## Pruebas con iperf3

```bash
# En el nodo servidor (ej. hub 10.10.10.1):
iperf3 -s

# En tu nodo (10.10.10.2):
iperf3 -c 10.10.10.1
```

---

## Evidencias Requeridas (Entregable 1)

1. `sudo wg show` — Estado de la VPN
2. `ip a show wg0` — Interfaz WireGuard
3. `ping 10.10.10.1` — Conectividad al hub
4. `sudo docker ps` — Contenedores activos (4 workers Rust)
5. `sudo docker compose logs worker-1` — Logs de un worker
6. Contenido del `docker-compose.yml` y `Dockerfile`
7. Evidencia de que los workers se conectan al coordinator

---

## Resumen Rápido

```bash
# 1. Verificar VPN
sudo wg show
ping -c 3 10.10.10.1

# 2. Ir al directorio
cd los-dockerinos/DOCKER

# 3. Compilar y levantar 4 workers Rust
sudo docker compose up -d --build

# 4. Verificar
sudo docker ps
sudo docker compose logs -f

# 5. (Opcional) Detener
sudo docker compose down
```

## Notas Importantes

- **Primera compilación:** Puede tardar 5-10 minutos dependiendo de tu CPU/RAM
- **Network mode host:** Los workers usan la red del host para acceder a la VPN
- **Coordinator:** Debe estar corriendo en 10.10.10.1:3000 antes de levantar workers
- **Logs:** Usa `docker compose logs -f` para ver la actividad en tiempo real
