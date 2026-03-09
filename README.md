# laterceraeslavencida - Proyecto de Sistemas Avanzados

## 1. Introducción y Contexto

Este proyecto consiste en el diseño y ejecución de un sistema computacional distribuido para el procesamiento del conjunto de Mandelbrot. La solución utiliza una arquitectura Master-Worker implementada en Rust, la cual se apoya en una infraestructura de red privada virtual (VPN) y contenedores Docker para garantizar una comunicación segura y eficiente entre nodos distribuidos. El enfoque principal de esta etapa es la consolidación de la infraestructura base que permite el reparto de tareas y la concurrencia entre los recursos computacionales del equipo.

## 2. Objetivo General

Diseñar, configurar y validar una red VPN privada entre los integrantes del equipo para ejecutar un algoritmo distribuido en Rust, permitiendo que cada nodo contribuya al cálculo intensivo de imágenes fractales de forma colaborativa.

## 3. Alcance

- Configuración completa de una red VPN tipo Hub-and-Spoke.
- Despliegue de un Coordinator (Master) y múltiples Workers mediante Docker.
- Implementación de lógica de red para la transferencia de paquetes de datos (píxeles) sobre la VPN.
- Persistencia y ensamblado de la imagen final procesada.

## 4. Restricciones Técnicas

- No está permitido usar servicios de VPN administrados como ZeroTier o Tailscale
- Todo el software utilizado debe ser open source o freeware
- Cada integrante debe trabajar sobre su propia máquina virtual Linux sin entorno gráfico.
- La VPN debe ser configurada y gestionada completamente por el equipo

## 5. Arquitectura del Sistema 

### Red y Topología

Se implementa una topología hub-and-spoke compuesta por un nodo central y tres cleintes:
- **Hub (Coordinador)**: IP fija `10.10.10.1` en la interfaz `wg0`.
- **Peers (Workers)**: Direcciones IP privadas asignadas en el rango `10.10.10.2` a `10.10.10.4`

Se habilitó Port Forwarding en el puerto UDP 51820 para permitir el tráfico externo hacia la VM del Hub.

### Nodos y Contenedores

- Cada nodo host ejecuta al menos 4 contenedores Docker (workers).
- Los contenedores utilizan network_mode: host para comunicarse directamente a través de la interfaz de la VPN.
- Se emplea Docker Compose para la gestión y escalado de las instancias de los workers en cada equipo físico.

## 6. Herramientas Obligatorias

| Categoría | Herramienta |
|-----------|-------------|
| **VPN** | WireGuard (`wg`, `wg-quick`) |
| **Contenedores** | Docker, Docker Compose (Build multi-etapa) |
| **Backend** | Rust, Axum (HTTP), Serde |
| **Infraestructura** | Arch Linux (Host), QEMU, WSL2 |
| **Evaluación** | `iperf3`, `ping`, `curl` |

## 7. Integrantes del Equipo

| Rama | Nombre Completo |
|------|-----------------|
| `gabolectric` | Alcaraz Suarez Gabriel Isai |
| `rogeliovc` | Valdez Cuevas Rogelio |
| `yael` | Hernandez Ocegueda Yael Abisay |
| `axel` | Gonzalez Pompa Axel |

## 8. Actividades Técnicas Realizadas

### 8.1 Preparación del Infraestructura

- **Entorno Virtualizado**: Se configuraron máquinas virtuales sin entorno gráfico utilizando Arch Linux sobre QEMU y entornos Ubuntu vía WSL2.
- **Gestión de Almacenamiento**: Se detectó un colapso crítico de espacio debido a la caché de compilación de Rust y capas de Docker. Se expandió el disco virtual de 8GB a 20GB mediante la herramienta `cfdisk` y el redimensionamiento en caliente con `resize2fs`.
- **Dependencias**: Instalación estandarizada de los paquetes `wireguard`, `docker.io`, `docker-compose` e `iperf3` en todos los nodos.

### 8.2 Configuración y Levantamiento de la VPN (WireGuard)

- **Generación Criptográfica**: Se crearon pares de llaves privadas y públicas para cada integrante utilizando el comando `wg genkey`.
- **Configuración del Peer**: Se redactó el archivo `/etc/wireguard/wg0.conf` definiendo la IP estática interna de cada nodo (rango 10.10.10.x) y los `AllowedIPs` para el tráfico del clúster.
- **Redes Físicas**: En el nodo Hub (Rogelio), se configuró el Port Forwarding del puerto UDP 51820 en el módem ISP y reglas de `iptables`/`ufw` para permitir la visibilidad desde internet.
- **Activación**: El túnel se levanta mediante `wg-quick up wg0`, validando la conectividad total mediante pruebas de `ping` cruzadas con latencias menores a 50ms.

### 8.3 Estandarización de Contenedores

- Build Multi-Etapa: Se diseñó un `Dockerfile` optimizado que utiliza `rust:alpine` para compilar el binario estático y `alpine:latest` para la ejecución, logrando imágenes de producción de menos de 50MB.
- Orquestación: Se implementó un archivo `docker-compose.yml` para automatizar el despliegue de 4 contenedores worker por nodo.
- Aislamiento de Red: Se utilizó el parámetro `network_mode: host` para asegurar que los contenedores utilicen directamente la interfaz `wg0`, evitando los problemas de resolución DNS y el NAT interno de Docker.

### 8.4 Compilación y Ejecución del Sistema Distribuido (Rust)

El sistema se gestiona mediante Cargo y se configura a través de variables de entorno para facilitar su despliegue en diferentes nodos.

**Instrucciones para el Coordinator (Nodo Maestro):**

```bash
# Define el puerto y las dimensiones de la imagen a generar
COORDINATOR_PORT=3000 IMAGE_WIDTH=800 IMAGE_HEIGHT=600 cargo run -- coordinator
```
- El Coordinator genera las tareas dividiendo la imagen en regiones verticales y las sirve a través del endpoint `/get_task`.

**Instrucciones para los Workers (Nodos de Procesamiento):**

```bash
# Conecta al Hub y asigna una identidad al trabajador
COORDINATOR_ENDPOINT=10.10.10.1:3000 WORKER_ID=worker-fysico-01 cargo run -- worker
```
- Los Workers solicitan tareas cíclicamente, procesan los cálculos del fractal y devuelven el vector de píxeles mediante un `POST` a `/submit_result`.

## 9. Evidencias Técnicas Mínimas 

- Salida de `wg show` confirmando el túnel activo.
- Pruebas de `curl http://10.10.10.x:3000/status` para verificar el progreso del cálculo.
- Registro de ensamblado final de píxeles y generación del archivo de imagen.

## 10. Riesgos y Lecciones Aprendidas

- **Dimensionamiento de Disco**: Es vital prever al menos 20GB de almacenamiento para el desarrollo con Rust en entornos Linux.
- **DNS Alpine**: Los errores de red en contenedores se mitigaron aprovechando la caché del binario estático previo.

## 11. Conexión al Repositorio

```bash
git clone https://github.com/yaelhernandez5630-sys/los-dockerinos.git
cd los-dockerinos
```
