output "db_secret_id" {
  description = "DB接続情報のSecret Manager ID"
  value       = module.secret_manager.secret_id
}

output "db_instance_connection_name" {
  description = "Cloud SQLの接続名ß"
  value       = module.cloud_sql.instance_connection_name
}
