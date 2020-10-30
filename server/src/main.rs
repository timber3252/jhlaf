use json;
use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
use digest::Digest;
use postgres::{NoTls, Client};
use regex::Regex;
use tungstenite::Message;

mod wss;
mod db;
mod data;

// TODO: SAVE PICTURES

const DATABASE_ARG: &str = "host=localhost user=postgres";

fn result_json() -> String {
  let mut res: String = String::new();
  res
}

fn ws_send(ws: &mut tungstenite::WebSocket<std::net::TcpStream>, data: String) {
  if let Err(e) = ws.write_message(Message::Text(data)) {
    println!("{:?}", e);
  }
}

fn handle_conn(mut ws: &mut tungstenite::WebSocket<std::net::TcpStream>) {
  let mut client = match Client::connect(DATABASE_ARG, NoTls) {
    Ok(c) => c,
    Err(x) => {
      println!("{:?}", x);
      if let Err(e) = ws.close(None) {
        println!("{:?}", e);
      }
      return;
    }
  };
  let re_username = Regex::new(r#"[^A-Za-z0-9_]"#).unwrap();
  loop {
    if let Ok(msg_raw) = ws.read_message() {
      if let Ok(msg) = msg_raw.to_text() {
        if let Ok(parsed) = json::parse(msg) {
          if parsed["type"].is_null() || !parsed["type"].is_string() {
            continue;
          }
          if parsed["type"] == "register" {
            let (username, password, contact) = (&parsed["username"], &parsed["password"], &parsed["contact"]);
            if username.is_null() || password.is_null() || contact.is_null() {
              continue;
            }
            if !username.is_string() || !password.is_string() || !contact.is_string() {
              continue;
            }
            if username.len() < 4 || username.len() > 32 || password.len() < 6 || password.len() > 32 || contact.len() == 0 {
              continue;
            }
            let ss = username.as_str().unwrap();
            if re_username.is_match(&ss) {
              continue;
            }
            if db::check_username(&mut client, ss) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "register",
                  "stat": false,
                  "err": "0101"
                }"#));
            }
            let userid = calc_userid(&username.as_str().unwrap(), &password.as_str().unwrap());
            if db::insert_user(&mut client, &username.as_str().unwrap(), &password.as_str().unwrap(), &contact.as_str().unwrap(), &userid) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "register",
                  "stat": true
                }"#));
            } else {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "register",
                  "stat": false,
                  "err": "0102"
                }"#));
            }
          } else if parsed["type"] == "login" {
            let (username, password) = (&parsed["username"], &parsed["password"]);
            if username.is_null() || password.is_null() {
              continue;
            }
            if !username.is_string() || !password.is_string() {
              continue;
            }
            if username.len() < 4 || username.len() > 32 || password.len() < 6 || password.len() > 32 {
              continue;
            }
            let ss = username.as_str().unwrap();
            if re_username.is_match(&ss) {
              continue;
            }
            if !db::check_username(&mut client, ss) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "login",
                  "stat": false,
                  "err": "0201"
                }"#));
            }
            let mut done: bool = false;
            for row in client.query("SELECT * FROM users WHERE username = $1", &[&username.as_str().unwrap()]).unwrap() {
              let standard_pwd: String = row.get(2);
              let userid: String = row.get(3);
              if standard_pwd == String::from(password.as_str().unwrap()) {
                ws_send(&mut ws, format!("
                  {{
                    \"type\": \"result\",
                    \"result_type\": \"login\",
                    \"stat\": true,
                    \"userid\": {}
                  }}", userid));
              } else {
                ws_send(&mut ws, String::from(r#"
                  {
                    "type": "result",
                    "result_type": "login",
                    "stat": false,
                    "err": "0202"
                  }
                }"#));
              }
              done = true;
            }
            if !done {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "login",
                  "stat": false,
                  "err": "0202"
                }
              }"#));
            }
          } else if parsed["type"] == "query_user" {
            let userid = &parsed["userid"];
            // TRUNC IF userid IS TOO LONG
            if userid.is_null() || !userid.is_string() || !db::check_userid(&mut client, &userid.as_str().unwrap()) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "query_user",
                  "stat": false,
                  "err": "0301"
                }"#));
              continue;
            }
            ws_send(&mut ws, String::from(r#"
              {
                "type": "result",
                "result_type": "query_user",
                "stat": true
              }"#));
          } else if parsed["type"] == "query_userdata" {
            let userid = &parsed["userid"];
            if userid.is_null() || !userid.is_string() || !db::check_username(&mut client, &userid.as_str().unwrap()) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "query_userdata",
                  "stat": false,
                  "err": "0401"
                }"#));
              continue;
            }
            match db::query_userdata(&mut client, &userid.as_str().unwrap()) {
              Ok(v) => {
                ws_send(&mut ws, format!("
                  {{
                    \"type\": \"result\",
                    \"result_type\": \"query_userdata\",
                    \"stat\": true,
                    \"data\": {{
                      \"username\": \"{}\",
                      \"contact\": \"{}\",
                      \"group\": \"{}\"
                    }}
                  }}
                ", v.username, v.contact, v.group));
              }
              Err(e) => {
                ws_send(&mut ws, format!("
                  {{
                    \"type\": \"result\",
                    \"result_type\": \"query_userdata\",
                    \"stat\": false,
                    \"err\": \"{}\"
                  }}
                ", e));
              }
            }
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
  db::check_database(DATABASE_ARG);
  wss::ws_server("0.0.0.0:3001", handle_conn);
}
