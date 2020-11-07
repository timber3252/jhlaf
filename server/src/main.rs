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
mod key;

// TODO: SAVE PICTURES

fn ws_send(ws: &mut tungstenite::WebSocket<std::net::TcpStream>, data: String) {
  if let Err(e) = ws.write_message(Message::Text(data)) {
    println!("{:?}", e);
  }
}

fn handle_conn(mut ws: &mut tungstenite::WebSocket<std::net::TcpStream>) {
  let mut client = match Client::connect(&(String::from("host=localhost user=postgres dbname=jhlaf password=") + key::PASSWORD), NoTls) {
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
          println!("{}", parsed);
          if parsed["type"] == "register" {
            let (username, password, contact) = (&parsed["username"], &parsed["password"], &parsed["contact"]);
            if username.is_null() || password.is_null() || contact.is_null() {
              continue;
            }
            if !username.is_string() || !password.is_string() || !contact.is_string() {
              continue;
            }
            if username.as_str().unwrap().len() < 4 || username.as_str().unwrap().len() > 32 {
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
              continue;
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
            if username.as_str().unwrap().len() < 4 || username.as_str().unwrap().len() > 32 {
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
              continue;
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
                    \"userid\": \"{}\"
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
              break;
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
            if userid.is_null() || !userid.is_string() || !db::check_userid(&mut client, &userid.as_str().unwrap()) {
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
          } else if parsed["type"] == "publish" {
            let userid = &parsed["userid"];
            if userid.is_null() || !userid.is_string() || !db::check_userid(&mut client, &userid.as_str().unwrap()) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "publish",
                  "stat": false,
                  "err": "0501"
                }"#));
              continue;
            }
            let itemdata = &parsed["item"];
            if itemdata.is_null() || !itemdata.is_object() {
              continue;
            }
            // todo check valid
            match db::insert_item(&mut client, &userid.as_str().unwrap(), 
              &data::ItemData {
                item_id: -1,
                islost: parsed["item"]["lof"].as_bool().unwrap(),
                userid: String::from(userid.as_str().unwrap()),
                item_type: String::from(parsed["item"]["type"].as_str().unwrap()),
                item_name: String::from(parsed["item"]["name"].as_str().unwrap()),
                image_url: String::from(parsed["item"]["image"].as_str().unwrap()),
                desc: String::from(parsed["item"]["desc"].as_str().unwrap()),
                pickup_time: parsed["item"]["pickup_time"].as_i64().unwrap(),
                place: String::from(parsed["item"]["place"].as_str().unwrap()),
                contact: String::from(parsed["item"]["contact"].as_str().unwrap()),
                post_time: parsed["item"]["post_time"].as_i64().unwrap(),
              }) {
                true => {
                  ws_send(&mut ws, String::from(r#"
                  {
                    "type": "result",
                    "result_type": "publish",
                    "stat": true,
                  }"#));
                },
                false => {
                  ws_send(&mut ws, String::from(r#"
                  {
                    "type": "result",
                    "result_type": "publish",
                    "stat": false,
                    "err": "0502"
                  }"#));
                }
              }
          } else if parsed["type"] == "delete" {
            let userid = &parsed["userid"];
            if userid.is_null() || !userid.is_string() || !db::check_userid(&mut client, &userid.as_str().unwrap()) {
              ws_send(&mut ws, String::from(r#"
                {
                  "type": "result",
                  "result_type": "delete",
                  "stat": false,
                  "err": "0601"
                }"#));
              continue;
            }
            match db::delete_item(&mut client, &userid.as_str().unwrap(), parsed["itemid"].as_i64().unwrap()) {
              true => {
                ws_send(&mut ws, String::from(r#"
                  {
                    "type": "result",
                    "result_type": "delete",
                    "stat": true,
                  }"#));
              },
              false => {
                ws_send(&mut ws, String::from(r#"
                  {
                    "type": "result",
                    "result_type": "delete",
                    "stat": true,
                    "err": "0602"
                  }"#));
              }
            }
          } else if parsed["type"] == "select_all" {

          } else if parsed["type"] == "select_me" {
            
          } else if parsed["type"] == "close_session" {
            break;
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
  db::check_database(&(String::from("host=localhost user=postgres password=") + key::PASSWORD));
  wss::ws_server("0.0.0.0:3001", handle_conn);
}
