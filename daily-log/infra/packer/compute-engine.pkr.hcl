packer {
  required_plugins {
    googlecompute = {
      source  = "github.com/hashicorp/googlecompute"
      version = "~> 1.2.7"
    }
  }
}

variable "project_id" {
  type        = string
  description = "イメージを保存する GCP プロジェクト ID"
}

variable "zone" {
  type        = string
  description = "イメージ作成に使うゾーン"
  default     = "asia-northeast1-b"
}

variable "image_family" {
  type        = string
  description = "作成するイメージファミリー"
  default     = "simple-compute"
}

variable "cloud_sql_proxy_version" {
  type        = string
  description = "Cloud SQL Auth Proxy のバージョン"
  default     = "v2.27.0"
}

variable "cloud_sql_proxy_sha256" {
  type        = string
  description = "Cloud SQL Auth Proxy linux amd64 の SHA-256"
  default     = "39ed2abcaa073c3b4714585e36a904e2b1d92218c5e07c18224fb73ef4bedde5"
}

source "googlecompute" "simple_compute" {
  project_id              = var.project_id
  source_image_family     = "debian-12"
  source_image_project_id = ["debian-cloud"]
  zone                    = var.zone
  machine_type            = "e2-micro"
  ssh_username            = "packer"
  image_name              = "${var.image_family}-{{timestamp}}"
  image_family            = var.image_family
  image_description       = "Debian 12 with Cloud SQL Auth Proxy"
}

build {
  sources = ["sources.googlecompute.simple_compute"]

  provisioner "file" {
    source      = "${path.root}/files/cloud-sql-proxy-run"
    destination = "/tmp/cloud-sql-proxy-run"
  }

  provisioner "file" {
    source      = "${path.root}/files/cloud-sql-proxy.service"
    destination = "/tmp/cloud-sql-proxy.service"
  }

  provisioner "shell" {
    script = "${path.root}/scripts/install-cloud-sql-proxy.sh"
    environment_vars = [
      "PROXY_VERSION=${var.cloud_sql_proxy_version}",
      "PROXY_SHA256=${var.cloud_sql_proxy_sha256}",
    ]
    execute_command = "chmod +x {{ .Path }}; {{ .Vars }} sudo -E {{ .Path }}"
  }
}
