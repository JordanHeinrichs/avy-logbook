-- Migration: 20251007230300_create_initial_tables.sql
CREATE TABLE IF NOT EXISTS trip (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    trip_date TEXT NOT NULL
);
