# Diagrama de Red - Los Dockerinos

## Topología Hub-and-Spoke

```
                    INTERNET
                        |
        +---------------+---------------+
        |                               |
  IP Pública:                    IP Pública:
  [TU VM HUB]                    [OTRAS VMs]
  10.10.10.1                     10.10.10.2-4
        |                               |
        +-----------+-----------+-------+
                    |
              WireGuard VPN
              UDP 51820
                    |
        +-----------+-----------+-------+
        |           |           |       |
   10.10.10.1  10.10.10.2  10.10.10.3 10.10.10.4
     (HUB)      (PEER1)     (PEER2)    (PEER3)
        |           |           |       |
    +---+---+   +---+---+   +---+---+  |
    |Docker |   |Docker |   |Docker |  |
    |Workers|   |Workers|   |Workers|  |
    +-------+   +-------+   +-------+  |
        |           |           |       |
        +-----------+-----------+-------+
                    |
              Comunicación
              Mandelbrot
              Distribuido
```

## Flujo de Comunicación

1. **Coordinator** (HUB) distribuye tareas
2. **Workers** (4 por nodo) procesan fragmentos
3. **Resultados** se retornan al coordinator
4. **Todo** viaja por VPN WireGuard

## Puertos Utilizados

- **WireGuard:** UDP 51820 (HUB)
- **Coordinator API:** TCP 8080 (HUB)
- **Workers:** Usan network_mode: host

## IPs Asignadas

| Nodo | IP VPN | Rol | Workers |
|------|--------|-----|---------|
| Rogelio | 10.10.10.1 | HUB/Coordinator | 4 workers |
| Compañero 1 | 10.10.10.2 | PEER | 4 workers |
| Compañero 2 | 10.10.10.3 | PEER | 4 workers |
| Compañero 3 | 10.10.10.4 | PEER | 4 workers |

**Total:** 16 workers distribuidos en 4 nodos
