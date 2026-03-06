# Roadmap Kubernetes - Los Dockerinos

## Fase 1: Algoritmo con Diseño Kubernetes-Ready

### Service Discovery Strategy
```rust
// No hacer esto (hardcoded):
let workers = vec!["10.10.10.1:8081", "10.10.10.2:8081"];

// Sí hacer esto (dinámico):
let workers = get_workers_from_env().await; // Env vars o API
```

### Environment Variables
```bash
WORKER_ENDPOINTS=10.10.10.1:8081,10.10.10.1:8082,10.10.10.2:8081
COORDINATOR_ENDPOINT=10.10.10.1:8080
WORKER_ID=worker-rogelio-1
```

### Health Checks
```rust
// Endpoint /status para Kubernetes liveness probe
GET /status
Response: {"status": "healthy", "worker_id": "worker-rogelio-1"}
```

## Fase 2: Migración a Kubernetes

### Kubernetes Objects
```yaml
# Deployment para workers
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mandelbrot-workers
spec:
  replicas: 16  # 4 por nodo × 4 nodos
  selector:
    matchLabels:
      app: mandelbrot-worker
  template:
    spec:
      containers:
      - name: worker
        image: los-dockerinos/worker:latest
        env:
        - name: COORDINATOR_ENDPOINT
          value: "mandelbrot-coordinator:8080"
```

### Service Discovery Automático
```yaml
# Service para coordinator
apiVersion: v1
kind: Service
metadata:
  name: mandelbrot-coordinator
spec:
  selector:
    app: mandelbrot-coordinator
  ports:
  - port: 8080
```

## Ventajas de este Enfoque

1. **Sin reingeniería:** El mismo código Rust funciona en ambos
2. **Migración gradual:** Docker Compose → Kubernetes
3. **Testing fácil:** Probar en Docker antes de K8s
4. **Flexibilidad:** Ambos entornos soportados

## Tiempo Estimado
- **Fase 1 (Algoritmo):** 3-4 días
- **Fase 2 (Kubernetes):** 2-3 días (opcional)

## Recomendación
Implementar el algoritmo con diseño desacoplado desde el inicio.
