resource "google_secret_manager_secret_iam_member" "db_accessor" {
  for_each = toset(var.db_secret_accessor_members)

  secret_id = google_secret_manager_secret.db.id
  role      = "roles/secretmanager.secretAccessor"
  member    = each.value
}
