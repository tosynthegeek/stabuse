pub const TRIGGER_FUNCTION_MERCHANTS: &str = r#" 
    CREATE TRIGGER set_updated_at_merchants
    BEFORE UPDATE ON merchants
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
"#;
