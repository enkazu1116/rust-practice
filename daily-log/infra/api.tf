locals {
    apis = [
        "compute.googleapis.com",
        "logging.googleapis.com",
        "monitoring.googleapis.com",
        "iam.googleapis.com",
        "sqladmin.googleapis.com",
    ]
}

# APIを有効化
resource "google_project_service" "enable_apis" {
    for_each = toset(local.apis)

    project = var.project_id
    service = each.value

}