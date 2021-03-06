use postgres::{NoTls, Client};
use crate::data::{UserData, ItemData};
use crate::key;

pub fn check_database(args: &str) {
  let s = String::from(args) + " password=" + key::PASSWORD;
  let ss = String::from(args) + " dbname=jhlaf password=" + key::PASSWORD;
  let mut client = match Client::connect(&ss, NoTls) {
    Ok(c) => c,
    Err(_) => {
      let mut c = Client::connect(&s, NoTls).unwrap();
      c.batch_execute("CREATE DATABASE jhlaf").unwrap();
      Client::connect(&ss, NoTls).unwrap()
    }
  };
  if let Err(e) = client.batch_execute("
    CREATE TABLE IF NOT EXISTS users (
      id          SERIAL PRIMARY KEY,
      username    TEXT NOT NULL,
      password    TEXT NOT NULL,
      userid      TEXT NOT NULL,
      contact     TEXT NOT NULL,
      _group       TEXT NOT NULL
    )
  ") {
    panic!("{:?}", e);
  }
  if let Err(e) = client.batch_execute("
    CREATE TABLE IF NOT EXISTS items (
      id          BIGSERIAL PRIMARY KEY,
      item_id     BIGSERIAL,
      lof         BOOLEAN,
      userid      TEXT NOT NULL,
      type        TEXT NOT NULL,
      name        TEXT NOT NULL,
      image_url   TEXT NOT NULL,
      _desc        TEXT NOT NULL,
      pickup_time BIGINT,
      place       TEXT NOT NULL,
      contact     TEXT NOT NULL,
      post_time   BIGINT
    )
  ") {
    panic!("{:?}", e);
  }
//  if let Err(_) = client.batch_execute("
//    DROP TABLE IF EXISTS logs
//  ") {
//    // do nothing
//  }
//  if let Err(e) = client.batch_execute("
//    CREATE TABLE IF NOT EXISTS logs (
//      id          UNIQUE BIGSERIAL PRIMARY KEY,
//      operation   TEXT NOT NULL,
//      item_id     BIGINT,
//      timestamp   BIGINT,
//    )
//  ") {
//    panic!("{:?}", e);
//  }
}

pub fn check_username(client: &mut Client, username: &str) -> bool {
  let mut exist: bool = false;
  for _ in client.query("SELECT * FROM users WHERE username = $1", &[&username]).unwrap() {
    exist = true;
  }
  exist
}

pub fn insert_user(client: &mut Client, username: &str, password: &str, contact: &str, userid: &str) -> bool {
  match client.execute("
    INSERT INTO users (username, password, userid, contact, _group)
    VALUES ($1, $2, $3, $4, 'normal')
  ", &[&username, &password, &userid, &contact]) {
    Ok(_) => {
      true
    },
    Err(e) => {
      println!("{:?}", e);
      false
    }
  }
}

pub fn check_userid(client: &mut Client, userid: &str) -> bool {
  let mut exist: bool = false;
  for _ in client.query("SELECT * FROM users WHERE userid = $1", &[&userid]).unwrap() {
    exist = true;
  }
  exist
}

pub fn query_userdata(client: &mut Client, userid: &str) -> Result<UserData, String> {
  let mut cnt: i32 = 0;
  let mut dat: UserData = UserData {
    username: String::new(), userid: String::new(), contact: String::new(), group: String::new()
  };
  for row in client.query("SELECT * FROM users WHERE userid = $1", &[&userid]).unwrap() {
    cnt += 1;
    dat = UserData {
      username: row.get(1),
      userid: row.get(3),
      contact: row.get(4),
      group: row.get(5)
    };
  }
  if cnt == 0 { Err(String::from("0401")) } else if cnt == 1 { Ok(dat) } else { Err(String::from("0402")) }
}

pub fn insert_item(client: &mut Client, userid: &str, d: &ItemData) -> bool {
  match client.execute("
    INSERT INTO items (lof, userid, type, name, image_url, _desc, pickup_time, place, contact, post_time)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
  ", &[&d.islost, &userid, &d.item_type, &d.item_name, &d.image_url, &d.desc, &d.pickup_time, &d.place, &d.contact, &d.post_time]) {
    Ok(_) => {
      true
    },
    Err(e) => {
      println!("{:?}", e);
      false
    }
  }
}

pub fn delete_item(client: &mut Client, userid: &str, itemid: i64) -> bool {
  // TODO: admin support
  match client.execute("
    DELETE FROM items WHERE itemid = $1 and userid = $2
  ", &[&itemid, &userid]) {
    Ok(_) => {
      true
    },
    Err(e) => {
      println!("{:?}", e);
      false
    }
  }
}

