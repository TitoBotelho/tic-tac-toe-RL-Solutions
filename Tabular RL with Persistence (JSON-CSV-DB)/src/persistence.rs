use std::collections::HashMap;
use std::fs;
use std::path::Path;

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::board::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateValue {
    pub state: State,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedVTable {
    pub entries: Vec<StateValue>,
}

pub fn export_json(path: &str, v: &HashMap<State, f64>) -> Result<(), String> {
    let mut entries: Vec<StateValue> = v
        .iter()
        .map(|(state, value)| StateValue {
            state: *state,
            value: *value,
        })
        .collect();

    entries.sort_by(|a, b| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal));

    let payload = PersistedVTable { entries };
    let text = serde_json::to_string_pretty(&payload).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn import_json(path: &str) -> Result<HashMap<State, f64>, String> {
    if !Path::new(path).exists() {
        return Ok(HashMap::new());
    }

    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let payload: PersistedVTable = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    let mut table = HashMap::with_capacity(payload.entries.len());
    for entry in payload.entries {
        table.insert(entry.state, entry.value);
    }

    Ok(table)
}

pub fn import_csv(path: &str) -> Result<HashMap<State, f64>, String> {
    if !Path::new(path).exists() {
        return Ok(HashMap::new());
    }

    let mut reader = csv::Reader::from_path(path).map_err(|e| e.to_string())?;
    let mut table = HashMap::new();

    for result in reader.records() {
        let record = result.map_err(|e| e.to_string())?;
        if record.len() < 10 {
            continue;
        }

        let state = [
            record.get(0).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(1).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(2).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(3).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(4).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(5).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(6).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(7).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
            record.get(8).unwrap_or("0").parse::<u8>().map_err(|e| e.to_string())?,
        ];
        let value = record
            .get(9)
            .unwrap_or("0")
            .parse::<f64>()
            .map_err(|e| e.to_string())?;

        table.insert(state, value);
    }

    Ok(table)
}

pub fn import_sqlite(path: &str) -> Result<HashMap<State, f64>, String> {
    if !Path::new(path).exists() {
        return Ok(HashMap::new());
    }

    let conn = Connection::open(path).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT s0, s1, s2, s3, s4, s5, s6, s7, s8, value FROM state_values",
        )
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([], |row| {
            let state = [
                row.get::<_, i64>(0)? as u8,
                row.get::<_, i64>(1)? as u8,
                row.get::<_, i64>(2)? as u8,
                row.get::<_, i64>(3)? as u8,
                row.get::<_, i64>(4)? as u8,
                row.get::<_, i64>(5)? as u8,
                row.get::<_, i64>(6)? as u8,
                row.get::<_, i64>(7)? as u8,
                row.get::<_, i64>(8)? as u8,
            ];
            let value = row.get::<_, f64>(9)?;
            Ok((state, value))
        })
        .map_err(|e| e.to_string())?;

    let mut table = HashMap::new();
    for row in rows {
        let (state, value) = row.map_err(|e| e.to_string())?;
        table.insert(state, value);
    }

    Ok(table)
}

pub fn export_csv(path: &str, v: &HashMap<State, f64>) -> Result<(), String> {
    let mut writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;

    writer
        .write_record(["s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "value"])
        .map_err(|e| e.to_string())?;

    let mut entries: Vec<_> = v.iter().collect();
    entries.sort_by_key(|(state, _)| **state);

    for (state, value) in entries {
        writer
            .write_record([
                state[0].to_string(),
                state[1].to_string(),
                state[2].to_string(),
                state[3].to_string(),
                state[4].to_string(),
                state[5].to_string(),
                state[6].to_string(),
                state[7].to_string(),
                state[8].to_string(),
                value.to_string(),
            ])
            .map_err(|e| e.to_string())?;
    }

    writer.flush().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn export_sqlite(path: &str, v: &HashMap<State, f64>) -> Result<(), String> {
    let mut conn = Connection::open(path).map_err(|e| e.to_string())?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS state_values (
            s0 INTEGER NOT NULL,
            s1 INTEGER NOT NULL,
            s2 INTEGER NOT NULL,
            s3 INTEGER NOT NULL,
            s4 INTEGER NOT NULL,
            s5 INTEGER NOT NULL,
            s6 INTEGER NOT NULL,
            s7 INTEGER NOT NULL,
            s8 INTEGER NOT NULL,
            value REAL NOT NULL,
            PRIMARY KEY (s0, s1, s2, s3, s4, s5, s6, s7, s8)
        );
        ",
    )
    .map_err(|e| e.to_string())?;

    let tx = conn.transaction().map_err(|e| e.to_string())?;

    {
        let mut stmt = tx
            .prepare(
                "
                INSERT INTO state_values (s0, s1, s2, s3, s4, s5, s6, s7, s8, value)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                ON CONFLICT(s0, s1, s2, s3, s4, s5, s6, s7, s8)
                DO UPDATE SET value = excluded.value
                ",
            )
            .map_err(|e| e.to_string())?;

        for (state, value) in v {
            stmt.execute(params![
                state[0] as i64,
                state[1] as i64,
                state[2] as i64,
                state[3] as i64,
                state[4] as i64,
                state[5] as i64,
                state[6] as i64,
                state[7] as i64,
                state[8] as i64,
                *value,
            ])
            .map_err(|e| e.to_string())?;
        }
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_table_from_file(path: &str) -> Result<HashMap<State, f64>, String> {
    if !Path::new(path).exists() {
        return Ok(HashMap::new());
    }

    let ext = Path::new(path)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match ext.to_ascii_lowercase().as_str() {
        "json" => import_json(path),
        "csv" => import_csv(path),
        "db" | "sqlite" => import_sqlite(path),
        _ => Err(format!("Formato de arquivo não suportado: {}", path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn import_json_table_from_disk() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = format!("/tmp/v_table_test_{}.json", unique);

        let mut expected = HashMap::new();
        expected.insert([1, 0, 2, 0, 1, 0, 0, 2, 0], 0.75);
        expected.insert([0, 1, 2, 0, 0, 0, 1, 0, 2], 0.25);

        export_json(&path, &expected).unwrap();

        let loaded = load_table_from_file(&path).unwrap();
        assert_eq!(loaded.len(), expected.len());
        assert_eq!(loaded.get(&[1, 0, 2, 0, 1, 0, 0, 2, 0]), Some(&0.75));
        assert_eq!(loaded.get(&[0, 1, 2, 0, 0, 0, 1, 0, 2]), Some(&0.25));

        std::fs::remove_file(path).ok();
    }

    #[test]
    fn import_csv_skips_header_and_preserves_values() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = format!("/tmp/v_table_csv_test_{}.csv", unique);

        let mut expected = HashMap::new();
        expected.insert([1, 0, 0, 0, 1, 0, 0, 0, 2], 0.82);
        expected.insert([0, 1, 2, 0, 0, 0, 1, 0, 2], 0.42);

        export_csv(&path, &expected).unwrap();

        let loaded = import_csv(&path).unwrap();
        assert_eq!(loaded.len(), expected.len());
        assert_eq!(loaded.get(&[1, 0, 0, 0, 1, 0, 0, 0, 2]), Some(&0.82));
        assert_eq!(loaded.get(&[0, 1, 2, 0, 0, 0, 1, 0, 2]), Some(&0.42));

        std::fs::remove_file(path).ok();
    }
}
