-- Change ID columns from UUID to TEXT to match ChartDB's custom ID format

ALTER TABLE diagrams ALTER COLUMN id TYPE TEXT;
ALTER TABLE db_tables ALTER COLUMN id TYPE TEXT;
ALTER TABLE db_tables ALTER COLUMN diagram_id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN diagram_id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN source_table_id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN target_table_id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN source_field_id TYPE TEXT;
ALTER TABLE db_relationships ALTER COLUMN target_field_id TYPE TEXT;
ALTER TABLE db_dependencies ALTER COLUMN id TYPE TEXT;
ALTER TABLE db_dependencies ALTER COLUMN diagram_id TYPE TEXT;
ALTER TABLE db_dependencies ALTER COLUMN table_id TYPE TEXT;
ALTER TABLE db_dependencies ALTER COLUMN dependent_table_id TYPE TEXT;
ALTER TABLE areas ALTER COLUMN id TYPE TEXT;
ALTER TABLE areas ALTER COLUMN diagram_id TYPE TEXT;
ALTER TABLE db_custom_types ALTER COLUMN id TYPE TEXT;
ALTER TABLE db_custom_types ALTER COLUMN diagram_id TYPE TEXT;
ALTER TABLE notes ALTER COLUMN id TYPE TEXT;
ALTER TABLE notes ALTER COLUMN diagram_id TYPE TEXT;

