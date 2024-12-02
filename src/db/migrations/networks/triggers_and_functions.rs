pub const TRIGGER: &str = r#"
    CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
    BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
    END;
    $$ LANGUAGE plpgsql
"#;

pub const TRIGGER_FUNCTION: &str = r#" 
    CREATE TRIGGER set_updated_at
    BEFORE UPDATE ON networks
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
"#;
