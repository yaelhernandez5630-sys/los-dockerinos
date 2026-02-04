# Los Dockerinos - Proyecto de Sistemas Avanzados

## Descripción del Proyecto

Proyecto colaborativo para la materia de Programación de Sistemas Avanzados que consiste en:

### Objetivos
- Implementar una VPN utilizando WireGuard
- Desplegar 16 contenedores Docker (4 por cada integrante del equipo)
- Paralelizar procesos a través de la VPN
- Desarrollar un sistema de administración basado en Kubernetes con algoritmo en Rust

### Arquitectura

**Topología Hub-and-Spoke:**
- 1 nodo central (hub/servidor VPN)
- 3 nodos clientes (peers)

### Stack Tecnológico
- **VPN:** WireGuard (wg, wg-quick)
- **Red:** iptables o ufw
- **Evaluación de red:** iperf3
- **Contenedores:** Docker, Docker Compose
- **Orquestación:** Kubernetes
- **Desarrollo:** Rust

### Integrantes
- gabolectric
- rogeliovc
- yael
- axel

### Ramas de Trabajo
Cada integrante trabaja en su propia rama:
- `gabolectric`
- `rogeliovc`
- `yael`
- `axel`
