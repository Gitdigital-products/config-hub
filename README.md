# config-hub
config-hub Centralized configuration server for Gitdigital apps.
# Config Hub

Centralized configuration manager for the **Gitdigital Products** ecosystem.  
Keeps all service configs in one place so nothing is hardcoded.

## 🚀 Features
- `POST /set` → Store or update config values.
- `GET /get/:key` → Retrieve config values.
- In-memory store with DashMap.
- Future-ready for database + secret vault integration.

## 🛠️ Setup
```bash
cargo run
# Config Hub

**Mission:** A centralized configuration server providing a single source of truth for application settings and feature flags across all Gitdigital-products services. It is for DevOps and development teams to ensure consistent deployment and management. *(Stage: Core Infrastructure)*
