variable "db_secret_id" {
  type        = string
  description = "DB 接続情報を入れるSecret ManagerのID"
  default     = "simple-compute-db"
}

variable "db_secret_accessor_members" {
  type        = list(string)
  description = "DB シークレットを読める IAM メンバー"
  default     = []
}

variable "db_connection" {
  description = "Cloud SQLモジュールが出力した接続情報"
  sensitive   = true
  type = object({
    username                 = string
    password                 = string
    database                 = string
    schema                   = string
    instance_connection_name = string
    port                     = number
  })
}
