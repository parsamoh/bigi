use base64::{engine::general_purpose, Engine};
use celes::Country;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tide::Request;
mod db;
use db::db::DB;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/test").get(|_| async { Ok("Hello, world!") });
    app.at("/").get(get_ip_data);
    app.listen("0.0.0.0:8080").await?;
    println!("Server running on port 8080");
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
enum SortBy {
    Ping,
    Speed,
    Random,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IPGetParams {
    country: Option<String>,
    limit: Option<i8>,
    sort: Option<SortBy>,
}

async fn get_ip_data(req: Request<()>) -> tide::Result {
    let db = DB::new().await?;
    let params: IPGetParams = req.query()?;
    println!("{:?}", params);
    let data = db.get(params).await.map(|v| -> Vec<_> {
        v.into_iter()
        .map(|ip_data| {
                let country = Country::from_str(ip_data.cf_country.unwrap().as_str()).unwrap();
                let id = std::env::var("UUID").expect("Expected UUID in env vars");
                let domain = std::env::var("DOMAIN").expect("Expected DOMAIN in env vars");
                format!(
                    "vless://{}@{}:443?encryption=none&type=grpc&security=tls&serviceName=vlgr&fp=chrome&sni={}#{}",
                    id,
                    ip_data.ip,
                    domain,
                    format!(
                        "{} {} (ping:{})",
                        emojic::country_flag(country.alpha2),
                         ip_data.id,
                         ip_data.ping_avg.unwrap_or(0.0)
                        )
                )
            })
            .collect()
    });

    return match data {
        Ok(rows) => {
            let x = rows.join("\n");
            let b64 = general_purpose::STANDARD.encode(x.as_bytes());
            return Ok(b64.into());
        }
        Err(err) => {
            println!("{:?}", err);
            Err(tide::Error::from_str(404, "Not Found"))
        }
    };
}

