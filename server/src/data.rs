pub struct UserData {
  pub username: String,
  pub userid: String,
  pub contact: String,
  pub group: String
}

pub struct ItemData {
  pub item_id: i64,
  pub islost: bool,
  pub userid: String,
  pub item_type: String,
  pub item_name: String,
  pub image_url: String,
  pub desc: String,
  pub pickup_time: i64,
  pub place: String,
  pub contact: String,
  pub post_time: i64,
}
