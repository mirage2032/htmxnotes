ALTER SYSTEM SET max_connections = 100;
SELECT pg_reload_conf();