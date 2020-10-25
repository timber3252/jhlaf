use postgres::{NoTls, Client};

pub fn check_database(args: &str) {
  let s = String::from(args);
  let ss = String::from(args) + " dbname=jhlaf";
  let mut client = match Client::connect(&ss, NoTls) {
    Ok(c) => c,
    Err(e) => {
      if e.code().unwrap().code() == "3D000" {
        let mut c = Client::connect(&s, NoTls).unwrap();
        c.batch_execute("CREATE DATABASE jhlaf").unwrap();
        Client::connect(&ss, NoTls).unwrap()
      } else {
        panic!("{:?}", e);
      }
    }
  };
  if let Err(e) = client.batch_execute("
    CREATE TABLE IF NOT EXISTS users (
      id          SERIAL PRIMARY KEY,
      username    TEXT NOT NULL,
      password    TEXT NOT NULL,
      userid      TEXT NOT NULL,
      contact     TEXT NOT NULL,
      group       TEXT NOT NULL
    )
  ") {
    panic!("{:?}", e);
  }
  if let Err(e) = client.batch_execute("
    CREATE TABLE IF NOT EXISTS items (
      id          SERIAL PRIMARY KEY,
      lof         BOOLEAN,
      userid      TEXT NOT NULL,
      type        TEXT NOT NULL,
      name        TEXT NOT NULL,
      image_url   TEXT NOT NULL,
      desc        TEXT NOT NULL,
      pickup_time TIMESTAMP WITHOUT TIME ZONE,
      place       TEXT NOT NULL,
      contact     TEXT NOT NULL,
      post_time   TIMESTAMP WITHOUT TIME ZONE,
      tags        TEXT NOT NULL
    )
  ") {
    panic!("{:?}", e);
  }
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
    INSERT INTO users (username, password, userid, contact, group)
    VALUES ($1, $2, $3, $4, \"normal\")
  ", &[&username, &password, &userid, &contact]) {
    Ok(_) => {
      true
    },
    Err(_) => {
      false
    }
  }
}
