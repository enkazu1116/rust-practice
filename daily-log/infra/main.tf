resource "google_sql_database_instance" "simple_instance" {
  name = "simple-postgres"
  database_version = "POSTGRES_18"
  region = "asia-northeast1"

  settings {
    tier = "db-f1-micro"
    edition = "ENTERPRISE"
  }

  depends_on = [google_project_service.enable_apis]
}

resource "google_sql_database" "default_db" {
  name = "default_postgres"
  instance = google_sql_database_instance.simple_instance.name
}

resource "google_sql_user" "default_user" {
  name = "admin_user"
  password = var.db_password
  instance = google_sql_database_instance.simple_instance.name
}