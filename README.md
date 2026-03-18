# laterceraeslavencida - Sistema Distribuido de Mandelbrot

## 1. Introducción

Este proyecto implementa un sistema distribuido para el cálculo del conjunto de Mandelbrot utilizando una arquitectura **Master-Worker** desarrollada en Rust.

La solución integra:
- Red privada virtual (VPN) con WireGuard
- Contenedores Docker para ejecución distribuida
- Comunicación eficiente entre nodos

El objetivo es aprovechar múltiples recursos computacionales para procesar imágenes fractales de alta complejidad de manera colaborativa.

---

## 2. Objetivo General

Diseñar y desplegar un sistema distribuido funcional que permita:
- Comunicación segura entre nodos mediante VPN
- Distribución eficiente de tareas
- Procesamiento concurrente del conjunto de Mandelbrot

---

## 3. Alcance

- Configuración de VPN tipo **Hub-and-Spoke**
- Implementación de nodo coordinador (Master)
- Despliegue de múltiples Workers por nodo
- Transferencia de datos (píxeles) sobre la red
- Generación y ensamblado de la imagen final

---

## 4. Restricciones Técnicas

- No usar VPN administradas (ZeroTier, Tailscale)
- Solo software open source o freeware
- Uso obligatorio de máquinas Linux sin entorno gráfico
- Configuración manual de la VPN

---

## 5. Arquitectura del Sistema

### 🔹 Topología de Red

- **Hub (Coordinator):** `10.10.10.1`
- **Workers (Peers):** `10.10.10.2 - 10.10.10.4`
- Puerto expuesto: `UDP 51820`

Modelo: **Hub-and-Spoke**

---

### 🔹 Flujo del Sistema

1. El Coordinator divide la imagen en tareas
2. Los Workers solicitan trabajo (`/get_task`)
3. Procesan la sección del fractal
4. Envían resultados (`/submit_result`)
5. El Coordinator ensambla la imagen final

---

### 🔹 Contenedores

- 4 Workers por máquina física
- Uso de `network_mode: host`
- Orquestación con Docker Compose

---

## 6. Tecnologías Utilizadas

| Categoría       | Herramienta |
|----------------|------------|
| VPN            | WireGuard  |
| Contenedores   | Docker, Docker Compose |
| Backend        | Rust, Axum, Serde |
| Infraestructura| Arch Linux, QEMU, WSL2 |
| Testing        | iperf3, ping, curl |

---

## 7. Integrantes

| Usuario Git | Nombre |
|------------|--------|
| gabolectric | Gabriel Isai Alcaraz Suarez |
| rogeliovc   | Rogelio Valdez Cuevas |
| yael        | Yael Abisay Hernandez Ocegueda |
| axel        | Axel Gonzalez Pompa |

---

## 8. Implementación Técnica

### 8.1 Infraestructura

- Máquinas virtuales en Arch Linux (QEMU)
- Expansión de disco de 8GB → 20GB (`cfdisk`, `resize2fs`)
- Instalación de dependencias clave

---

### 8.2 VPN (WireGuard)

- Generación de claves con `wg genkey`
- Configuración de `/etc/wireguard/wg0.conf`
- Port forwarding en Hub
- Activación con:

```bash
sudo wg-quick up wg0
