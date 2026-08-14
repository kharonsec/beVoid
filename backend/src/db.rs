use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use rusqlite::Connection;
use std::sync::Mutex;

pub struct EmotionRow {
    pub uuid: String,
    pub emotion: String,
    pub freq_hz: f64,
    pub css_color: String,
    pub color_name: String,
    pub vibes: f64,
    pub ts_ms: i64,
    pub srgb_fallback: String,
}

pub struct Db {
    conn: Mutex<Connection>,
}

const SCHEMA_TEMPLATE: &str = "
ATTACH DATABASE '{path}' AS soul;
CREATE TABLE IF NOT EXISTS soul.emotions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts_ms INTEGER NOT NULL,
    uuid TEXT NOT NULL,
    emotion TEXT NOT NULL,
    freq_hz REAL NOT NULL,
    css_color TEXT NOT NULL,
    color_name TEXT NOT NULL,
    vibes REAL NOT NULL,
    srgb_fallback TEXT NOT NULL,
    created_at INTEGER NOT NULL DEFAULT (unixepoch())
);
CREATE TRIGGER IF NOT EXISTS soul.no_update
BEFORE UPDATE ON soul.emotions
BEGIN
    SELECT RAISE(ABORT, 'beVoid is append-only: updates vanish into the void');
END;
CREATE TRIGGER IF NOT EXISTS soul.no_delete
BEFORE DELETE ON soul.emotions
BEGIN
    SELECT RAISE(ABORT, 'beVoid is append-only: deletes vanish into the void');
END;
CREATE TEMP VIEW IF NOT EXISTS emotions AS SELECT * FROM soul.emotions;
";

pub fn open(path: &str) -> rusqlite::Result<Db> {
    let escaped = path.replace('\'', "''");
    let schema = SCHEMA_TEMPLATE.replace("{path}", &escaped);
    let conn = Connection::open_in_memory()?;
    conn.execute_batch(&schema)?;
    Ok(Db {
        conn: Mutex::new(conn),
    })
}

impl Db {
    pub fn insert(&self, row: &EmotionRow) -> rusqlite::Result<()> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        conn.execute(
            "INSERT INTO soul.emotions
             (ts_ms, uuid, emotion, freq_hz, css_color, color_name, vibes, srgb_fallback)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                row.ts_ms,
                row.uuid,
                row.emotion,
                row.freq_hz,
                row.css_color,
                row.color_name,
                row.vibes,
                row.srgb_fallback,
            ],
        )?;
        Ok(())
    }

    pub fn list(&self) -> rusqlite::Result<Vec<EmotionRow>> {
        let conn = self.conn.lock().expect("db mutex poisoned");
        let mut stmt = conn.prepare(
            "SELECT uuid, emotion, freq_hz, css_color, color_name, vibes, ts_ms, srgb_fallback
             FROM temp.emotions ORDER BY id",
        )?;
        let rows = stmt
            .query_map([], |r| {
                Ok(EmotionRow {
                    uuid: r.get(0)?,
                    emotion: r.get(1)?,
                    freq_hz: r.get(2)?,
                    css_color: r.get(3)?,
                    color_name: r.get(4)?,
                    vibes: r.get(5)?,
                    ts_ms: r.get(6)?,
                    srgb_fallback: r.get(7)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }
}

pub fn reverse_hash_emotion(emotion: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    emotion.hash(&mut hasher);
    hasher.finish() & 0x0000_FFFF_FFFF_FFFF
}
