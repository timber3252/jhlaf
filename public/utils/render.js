// TODO: time calcuate and simplification

function render_homepage() {
  // console.log('render_homepage()');
  if (!page_stack.isEmpty())
    return;
  if (global_userid == "")
    return;
  let ws = new WebSocket(ws_server_addr);
  ws.onopen = function (e) {
    console.log('render: connected to ws server');
    ws.send(JSON.stringify({
      type: "select_me",
      userid: global_userid,
      lof: true
    }));
    ws.send(JSON.stringify({
      type: "select_me",
      userid: global_userid,
      lof: false
    }));
  }
  let acnt = 0;
  ws.onmessage = function (e) {
    console.log(e.data);
    let d = JSON.parse(e.data);
    if (d.type == "result" && d.result_type == "select_me") {
      ++acnt;
      if (acnt == 2) {
        ws.send(JSON.stringify({
          type: "close_session"
        }));
      }
      let res = "";
      if (d.stat == true) {
        // let regex = /(?=\$\{).*?(?<=\})/g;
        for (let cur in d.items) {
          if (cur == 0) {
            let p = homepage_item_template_first;
            p = p.replace('${itemid}', d.items[cur].itemid);
            // console.log(d.items[cur].itemid);
            p = p.replace('${type_icon}', d.items[cur].type == "keys" ? item_icon_key : (d.items[cur].type == "cards" ? item_icon_cards : item_icon_others));
            // console.log(d.items[cur].type == "keys" ? item_icon_key : (d.items[cur].type == "cards" ? item_icon_cards : item_icon_others));
            p = p.replace('${name}', d.items[cur].name);
            // console.log(d.items[cur].name);
            p = p.replace('${pickup_time}', (new Date(parseInt(d.items[cur].pickup_time))).toLocaleDateString() + ' ' + (new Date(parseInt(d.items[cur].pickup_time))).toLocaleTimeString());
            // console.log((new Date(parseInt(d.items[cur].pickup_time))).toLocaleDateString() + ' ' + (new Date(parseInt(d.items[cur].pickup_time))).toLocaleTimeString());
            // console.log(p);
            res += p;
          } else {
            let p = homepage_item_template;
            p = p.replace('${itemid}', d.items[cur].itemid);
            // console.log(p);
            p = p.replace('${type_icon}', d.items[cur].type == "keys" ? item_icon_key : (d.items[cur].type == "cards" ? item_icon_cards : item_icon_others));
            // console.log(p);
            p = p.replace('${name}', d.items[cur].name);
            // console.log(p);
            p = p.replace('${pickup_time}', (new Date(parseInt(d.items[cur].pickup_time))).toLocaleDateString() + ' ' + (new Date(parseInt(d.items[cur].pickup_time))).toLocaleTimeString());
            // console.log(p);
            res += p;
          }
        }
        if (d.lof == true) {
          document.getElementById('my-lost-panel').innerHTML = '<div class="lineflex-v panel-title"><div class="fblack small bold titlebox">我的失物</div></div>' + (res == "" ? '<div class="lineflex panel-item-first exsmall">暂无数据</div>' : res);
        } else {
          document.getElementById('my-found-panel').innerHTML = '<div class="lineflex-v panel-title"><div class="fblack small bold titlebox">我的招领</div></div>' + (res == "" ? '<div class="lineflex panel-item-first exsmall">暂无数据</div>' : res);
        }
      } else {
        if (d.err == "0801") {
          alert("用户信息不合法或不存在");
        } else {
          alert("未知错误");
        }
      }
      if (acnt == 2) ws.close();
    }
  }
  ws.onclose = function (e) {
    console.log('render: disconnected');
  }
}

function render_laf() {
  // console.log('render_laf()');
  if (!page_stack.isEmpty())
    return;
  if (global_userid == "")
    return;
  let ws = new WebSocket(ws_server_addr);
  let lb = document.getElementsByClassName('left-button');
  islost = lb[0].classList.contains('enable');
  let rb = document.getElementById('do-filter-button');
  ws.onopen = function (e) {
    console.log('render: connected to ws server');
    ws.send(JSON.stringify({
      type: "select_all",
      userid: global_userid,
      lof: islost,
      item_type: rb.value,
      time_begin: "0",
      time_end: "4503599627370496"
    }));
  }
  ws.onmessage = function (e) {
    // console.log(e.data);
    let d = JSON.parse(e.data);
    if (d.type == "result" && d.result_type == "select_all") {
      ws.send(JSON.stringify({
        type: "close_session"
      }));
      let res = "";
      if (d.stat == true) {
        // let regex = /(?=\$\{).*?(?<=\})/g;
        for (let cur in d.items) {
          let p = laf_item_template;
          p = p.replace('${itemid}', d.items[cur].itemid);
          p = p.replace('${type_icon}', d.items[cur].type == "keys" ? item_icon_key : (d.items[cur].type == "cards" ? item_icon_cards : item_icon_others));
          p = p.replace('${name}', d.items[cur].name);
          p = p.replace('${pickup_time}', (new Date(parseInt(d.items[cur].pickup_time))).toLocaleDateString() + ' ' + (new Date(parseInt(d.items[cur].pickup_time))).toLocaleTimeString());
          p = p.replace('${place}', d.items[cur].place);
          p = p.replace('${desc}', d.items[cur].desc);
          res += p;
        }
        document.getElementById('main-content-id').innerHTML = res;
      } else {
        if (d.err == "0801") {
          alert("用户信息不合法或不存在");
        } else {
          alert("未知错误");
        }
      }
      ws.close();
    }
  }
  ws.onclose = function (e) {
    console.log('render: disconnected');
  }
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