variable "region" {
  type        = string
  description = "Cloud SQLのリージョン"
  default     = "asia-northeast1"
}

variable "db_instance_name" {
  type        = string
  description = "Cloud SQLインスタンス名"
  default     = "simple-postgres"
}

variable "db_name" {
  type        = string
  description = "Cloud SQLデータベース名"
  default     = "simple-compute"
}

variable "db_user" {
  type        = string
  description = "Cloud SQLユーザー名"
  default     = "admin_user"
}

variable "db_schema" {
  type        = string
  description = "Secret Managerに保存"
  default     = "simple-compute"

  validation {
    condition     = can(regex("^[A-Za-z0-9_-]+$", var.db_schema))
    error_message = "db_schema は英数字、アンダースコア、ハイフンだけにしてください。"
  }
}
