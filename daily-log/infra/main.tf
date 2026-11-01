module "cloud_sql" {
  source = "./module/cloud_sql"

  depends_on = [google_project_service.enable_apis]
}

module "secret_manager" {
  source = "./module/secret_manager"

  db_connection = module.cloud_sql.connection

  depends_on = [google_project_service.enable_apis]
}

moved {
  from = random_password.db
  to   = module.cloud_sql.random_password.db
}

moved {
  from = google_sql_database_instance.simple_instance
  to   = module.cloud_sql.google_sql_database_instance.simple_instance
}

moved {
  from = google_sql_database.default_db
  to   = module.cloud_sql.google_sql_database.default_db
}

moved {
  from = google_sql_user.default_user
  to   = module.cloud_sql.google_sql_user.default_user
}

moved {
  from = google_secret_manager_secret.db
  to   = module.secret_manager.google_secret_manager_secret.db
}

moved {
  from = google_secret_manager_secret_version.db
  to   = module.secret_manager.google_secret_manager_secret_version.db
}

moved {
  from = google_secret_manager_secret_iam_member.db_accessor
  to   = module.secret_manager.google_secret_manager_secret_iam_member.db_accessor
}
