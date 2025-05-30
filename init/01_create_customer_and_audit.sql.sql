-- Create source table for auditing
CREATE TABLE IF NOT EXISTS customer
(
    id             SERIAL PRIMARY KEY,
    name           TEXT           DEFAULT 'Unnamed',
    email          TEXT           DEFAULT 'unknown@example.com',
    is_active      BOOLEAN        DEFAULT true,
    signup_date    DATE           DEFAULT CURRENT_DATE,
    last_login     TIMESTAMPTZ    DEFAULT CURRENT_TIMESTAMP,
    loyalty_points INTEGER        DEFAULT 0,
    balance        NUMERIC(10, 2) DEFAULT 0.00,
    metadata       JSONB          DEFAULT '{}'::jsonb
);

-- Create audit table with schema/table info and full change data
CREATE TABLE IF NOT EXISTS audit_customer
(
    change_id   SERIAL PRIMARY KEY,
    schema_name TEXT NOT NULL,
    table_name  TEXT NOT NULL,
    customer_id INT,
    change_type TEXT CHECK (change_type IN ('INSERT', 'UPDATE', 'DELETE')),
    old_data    JSONB,
    new_data    JSONB,
    changed_at  TIMESTAMPTZ DEFAULT now()
);

-- Drop and recreate audit trigger function
DROP FUNCTION IF EXISTS log_customer_changes() CASCADE;

CREATE FUNCTION log_customer_changes()
    RETURNS TRIGGER AS
$$
BEGIN
    IF TG_OP = 'INSERT' THEN
        INSERT INTO audit_customer(schema_name, table_name, customer_id, change_type, old_data, new_data)
        VALUES (TG_TABLE_SCHEMA, TG_TABLE_NAME, NEW.id, 'INSERT', NULL, row_to_json(NEW));
        RETURN NEW;

    ELSIF TG_OP = 'UPDATE' THEN
        INSERT INTO audit_customer(schema_name, table_name, customer_id, change_type, old_data, new_data)
        VALUES (TG_TABLE_SCHEMA, TG_TABLE_NAME, NEW.id, 'UPDATE', row_to_json(OLD), row_to_json(NEW));
        RETURN NEW;

    ELSIF TG_OP = 'DELETE' THEN
        INSERT INTO audit_customer(schema_name, table_name, customer_id, change_type, old_data, new_data)
        VALUES (TG_TABLE_SCHEMA, TG_TABLE_NAME, OLD.id, 'DELETE', row_to_json(OLD), NULL);
        RETURN OLD;
    END IF;

    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Drop and recreate trigger
DROP TRIGGER IF EXISTS trg_customer_audit ON customer;

CREATE TRIGGER trg_customer_audit
    AFTER INSERT OR UPDATE OR DELETE
    ON customer
    FOR EACH ROW
EXECUTE FUNCTION log_customer_changes();
