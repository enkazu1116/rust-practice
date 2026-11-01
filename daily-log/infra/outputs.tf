output "db_secret_id" {
  description = "DB 接続情報の Secret Manager シークレット ID"
  value       = google_secret_manager_secret.db.secret_id
}

output "db_instance_connection_name" {
  description = "Cloud SQL の接続名 (project:region:instance)"
  value       = google_sql_database_instance.simple_instance.connection_name
}
