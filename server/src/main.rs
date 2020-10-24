mod db;
mod wss;

fn handle_conn(stream: &mut tungstenite::WebSocket<std::net::TcpStream>) {
  
}

fn main() {
  wss::ws_server("0.0.0.0:3001", handle_conn);
}
