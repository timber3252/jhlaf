function setCookie(name, value, expire_days) {
  let cookie_str = name + "=" + escape(value); 
  if (expire_days != null) {
    cookie_str += ";path=/;max-age=" + expire_days * 24 * 3600;
  }
  document.cookie = cookie_str;
}

function getCookie(name) {
  if (document.cookie.length > 0) {
    let cookies = document.cookie.split(';');
    for (let x of cookies) {
      let key = x.split('=')[0], value = x.split('=')[1];
      if (key == name) return value;
    }
  }
  return "";
}


function checkPassword(pwd) {
  if (pwd == "") {
    alert("请输入密码");
    return false;
  }
  if (pwd.length < 6 || pwd.length > 32) {
    alert("密码的长度需要在 6 到 32 之间");
    return false;
  }
  return true;
}

function checkUsername(usr) {
  if (usr == "") {
    alert("请输入用户名！");
    return false;pwd = resolvePassword(usr, pwd);
  }
  var patrn = /^[a-zA-Z0-9]{4,32}$/;
  if (!patrn.exec(usr)) {
    alert("用户名只能由字母和数字构成，并且长度在 4 到 32 之间");
    return false;
  }
  return true;
}

function resolvePassword(usr, pwd) {
  let hmac = CryptoJS.algo.HMAC.create(CryptoJS.algo.SHA256, pwd);
  hmac.update("zjutjh");
  hmac.update(usr);
  hmac.update("zjutjh");
  let hash = hmac.finalize();
  return hash.toString(CryptoJS.enc.Hex);
}

function login() {
  let usr = document.getElementById('login-username-textbox').value;
  let pwd = document.getElementById('login-password-textbox').value;
  if (checkUsername(usr) && checkPassword(pwd)) {
    pwd = resolvePassword(usr, pwd);
    let ws = new WebSocket(ws_server_addr);
    ws.onopen = function (e) {
      console.log('login: connected to ws server');
      ws.send(JSON.stringify({
        type: "login",
        username: usr,
        password: pwd
      }));
    }
    ws.onerror = function (e) {
      console.error('login fatal error:', e);
    }
    ws.onmessage = function (e) {
      // console.log(e.data);
      let d = JSON.parse(e.data);
      if (d.type == "result" && d.result_type == "login") {
        ws.send(JSON.stringify({
          type: "close_session"
        }));
        if (d.stat == true) {
          setCookie('JHLAF_USERID', d.userid, 15)
          pop_page_stack();
        } else {
          alert("用户名或密码错误");
        }
        ws.close();
      }
    }
    ws.onclose = function (e) {
      console.log('login: disconnected');
    }
  }
}

function register() {
  let usr = document.getElementById('register-username-textbox').value;
  let pwd = document.getElementById('register-password-textbox').value;
  let cont = document.getElementById('register-contact-textbox').value;
  if (checkUsername(usr) && checkPassword(pwd)) {
    pwd = resolvePassword(usr, pwd);
    let ws = new WebSocket(ws_server_addr);
    ws.onopen = function (e) {
      console.log('register: connected to ws server');
      ws.send(JSON.stringify({
        type: "register",
        username: usr,
        password: pwd,
        contact: cont
      }));
    }
    ws.onerror = function (e) {
      console.error('register fatal error:', e);
    }
    ws.onmessage = function (e) {
      let d = JSON.parse(e.data);
      if (d.type == "result" && d.result_type == "register") {
        ws.send(JSON.stringify({
          type: "close_session"
        }));
        if (d.stat == true) {
          alert("注册成功，请重新登录");
          pop_page_stack();
        } else {
          if (d.err == "0101")
            alert("注册出错，用户已存在");
          else if (d.err == "0102")
            alert("注册出错，未知错误");
        }
        ws.close();
      }
    }
    ws.onclose = function (e) {
      console.log('register: disconnected');
    }
  }
}

function checkStatus() {
  global_userid = getCookie('JHLAF_USERID');
  if (global_userid != "") {
    let ws = new WebSocket(ws_server_addr);
    ws.onopen = function (e) {
      console.log('query_user: connected to ws server');
      ws.send(JSON.stringify({
        type: "query_user",
        userid: global_userid
      }));
    }
    ws.onerror = function (e) {
      console.error('query_user fatal error:', e);
    }
    ws.onmessage = function (e) {
      let d = JSON.parse(e.data);
      if (d.type == "result" && d.result_type == "query_user") {
        ws.send(JSON.stringify({
          type: "close_session"
        }));
        if (d.stat == true) {
          render_page();
        } else {
          push_page_stack('login');
        }
        ws.close();
      }
    }
    ws.onclose = function (e) {
      console.log('query_user: disconnected');
    }
  } else {
    push_page_stack('login');
  }
}

function publish_post() {
  let lb = document.getElementsByClassName('left-button1');
  islost = lb[0].classList.contains('enable');
  item_type = document.getElementById('do-filter-button1').value;
  item_name = document.getElementById('item_name').value;
  item_place = document.getElementById('item_place').value;
  item_pickup_time = document.getElementById('item_pickup_time').value;
  item_image = document.getElementById('item_image').value;
  item_contact = document.getElementById('item_contact').value;
  item_desc = document.getElementById('item_desc').value;
  // console.log(islost, item_type, item_name, item_place, item_pickup_time, 
  //   item_contact, item_desc);
  // console.log(item_image);
  let ws = new WebSocket(ws_server_addr);
  ws.send(JSON.stringify({
    type: "publish",
    lof: islost,
    userid: global_userid,
    item: {
      itemid: "-1",
      type: item_type,
      name: item_name,
      image: "", // TODO
      desc: item_desc,
      pickup_time: (new Date(item_pickup_time)).getTime().toString(),
      place: item_place,
      contact: item_contact,
      post_time: (new Date()).getTime().toString()
    }
  }));
}
