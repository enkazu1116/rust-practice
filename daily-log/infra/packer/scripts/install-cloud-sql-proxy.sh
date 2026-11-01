#!/bin/bash
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive

apt-get update
apt-get install -y curl ca-certificates

install -d -m 0755 /usr/local/bin
curl -fsSL "https://storage.googleapis.com/cloud-sql-connectors/cloud-sql-proxy/${PROXY_VERSION}/cloud-sql-proxy.linux.amd64" \
  -o /usr/local/bin/cloud-sql-proxy
echo "${PROXY_SHA256}  /usr/local/bin/cloud-sql-proxy" | sha256sum -c -
chmod 0755 /usr/local/bin/cloud-sql-proxy

install -m 0755 /tmp/cloud-sql-proxy-run /usr/local/bin/cloud-sql-proxy-run
install -m 0644 /tmp/cloud-sql-proxy.service /etc/systemd/system/cloud-sql-proxy.service
rm -f /tmp/cloud-sql-proxy-run /tmp/cloud-sql-proxy.service

systemctl daemon-reload
systemctl enable cloud-sql-proxy.service
