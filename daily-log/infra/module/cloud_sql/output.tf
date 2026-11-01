output "connection" {
  description = "Secret Manager に保存する DB 接続情報"
  sensitive   = true
  value = {
    username                 = google_sql_user.default_user.name
    password                 = random_password.db.result
    database                 = google_sql_database.default_db.name
    schema                   = var.db_schema
    instance_connection_name = google_sql_database_instance.simple_instance.connection_name
    port                     = 5432
  }
}

output "instance_connection_name" {
  description = "Cloud SQL の接続名 (project:region:instance)"
  value       = google_sql_database_instance.simple_instance.connection_name
}
