# 請求先アカウント
variable "billing_account_id" {
  type        = string
  description = "GCP請求先アカウントID"
}

variable "project_id" {
  type        = string
  description = "Google Cloud Platform Project ID"
}

variable "region" {
  type        = string
  description = "デプロイ先リージョン"
  default     = "asia-northeast1"
}

variable "db_instance_name" {
  type        = string
  description = "Cloud SQL インスタンス名"
  default     = "simple-postgres"
}

variable "db_name" {
  type        = string
  description = "Cloud SQL データベース名"
  default     = "simple-compute"
}

variable "db_user" {
  type        = string
  description = "Cloud SQL ユーザー名"
  default     = "admin_user"
}

variable "db_schema" {
  type        = string
  description = "PostgreSQL スキーマ名。Secret Manager に保存し、アプリとマイグレーションが参照する"
  default     = "simple-compute"

  validation {
    condition     = can(regex("^[A-Za-z0-9_-]+$", var.db_schema))
    error_message = "db_schema は英数字、アンダースコア、ハイフンだけにしてください。"
  }
}

variable "db_secret_id" {
  type        = string
  description = "DB 接続情報を入れる Secret Manager のシークレット ID"
  default     = "daily-log-db"
}

variable "db_secret_accessor_members" {
  type        = list(string)
  description = "DB シークレットを読める IAM メンバー。例: user:name@example.com"
  default     = []
}
