use std::fs::File;
use std::io::Read;
use std::sync::Arc;
use std::thread::spawn;
use std::net::{TcpListener};
use tungstenite::{server::accept};
use native_tls::{Identity, TlsAcceptor};

pub fn ws_server(addr: &str, handle_conn: fn(&mut tungstenite::WebSocket<std::net::TcpStream>)) {
  let server = TcpListener::bind(addr).expect("failed to create websocket server");
  for stream in server.incoming() {
    if let Ok(ss) = stream {
      let wsacc = accept(ss);
      if let Ok(mut ws) = wsacc {
        spawn(move || handle_conn(&mut ws));
      }
    }
  }
}

pub fn wss_server(addr: &str, filepath: &str, handle_conn: fn(&mut tungstenite::WebSocket<native_tls::TlsStream<std::net::TcpStream>>)) {
  let mut file = File::open(filepath).expect("failed to open pkcs#12 cert file");
  let mut identity = vec![];
  file.read_to_end(&mut identity).expect("failed to read pkcs#12 cert file");
  let identity = Identity::from_pkcs12(&identity, "").unwrap();
  let server = TcpListener::bind(addr).expect("failed to create websocket server");
  let acceptor = TlsAcceptor::new(identity).expect("failed to initialize TlsAcceptor");
  let acceptor = Arc::new(acceptor);
  for stream in server.incoming() {
    if let Ok(ss) = stream {
      let wsacc = acceptor.accept(ss);
      if let Ok(wsac) = wsacc {
        let wsac = accept(wsac);
        if let Ok(mut ws) = wsac {
          spawn(move || handle_conn(&mut ws));
        }
      }
    }
  }
}
