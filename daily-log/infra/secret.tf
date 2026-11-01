resource "google_secret_manager_secret" "db" {
  secret_id = var.db_secret_id

  replication {
    auto {}
  }

  depends_on = [google_project_service.enable_apis]
}

resource "google_secret_manager_secret_version" "db" {
  secret = google_secret_manager_secret.db.id
  secret_data = jsonencode({
    username                 = var.db_user
    password                 = random_password.db.result
    database                 = var.db_name
    schema                   = var.db_schema
    instance_connection_name = google_sql_database_instance.simple_instance.connection_name
    port                     = 5432
  })
}
