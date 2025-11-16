-- Migration: 20251007230300_create_initial_tables.sql
CREATE TABLE IF NOT EXISTS trip (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    trip_date TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
);
CREATE TABLE IF NOT EXISTS avalanche_forecast (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trip_id INTEGER NOT NULL UNIQUE,
    forecast_alp TEXT,
    forecast_tl TEXT,
    forecast_btl TEXT,
    macro_trends TEXT,
    confidence TEXT,
    comment TEXT,
    FOREIGN KEY (trip_id) REFERENCES trip(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS avalanche_problem (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    forecast_id INTEGER NOT NULL,
    elevation TEXT NOT NULL,
    problem_type TEXT NOT NULL,
    FOREIGN KEY (forecast_id) REFERENCES avalanche_forecast(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS trip_planning (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trip_id INTEGER NOT NULL UNIQUE,
    areas_to_avoid TEXT,
    plan_left_with_someone INTEGER NOT NULL DEFAULT 0,
    decision_points_considered INTEGER NOT NULL DEFAULT 0,
    decision_points_comment TEXT,
    FOREIGN KEY (trip_id) REFERENCES trip(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS weather (
    trip_id INTEGER NOT NULL,
    -- ISO 8601 format: 'YYYY-MM-DDTHH:MM:SSZ'
    observation_time TEXT,
    precipitation TEXT,
    accumulation TEXT,
    wind_speed TEXT,
    wind_direction TEXT,
    solar_radiation TEXT,
    comment TEXT,
    PRIMARY KEY (trip_id, observation_time),
    FOREIGN KEY (trip_id) REFERENCES trip(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS avy_observation (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    trip_id INTEGER NOT NULL,
    -- ISO 8601 format: 'YYYY-MM-DDTHH:MM:SSZ'
    observation_time TEXT NOT NULL,
    -- Avalanche Activity
    avy_activity_size TEXT,
    avy_activity_trigger TEXT,
    avy_activity_characteristic TEXT,
    -- Signs of Instability
    instability_see_feel TEXT,
    instability_ct TEXT,
    instability_ect TEXT,
    comment TEXT,
    FOREIGN KEY (trip_id) REFERENCES trip(id) ON DELETE CASCADE
);
CREATE TRIGGER IF NOT EXISTS trigger_trip_updated_at
AFTER
UPDATE ON trip FOR EACH ROW BEGIN
UPDATE trip
SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
WHERE id = OLD.id;
END;
