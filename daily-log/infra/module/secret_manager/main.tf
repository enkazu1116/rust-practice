resource "google_secret_manager_secret" "db" {
  secret_id = var.db_secret_id

  replication {
    auto {}
  }
}

resource "google_secret_manager_secret_version" "db" {
  secret      = google_secret_manager_secret.db.id
  secret_data = jsonencode(var.db_connection)
}

resource "google_secret_manager_secret_iam_member" "db_accessor" {
  for_each = toset(var.db_secret_accessor_members)

  secret_id = google_secret_manager_secret.db.id
  role      = "roles/secretmanager.secretAccessor"
  member    = each.value
}
