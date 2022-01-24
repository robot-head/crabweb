#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub async fn launch() -> Result<(), napi::Error> {
  _launch().await.map_err(convert_err)
}

async fn _launch() -> Result<(), rocket::Error> {
  rocket::build().ignite().await?.launch().await
}

fn convert_err(rocket_err: rocket::Error) -> napi::Error {
  let reason = match rocket_err.kind() {
    rocket::error::ErrorKind::Bind(bind_err) => {
      format!("Rocket bind error: {}", bind_err.to_string())
    }
    rocket::error::ErrorKind::Io(io_err) => format!("Rocket IO error: {}", io_err.to_string()),
    rocket::error::ErrorKind::Runtime(runtime_err) => {
      format!("Rocket runtime error: {}", runtime_err)
    }
    rocket::error::ErrorKind::Config(config_err) => format!("Rocket config error: {}", config_err),
    rocket::error::ErrorKind::Collisions(collisions_err) => {
      format!("Rocket collision error: {:?}", collisions_err)
    }
    rocket::error::ErrorKind::FailedFairings(failed_fairings_err) => {
      format!(
        "Rocket fairings error: {}",
        failed_fairings_err
          .iter()
          .map(|info| format!("{:?}", info))
          .collect::<Vec<String>>()
          .join(",")
      )
    }
    rocket::error::ErrorKind::SentinelAborts(_sentinel_aborts_err) => {
      format!("Rocket sentinel abort error")
    }
    rocket::error::ErrorKind::InsecureSecretKey(insecure_secrets_key_err) => {
      format!(
        "Rocket insecure secrets key error: {}",
        insecure_secrets_key_err
      )
    }
    _ => format!("Unknown rocket error"),
  };
  napi::Error::from_reason(reason)
}
