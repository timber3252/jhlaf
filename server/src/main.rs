use json;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use digest::Digest;
use postgres::{NoTls, Client};

mod wss;
mod db;

// TODO: SAVE PICTURES

fn result_json() -> String {
  let mut res: String = String::new();
  res
}

fn handle_conn(ws: &mut tungstenite::WebSocket<std::net::TcpStream>, dbarg: &str) {
  let mut client = match Client::connect(dbarg, NoTls) {
    Ok(c) => c,
    Err(err) => {
      ws.close(None);
      return;
    }
  };
  loop {
    if let Ok(msg_raw) = ws.read_message() {
      if let Ok(msg) = msg_raw.to_text() {
        if let Ok(parsed) = json::parse(msg) {
          if parsed["type"] == "register" {
            let (username, password, contact) = (&parsed["username"], &parsed["password"], &parsed["contact"]);
            if username.is_null() || password.is_null() || contact.is_null() {
              continue;
            }
            if username.len() < 4 || username.len() > 32 || password.len() == 0 || contact.len() == 0 {
              continue;
            }
            // TODO: Double check: make sure username doesn't have any other chars
            
            if db::check_username(&mut client, username.as_str().unwrap()) {
              continue;
            }
            let uid = calc_userid(&username.dump(), &password.dump());
          } else if parsed["type"] == "login" {

          } else if parsed["type"] == "query_user" {

          } else if parsed["type"] == "query_userdata" {

          } else if parsed["type"] == "publish_lost" {

          } else if parsed["type"] == "publish_found" {

          } else if parsed["type"] == "delete_lost" {

          } else if parsed["type"] == "delete_found" {

          } else if parsed["type"] == "select" {

          }
        }
      }
    }
  }
}

fn calc_userid(usr: &str, pwd: &str) -> String {
  type HmacSha256 = Hmac<Sha256>;
  if let Ok(mut mac) = HmacSha256::new_varkey(pwd.as_bytes()) {
    mac.update(usr.as_bytes());
    mac.update(pwd.as_bytes());
    return format!("{:x}", Sha256::digest(&mac.finalize().into_bytes()));
  }
  String::from(pwd) + usr
}

fn main() {
  db::check_database("host=localhost user=postgres");
  wss::ws_server("0.0.0.0:3001", handle_conn);
}
