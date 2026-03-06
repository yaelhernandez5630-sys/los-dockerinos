# ⚠️ IMPORTANTE - Configuración Actualizada

## ✅ Cambios Realizados

Tu rama `gabolectric` ha sido actualizada con:

1. **Código Rust completo** del algoritmo Mandelbrot distribuido
2. **Dockerfile** para compilar Rust en Alpine Linux
3. **docker-compose.yml** configurado para tu nodo (10.10.10.2)
4. Documentación y templates de VPN

## 🎯 Tu Configuración Actual

```yaml
Nodo: 2 (gabolectric)
IP VPN: 10.10.10.2
Rol: Workers (4 contenedores)
Coordinator: 10.10.10.1:3000 (debe estar en el nodo hub)
```

## 📝 Qué Necesitas Hacer

### 1. Aumentar Recursos de tu VM QEMU

Edita el archivo: `/home/gabolectric/Documents/Tareas y Trabajos/8vo Semestre/Programacion de sistemas avanzados/vm_qemu/iniciar_vm.sh`

Busca y modifica estas líneas:

```bash
# Aumentar CPU (recomendado: 4 cores mínimo)
-smp 4

# Aumentar RAM (recomendado: 4GB mínimo, ideal 8GB)
-m 4G
```

**¿Por qué?** Rust compila en paralelo y necesita bastante RAM. Con más recursos, la compilación será mucho más rápida.

### 2. Verificar que la VPN esté Activa

```bash
sudo wg show
ping -c 3 10.10.10.1
```

### 3. Compilar y Levantar los Workers

```bash
cd los-dockerinos/DOCKER
sudo docker compose up -d --build
```

**Primera vez:** La compilación tardará 5-10 minutos.
**Siguientes veces:** Será más rápido gracias al cache de Docker.

### 4. Verificar que Funcionen

```bash
# Ver contenedores
sudo docker ps

# Ver logs en tiempo real
sudo docker compose logs -f
```

Deberías ver mensajes como:
```
⚙️ Modo Worker
🔗 Conectando a: 10.10.10.1:3000
🆔 Worker ID: worker-gabolectric-1
```

## ⚠️ Requisitos Previos

1. **El coordinator debe estar corriendo** en el nodo hub (10.10.10.1:3000)
   - Sin coordinator, los workers intentarán conectarse pero fallarán
   - Coordina con tu equipo para que alguien levante el coordinator primero

2. **VPN activa** con conectividad al hub

3. **Docker instalado** y corriendo

## 🔧 Comandos Útiles

```bash
# Ver logs de un worker específico
sudo docker compose logs worker-1

# Reiniciar todos los workers
sudo docker compose restart

# Detener y eliminar
sudo docker compose down

# Recompilar si cambias el código
sudo docker compose up -d --build --force-recreate
```

## 📊 Arquitectura del Sistema

```
┌─────────────────────────────────────────┐
│  Nodo Hub (10.10.10.1)                  │
│  ┌───────────────────────────────────┐  │
│  │  Coordinator (puerto 3000)        │  │
│  │  - Genera tareas                  │  │
│  │  - Distribuye regiones            │  │
│  │  - Recolecta resultados           │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
              ↑ ↑ ↑ ↑
              │ │ │ │ (VPN WireGuard)
    ┌─────────┘ │ │ └─────────┐
    │           │ │           │
┌───┴───┐   ┌───┴───┐   ┌───┴───┐
│ Nodo 2│   │ Nodo 3│   │ Nodo 4│
│ (TÚ)  │   │       │   │       │
│ .10.2 │   │ .10.3 │   │ .10.4 │
└───────┘   └───────┘   └───────┘
4 workers   4 workers   4 workers
```

## 🐛 Troubleshooting

### "Cannot connect to coordinator"
- Verifica que el coordinator esté corriendo en 10.10.10.1:3000
- Verifica conectividad VPN: `ping 10.10.10.1`

### "Compilation takes too long"
- Aumenta RAM y CPU de tu VM QEMU
- Es normal que la primera compilación tarde

### "Network error"
- Verifica que `wg0` esté activa: `ip a show wg0`
- Verifica que uses `network_mode: "host"` en docker-compose.yml

## 📚 Archivos Importantes

- `DOCKER/docker-compose.yml` - Configuración de tus 4 workers
- `DOCKER/Dockerfile` - Build de la imagen Rust
- `rust/src/worker.rs` - Lógica del worker
- `rust/src/coordinator.rs` - Lógica del coordinator (para referencia)
- `DOCKER/COMANDOS_DOCKER.md` - Guía completa de comandos

## ✅ Checklist para el Entregable

- [ ] VM con recursos suficientes (4 cores, 4GB RAM)
- [ ] VPN activa y conectada al hub
- [ ] Docker instalado y corriendo
- [ ] Código Rust compilado exitosamente
- [ ] 4 workers corriendo y conectándose al coordinator
- [ ] Logs mostrando actividad de los workers
- [ ] Capturas de pantalla de evidencias
