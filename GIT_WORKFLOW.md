# 🚀 Git Workflow para Los Dockerinos

## 📋 Estructura del Repositorio

```
los-dockerinos/
├── 📁 rust/                 # Código fuente Rust (compartido)
├── 📁 DOCKER/              # Configuración Docker (templates + personal)
├── 📁 VPN/                 # Configuraciones WireGuard (templates + personales)
├── 📁 docs/                # Documentación compartida
├── 🔧 setup.sh             # Script de configuración
├── 📄 .gitignore           # Reglas de seguridad
└── 📄 README.md            # Este archivo
```

## 🎯 Flujo de Trabajo

### 1. Configuración Inicial (una vez por persona)
```bash
# Clonar el repo
git clone <url-repo>
cd los-dockerinos

# Ejecutar script de configuración
./setup.sh 10.10.10.2 rogelio  # Ajustar IP y nombre
```

### 2. Desarrollo Diario
```bash
# 1. Compilar el proyecto
cd rust && cargo build --release

# 2. Probar localmente
./target/release/mandelbrot_distribuido coordinator

# 3. Commit de cambios
git add .
git commit -m "feat: nueva funcionalidad"
git push origin mi-branch
```

### 3. Despliegue
```bash
# Iniciar workers
docker-compose -f DOCKER/docker-compose.yml up -d

# Iniciar coordinator
cd rust && ./target/release/mandelbrot_distribuido coordinator
```

## 🔐 Reglas de Seguridad

### ❌ NUNCA subir a Git:
- Claves privadas (`*.key`, `privatekey`)
- Configuraciones personales (`*.conf`)
- Archivos compilados (`/rust/target/`)
- Variables de entorno (`.env`)

### ✅ SÍ subir a Git:
- Templates de configuración (`*.template`)
- Código fuente Rust
- Documentación
- Scripts de automatización

## 🌐 Configuración por Nodo

### Hub (10.10.10.1)
```bash
./setup.sh 10.10.10.1 hub
# Ejecuta coordinator + workers
```

### Spoke 1 (10.10.10.2)
```bash
./setup.sh 10.10.10.2 compañero1
# Solo ejecuta workers
```

### Spoke 2 (10.10.10.3)
```bash
./setup.sh 10.10.10.3 compañero2
# Solo ejecuta workers
```

### Spoke 3 (10.10.10.4)
```bash
./setup.sh 10.10.10.4 compañero3
# Solo ejecuta workers
```

## 🔄 Git Branch Strategy

### Branches Principales
- `main` - Producción estable
- `develop` - Desarrollo integrado
- `feature/nombre-funcionalidad` - Features individuales

### Flujo de Trabajo
1. Crear branch: `git checkout -b feature/mi-feature`
2. Desarrollar y commitear
3. Push: `git push origin feature/mi-feature`
4. Pull Request a `develop`
5. Review y merge

## 📝 Compartir Claves Públicas

### Paso 1: Generar y compartir
```bash
# El script muestra tu clave pública:
📤 Tu clave pública: abc123def456...
```

### Paso 2: Agregar peers en VPN/wg0.conf
```ini
[Peer]
PublicKey = abc123def456...  # Clave del compañero
AllowedIPs = 10.10.10.2/32  # IP del compañero
```

## 🚀 Comandos Útiles

### Ver estado VPN
```bash
sudo wg show
sudo ip a show wg0
ping 10.10.10.1
```

### Ver estado Docker
```bash
docker-compose -f DOCKER/docker-compose.yml ps
docker-compose -f DOCKER/docker-compose.yml logs
```

### Probar sistema
```bash
# Health check
curl http://localhost:3000/health

# Obtener tarea
curl http://localhost:3000/get_task

# Ver estado
curl http://localhost:3000/status
```

## 🆘 Troubleshooting

### VPN no conecta
```bash
# Verificar configuración
sudo wg-quick down wg0
sudo wg-quick up wg0
sudo wg show
```

### Workers no se conectan
```bash
# Verificar variables de entorno
docker-compose -f DOCKER/docker-compose.yml logs worker-1

# Verificar red
ping 10.10.10.1
telnet 10.10.10.1 3000
```

### Coordinator no responde
```bash
# Verificar puerto
ss -tlnp | grep 3000

# Verificar logs
cd rust && ./target/release/mandelbrot_distribuido coordinator
```

## 🎯 Mejores Prácticas

1. **Commits descriptivos**: `feat: add health check`, `fix: json serialization`
2. **Branchs cortos**: No dejar branches abiertos mucho tiempo
3. **Documentar cambios**: Actualizar README cuando se modifica la arquitectura
4. **Testing**: Probar localmente antes de commitear
5. **Seguridad**: NUNCA commitear claves privadas

## 📞 Soporte

Si tienes problemas:
1. Revisa este README
2. Revisa los logs de Docker y WireGuard
3. Pide ayuda en el grupo del equipo
4. Crea un issue en el repo con detalles del error
