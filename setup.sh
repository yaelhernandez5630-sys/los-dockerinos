#!/bin/bash

# 🚀 Script de Configuración para Los Dockerinos
# Uso: ./setup.sh [tu-ip] [tu-nombre]

set -e

IP=$1
NOMBRE=$2

if [ -z "$IP" ] || [ -z "$NOMBRE" ]; then
    echo "❌ Uso: $0 [tu-ip] [tu-nombre]"
    echo "📝 Ejemplo: $0 10.10.10.2 rogelio"
    exit 1
fi

echo "🔧 Configurando para $NOMBRE en IP $IP"

# 1. Configurar Docker Compose
echo "📦 Configurando Docker Compose..."
sed "s/10.10.10.X/10.10.10.$IP/g; s/worker-tu-nombre/worker-$NOMBRE/g" \
    DOCKER/docker-compose.yml.template > DOCKER/docker-compose.yml

echo "✅ Docker Compose configurado"

# 2. Generar claves WireGuard (si no existen)
if [ ! -f "VPN/privatekey" ]; then
    echo "🔐 Generando claves WireGuard..."
    wg genkey | tee VPN/privatekey | wg pubkey > VPN/publickey
    echo "✅ Claves generadas"
    echo "📤 Tu clave pública: $(cat VPN/publickey)"
    echo "🔐 Tu clave privada: $(cat VPN/privatekey)"
fi

# 3. Configurar WireGuard
echo "🌐 Configurando WireGuard..."
sed "s/TU_CLAVE_PRIVADA_AQUI/$(cat VPN/privatekey)/g; s/10.10.10.X/10.10.10.$IP/g" \
    VPN/wg0.conf.template > VPN/wg0.conf

echo "✅ WireGuard configurado"

# 4. Instrucciones finales
echo ""
echo "🎯 Configuración completada!"
echo ""
echo "📋 Siguientes pasos:"
echo "1. 📤 Comparte tu clave pública con tus compañeros:"
echo "   $(cat VPN/publickey)"
echo ""
echo "2. 📡 Agrega las claves públicas de tus compañeros a VPN/wg0.conf"
echo ""
echo "3. 🚀 Inicia WireGuard:"
echo "   sudo cp VPN/wg0.conf /etc/wireguard/wg0.conf"
echo "   sudo wg-quick up wg0"
echo ""
echo "4. 🐳 Inicia los workers:"
echo "   docker-compose -f DOCKER/docker-compose.yml up -d"
echo ""
echo "5. 🦀 Inicia el coordinator:"
echo "   cd rust && ./target/release/mandelbrot_distribuido coordinator"
echo ""
echo "🎉 ¡Listo para procesar Mandelbrot!"
