🧠 Podsumowanie Architektury
PostgreSQL zawiera główne tabele (np. customer) oraz odpowiadające im tabele audytowe (np. audit_customer)

Dla każdej operacji INSERT, UPDATE, DELETE wyzwalacze (trigger) zapisują zmiany w tabeli audytowej — z polami takimi jak:
schema_name, table_name, customer_id, change_type, old_data, new_data, changed_at

Aplikacja Rustowa (audit_exporter) jest uruchamiana co 24 godziny przez scheduler (np. cron lub Kubernetes Job)

Aplikacja:

Wczytuje konfigurację z pliku config.yaml (m.in. since, max_rows, ścieżka wyjściowa)

Pobiera dane z tabel audytowych, filtrując po changed_at > since

Zapisuje dane jako pliki .json lub .ndjson do odpowiednich ścieżek w Google Cloud Storage (GCS), z podziałem na schemat, nazwę tabeli i datę

