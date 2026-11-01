variable "project_id" {
  type        = string
  description = "Google Cloud Platform Project ID"
}

variable "instance_connection_name" {
  type        = string
  description = "Cloud SQL の接続名 (project:region:instance)"
}

variable "name" {
  type        = string
  description = "Compute Engine インスタンス名"
  default     = "simple-compute"
}

variable "machine_type" {
  type        = string
  description = "Compute Engine のマシンタイプ"
  default     = "e2-micro"
}

variable "zone" {
  type        = string
  description = "Compute Engine のゾーン"
  default     = "asia-northeast1-b"
}

variable "service_account_id" {
  type        = string
  description = "VM に付けるサービスアカウント ID"
  default     = "simple-compute-vm"
}

variable "image_family" {
  type        = string
  description = "Packer で作成した Compute Engine イメージファミリー"
  default     = "simple-compute"
}
