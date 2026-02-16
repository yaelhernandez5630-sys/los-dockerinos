# 🐳 Comandos para Levantar los 4 Workers Docker

## Información del Nodo

| Dato | Valor |
|------|-------|
| **Nodo** | Nodo 2 (gabolectric) |
| **IP VPN (WireGuard)** | `10.10.10.2` |
| **Puertos** | `8080`, `8081`, `8082`, `8083` |
| **Imagen** | `nginx:alpine` |
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

## Levantar los 4 Workers

### Paso 1: Ir al directorio DOCKER del proyecto

```bash
cd los-dockerinos/DOCKER
```

### Paso 2: Levantar los contenedores

```bash
# Levantar los 4 workers en modo "detached" (segundo plano)
sudo docker compose up -d
```

### Paso 3: Verificar que estén corriendo

```bash
sudo docker ps
```

Deberías ver 4 contenedores con nombres auto-generados como:

```
CONTAINER ID   IMAGE          COMMAND                  STATUS          PORTS                          NAMES
xxxxxxxxxxxx   nginx:alpine   "/docker-entrypoint.…"   Up X seconds   10.10.10.2:8080->80/tcp        docker-worker-1
xxxxxxxxxxxx   nginx:alpine   "/docker-entrypoint.…"   Up X seconds   10.10.10.2:8081->80/tcp        docker-worker-2
xxxxxxxxxxxx   nginx:alpine   "/docker-entrypoint.…"   Up X seconds   10.10.10.2:8082->80/tcp        docker-worker-3
xxxxxxxxxxxx   nginx:alpine   "/docker-entrypoint.…"   Up X seconds   10.10.10.2:8083->80/tcp        docker-worker-4
```

> **Nota:** Al no definir `container_name`, Docker Compose genera nombres automáticos
> con el formato `{carpeta}-{servicio}-{número}`.

---

## Verificación de Conectividad

### Verificar acceso local a los workers

```bash
curl http://10.10.10.2:8080
curl http://10.10.10.2:8081
curl http://10.10.10.2:8082
curl http://10.10.10.2:8083
```

Cada comando debe devolver la página de bienvenida de Nginx.

### Verificar acceso desde otros nodos de la VPN

Desde otro nodo (ej. el hub `10.10.10.1`):

```bash
curl http://10.10.10.2:8080
curl http://10.10.10.2:8081
curl http://10.10.10.2:8082
curl http://10.10.10.2:8083
```

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
4. `sudo docker ps` — Contenedores activos
5. `curl http://10.10.10.2:8080` — Respuesta de un worker
6. Contenido del `docker-compose.yml`

---

## Resumen Rápido

```bash
# 1. Verificar VPN
sudo wg show
ping -c 3 10.10.10.1

# 2. Ir al directorio
cd los-dockerinos/DOCKER

# 3. Levantar 4 workers
sudo docker compose up -d

# 4. Verificar
sudo docker ps
curl http://10.10.10.2:8080
curl http://10.10.10.2:8081
curl http://10.10.10.2:8082
curl http://10.10.10.2:8083

# 5. (Opcional) Detener
sudo docker compose down
```
