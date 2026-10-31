// 請求先アカウント
variable "billing_account_id" {
  type = string
  description = "GCP請求先アカウントID"
}

variable "project_id" {
  type = string
  description = "Google Cloud Platform Project ID"
}

variable "db_password" {
  type = string
  description = "Cloud SQL ユーザーのパスワード"
  sensitive = true
}