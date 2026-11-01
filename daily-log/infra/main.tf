resource "random_password" "db" {
  length  = 32
  special = true
}

resource "google_sql_database_instance" "simple_instance" {
  name             = var.db_instance_name
  database_version = "POSTGRES_18"
  region           = var.region

  settings {
    tier    = "db-f1-micro"
    edition = "ENTERPRISE"
  }

  depends_on = [google_project_service.enable_apis]
}

resource "google_sql_database" "default_db" {
  name     = var.db_name
  instance = google_sql_database_instance.simple_instance.name
}

resource "google_sql_user" "default_user" {
  name     = var.db_user
  password = random_password.db.result
  instance = google_sql_database_instance.simple_instance.name
}
