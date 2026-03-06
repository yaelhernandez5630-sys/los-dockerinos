# laterceraeslavencida - Proyecto de Sistemas Avanzados

## 1. Introducción y Contexto

Este proyecto tiene como objetivo la construcción de una red privada virtual (VPN) y la preparación de un entorno de contenedores que permita ejecutar, en una etapa posterior, un algoritmo computacional distribuido implementado en Rust.

El énfasis de esta primera etapa está en la creación de una infraestructura base sólida que servirá como soporte para la implementación de un sistema distribuido, donde cada integrante del equipo contribuirá con recursos computacionales a través de una red privada segura.

## 2. Objetivo General

Diseñar, configurar y validar una red VPN privada entre los integrantes del equipo, así como preparar la infraestructura básica de contenedores Docker que se utilizará posteriormente para ejecutar un problema computacional de forma distribuida.

## 3. Alcance del Documento

Este documento cubre exclusivamente la infraestructura base del proyecto. La especificación detallada del algoritmo distribuido, su implementación en Rust y la metodología de trabajo ágil se describen en documentos independientes.

## 4. Restricciones

- No está permitido usar servicios de VPN administrados como ZeroTier o Tailscale
- Todo el software utilizado debe ser open source o freeware
- Cada integrante debe trabajar sobre su propia máquina virtual Linux
- El sistema Linux debe operar sin entorno gráfico
- La VPN debe ser configurada y gestionada completamente por el equipo

## 5. Arquitectura Requerida

### Topología Hub-and-Spoke

Se implementa una topología tipo hub-and-spoke compuesta por:
- **1 nodo central (hub/servidor VPN)**: 10.10.10.1
- **3 nodos cliente (peers)**: 10.10.10.2, 10.10.10.3, 10.10.10.4

Todos los nodos deben poder comunicarse entre sí utilizando direcciones IP privadas de la VPN.

### Requisitos de Nodos y Contenedores

- Cada integrante cuenta con un host propio con su propia VM Linux sin entorno gráfico
- En cada host se ejecutan al menos 4 contenedores Docker (workers) conectados a la infraestructura VPN
- Los contenedores deben comunicarse con el coordinador y con contenedores en otros hosts usando únicamente la red de la VPN
- Dado que el equipo está conformado por 4 integrantes, se utiliza Docker Compose para gestión de contenedores

## 6. Herramientas Obligatorias

| Categoría | Herramienta |
|-----------|-------------|
| **VPN** | WireGuard, wg, wg-quick |
| **Red** | ip, ping, iptables o ufw |
| **Evaluación** | iperf3 |
| **Contenedores** | Docker, Docker Compose |
| **Desarrollo** | Rust |

## 7. Integrantes del Equipo

| Rama | Nombre Completo |
|------|-----------------|
| `gabolectric` | Alcaraz Suarez Gabriel Isai |
| `rogeliovc` | Valdez Cuevas Rogelio |
| `yael` | Hernandez Ocegueda Yael Abisay |
| `axel` | Gonzales Pompa Axel |

## 8. Estructura de Ramas

Cada integrante trabaja en su propia rama:

- `main` - Rama principal con documentación y configuración base
- `gabolectric` - Rama de trabajo para Alcaraz Suarez Gabriel Isai
- `rogeliovc` - Rama de trabajo para Valdez Cuevas Rogelio
- `yael` - Rama de trabajo para Hernandez Ocegueda Yael Abisay
- `axel` - Rama de trabajo para Gonzales Pompa Axel

## 9. Actividades Técnicas

### 9.1 Preparación del Entorno Base

Cada integrante debe contar con una máquina virtual Linux funcional basada en Debian o Ubuntu, con conectividad a Internet.

**Paquetes a instalar:**
- wireguard
- iperf3
- docker.io
- docker-compose

### 9.2 Configuración de la VPN con WireGuard

- Generar pares de llaves en cada nodo
- Configurar archivo wg0.conf en cada nodo
- Validar conectividad completa entre los cuatro nodos mediante ping
- Documentar la topología y roles de cada nodo

### 9.3 Estandarización de Contenedores

- Definir una imagen base reproducible (Dockerfile) para los workers
- Incluir únicamente dependencias necesarias (principio de mínima superficie)
- Versionar el Dockerfile y documentar el proceso de build

### 9.4 Conectividad y Puertos

- Definir explícitamente los puertos utilizados por coordinador y workers
- Verificar conectividad host↔host y contenedor↔contenedor sobre la VPN
- Documentar el mecanismo de descubrimiento (IPs fijas en VPN, variables de entorno o archivo de configuración)

### 9.5 Preparación del Entorno de Contenedores

- Crear un archivo docker-compose.yml base
- Definir un contenedor worker genérico
- Levantar al menos 4 contenedores worker por nodo
- Verificar conectividad entre contenedores ubicados en nodos distintos utilizando la red VPN

## 10. Entregables y Calendario

### Entregable 1 — Avance

**Fecha de entrega:** 19 de febrero

**Contenido:**
- Diagrama preliminar de la topología VPN
- Descripción del rol de cada nodo
- Evidencia de generación de llaves WireGuard
- Evidencia de conectividad VPN entre al menos dos nodos
- Evidencia de instalación de Docker y Docker Compose
- Captura del docker-compose.yml base
- Descripción de problemas encontrados y decisiones técnicas tomadas
- Definición y justificación del problema computacional distribuido a implementar

### Entregable 2 — Entrega Final

**Fecha de entrega:** 5 de marzo

**Contenido:**
- Diagrama final de la topología VPN
- Evidencia de conectividad completa entre los cuatro nodos
- Evidencia de contenedores Docker levantados en todos los nodos
- Pruebas de comunicación entre contenedores de distintos nodos
- Evidencia del planteamiento e implementación base del algoritmo distribuido en Rust

## 11. Evidencias Técnicas Mínimas

Las siguientes evidencias deben ser documentadas:

- Salida de `wg show` (al menos en el nodo hub)
- Salida de `ip a` mostrando la interfaz wg0
- Ping entre todos los nodos
- Prueba completa de iperf3
- `docker ps` en cada nodo
- Dockerfile(s) del worker
- docker-compose.yml base
- Evidencia del prototipo distribuido en Rust (entrega final)

## 12. Cierre y Riesgos

Principales riesgos técnicos identificados:

- **Configuración de red**: Problemas de conectividad entre nodos debido a configuración incorrecta de firewall o rutas
- **Latencia**: Retardo en la comunicación que puede afectar el rendimiento del algoritmo distribuido
- **Firewall**: Bloqueo accidental de puertos necesarios para la comunicación VPN
- **Asignación de tareas**: Dificultad para balancear la carga de trabajo entre nodos
- **Empaquetado del binario en Rust**: Compatibilidad del binario entre diferentes sistemas

## 13. Criterios de Evaluación

Se evaluará que:
- La VPN funcione correctamente entre todos los nodos
- El entorno de contenedores esté listo para ejecutar una aplicación distribuida
- Cada nodo tenga al menos 4 contenedores workers operativos
- Exista comunicación verificable entre contenedores de nodos distintos
- La evidencia presentada sea clara, coherente y verificable
- El código en Rust compile y ejecute correctamente en el entorno distribuido

## 14. Conexión al Repositorio

```bash
git clone https://github.com/yaelhernandez5630-sys/los-dockerinos.git
cd los-dockerinos
```

Para contribuir, cada integrante debe trabajar en su rama asignada y realizar pull requests a main para integrar cambios.
