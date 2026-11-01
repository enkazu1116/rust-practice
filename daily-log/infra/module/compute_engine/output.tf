output "instance_name" {
  description = "Compute Engine インスタンス名"
  value       = google_compute_instance.vm.name
}

output "zone" {
  description = "Compute Engine のゾーン"
  value       = google_compute_instance.vm.zone
}

output "service_account_email" {
  description = "VM のサービスアカウント"
  value       = google_service_account.vm.email
}
