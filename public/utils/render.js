function render_homepage() {
  // console.log('render_homepage()');
  // if (!page_stack.isEmpty())
  //   return;
  // if (global_userid == "")
  //   return;
  // let ws = new WebSocket(ws_server_addr);
  // ws.onopen = function (e) {
  //   console.log('render: connected to ws server');
  //   ws.send(JSON.stringify({

  //   }));
  // }
  // ws.onmessage = function (e) {
  //   ws.send(JSON.stringify({
  //     type: "close_session"
  //   }));
  //   ws.close();
  // }
  // ws.onclose = function (e) {
  //   console.log('render: disconnected');
  // }
}

function render_laf() {
  // console.log('render_laf()');
  // if (!page_stack.isEmpty())
  //   return;
  // if (global_userid == "")
  //   return;
  // let ws = new WebSocket(ws_server_addr);
  // ws.onopen = function (e) {
  //   console.log('render: connected to ws server');
  //   ws.send(JSON.stringify({

  //   }));
  // }
  // ws.onmessage = function (e) {
  //   ws.send(JSON.stringify({
  //     type: "close_session"
  //   }));
  //   ws.close();
  // }
  // ws.onclose = function (e) {
  //   console.log('render: disconnected');
  // }
}

function render_me() {
  // console.log('render_me()');
  if (!page_stack.isEmpty())
    return;
  if (global_userid == "")
    return;
  let ws = new WebSocket(ws_server_addr);
  ws.onopen = function (e) {
    console.log('render: connected to ws server');
    ws.send(JSON.stringify({
      type: "query_userdata",
      userid: global_userid
    }));
  }
  ws.onerror = function (e) {
    console.error('render fatal error: ', e);
  }
  ws.onmessage = function (e) {
    // console.log(e.data);
    let d = JSON.parse(e.data);
    if (d.type == "result" && d.result_type == "query_userdata") {
      ws.send(JSON.stringify({
        type: "close_session"
      }));
      if (d.stat == true) {
        document.getElementById('userinfo-usericon').setAttribute("src", "https://avatars3.githubusercontent.com/" + d.data.username);
        document.getElementById('userinfo-username').innerHTML = d.data.username + '<br><div class="exsmall fgrey">' + (d.data.group == "admin" ? "管理员用户" : "普通用户" ) + '</div>';
      } else {
        if (d.err == "0401")
          alert("获取数据失败：用户信息不合法或不存在");
        else
          alert("获取数据失败：未知错误");
      }
      ws.close();
    }
  }
  ws.onclose = function (e) {
    console.log('render: disconnected');
  }
}

function render_page() {
  if (current_page_id == 'homepage') {
    render_homepage();
  } else if (current_page_id == 'laf') {
    render_laf();
  } else if (current_page_id == 'me') {
    render_me();
  }
}