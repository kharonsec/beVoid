mod auth;
mod color;
mod db;
mod hum;
mod wav;

pub mod bevoid {
    tonic::include_proto!("bevoid");
}

use std::sync::Arc;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use bevoid::void_service_server::{VoidService, VoidServiceServer};
use bevoid::{
    AuthenticateRequest, AuthenticateResponse, EmotionPoint, HumRequest, HumResponse,
    ListEmotionsRequest, ListEmotionsResponse,
};
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::db::{reverse_hash_emotion, Db};

#[derive(Clone)]
struct Void {
    db: Arc<Db>,
    auth: Arc<auth::Auth>,
    last_emotions: Arc<RwLock<Vec<EmotionPoint>>>,
}

#[tonic::async_trait]
impl VoidService for Void {
    async fn hum(&self, request: Request<HumRequest>) -> Result<Response<HumResponse>, Status> {
        let req = request.into_inner();
        let bytes = BASE64
            .decode(req.audio_base64)
            .map_err(|e| Status::invalid_argument(format!("audio is not base64: {e}")))?;

        let wav = wav::parse(&bytes)
            .map_err(|e| Status::invalid_argument(format!("not a valid WAV: {e}")))?;
        let reading = hum::analyze(&wav.samples, wav.sample_rate).map_err(Status::internal)?;

        let hashed_ts_ms = reverse_hash_emotion(&reading.emotion);
        let timestamp = uuid::Timestamp::from_unix(
            uuid::NoContext,
            hashed_ts_ms / 1000,
            (hashed_ts_ms % 1000) as u32 * 1_000_000,
        );
        let id = Uuid::new_v7(timestamp);

        let row = db::EmotionRow {
            uuid: id.to_string(),
            emotion: reading.emotion.clone(),
            freq_hz: reading.freq_hz as f64,
            css_color: reading.color.css.clone(),
            color_name: reading.color.name.clone(),
            vibes: reading.vibes as f64,
            ts_ms: hashed_ts_ms as i64,
            srgb_fallback: reading.color.srgb_fallback.clone(),
        };
        self.db
            .insert(&row)
            .map_err(|e| Status::internal(format!("the void refused the record: {e}")))?;

        let response = HumResponse {
            uuid: row.uuid,
            emotion: row.emotion,
            freq_hz: row.freq_hz,
            css_color: row.css_color,
            color_name: row.color_name,
            color_exists_in_srgb: false,
            vibes: row.vibes,
            ts_ms: row.ts_ms,
            srgb_fallback: row.srgb_fallback,
        };
        self.refresh_cache().await;
        Ok(Response::new(response))
    }

    async fn list_emotions(
        &self,
        _request: Request<ListEmotionsRequest>,
    ) -> Result<Response<ListEmotionsResponse>, Status> {
        let points = self.last_emotions.read().await.clone();
        Ok(Response::new(ListEmotionsResponse { points }))
    }

    async fn authenticate(
        &self,
        request: Request<AuthenticateRequest>,
    ) -> Result<Response<AuthenticateResponse>, Status> {
        let req = request.into_inner();
        let ok = self.auth.verify(&req.x, &req.y);
        let token = if ok {
            Uuid::new_v4().to_string()
        } else {
            String::new()
        };
        Ok(Response::new(AuthenticateResponse { ok, token }))
    }
}

impl Void {
    async fn refresh_cache(&self) {
        let rows = match self.db.list() {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("cache refresh failed: {e}");
                return;
            }
        };
        let points = rows
            .into_iter()
            .map(|r| EmotionPoint {
                uuid: r.uuid,
                emotion: r.emotion,
                freq_hz: r.freq_hz,
                css_color: r.css_color,
                color_name: r.color_name,
                color_exists_in_srgb: false,
                vibes: r.vibes,
                ts_ms: r.ts_ms,
                srgb_fallback: r.srgb_fallback,
            })
            .collect();
        *self.last_emotions.write().await = points;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = std::env::var("BEVOID_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".into());
    let db_path = std::env::var("BEVOID_DB").unwrap_or_else(|_| "data/bevoid.db".into());
    let auth_path = std::env::var("BEVOID_AUTH").unwrap_or_else(|_| "data/auth.toml".into());

    let db = Arc::new(db::open(&db_path)?);
    let auth = Arc::new(auth::Auth::load(&auth_path)?);

    let svc = Void {
        db,
        auth: auth.clone(),
        last_emotions: Arc::new(RwLock::new(Vec::new())),
    };
    svc.refresh_cache().await;

    println!("beVoid listening on {addr}");
    println!("sigil required for entry: {}", auth.sigil_name);

    tonic::transport::Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(VoidServiceServer::new(svc)))
        .serve(addr.parse()?)
        .await?;

    Ok(())
}
