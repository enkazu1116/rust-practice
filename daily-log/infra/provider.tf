terraform {
  required_version = "1.15.8"
  required_providers {
    google = {
        source = "hashicorp/google"
        version = "7.3.0"
    }
  }
  backend "gcs" {
    bucket = "simple-compute-storage"
  }
}