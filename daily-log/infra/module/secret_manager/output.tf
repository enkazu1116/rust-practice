output "secret_id" {
  description = "DB接続情報のSecret ManagerのID"
  value       = google_secret_manager_secret.db.secret_id
}
