function page_initial() {
  page_dict = { "homepage": null, "laf": null, "me": null };
  back = null;
  current_page_id = null;
  default_page = "homepage";
  page_update(default_page);
}

function Stack() {
  this.items = []
  Stack.prototype.push = function (element) {
    this.items.push(element)
  }
  Stack.prototype.pop = () => {
    return this.items.pop()
  }
  Stack.prototype.peek = () => {
    return this.items[this.items.length - 1]
  }
  Stack.prototype.isEmpty = () => {
    return this.items.length == 0
  }
  Stack.prototype.size = () => {
    return this.items.length
  }
  Stack.prototype.toString = () => {
    let resultString = ''
    for (let i of this.items) {
      resultString += i + ' '
    }
    return resultString
  }
}

var page_stack = new Stack();

function page_update(target_page_id) {
  if (!page_stack.isEmpty())
    return;
  var content = document.querySelector('.content');
  if (current_page_id != null) {
    page_dict[current_page_id] = content.innerHTML;
  }
  if (page_dict[target_page_id] == null) {
    fetch('/pages/' + target_page_id + '.html')
      .then(function (response) {
        response.text().then(function (data) {
          if (data.length > 0) {
            content.innerHTML = data;
          } else {
            content.innerHTML = "";
            console.log('fetch error: data is null');
          }
        })
      })
  } else {
    content.innerHTML = page_dict[target_page_id];
  }
  if (target_page_id == "homepage") {
    document.getElementById("nav-item-homepage-svg").setAttribute("fill", "#0078D7");
    document.getElementById("nav-item-laf-svg").setAttribute("fill", "#808080");
    document.getElementById("nav-item-me-svg").setAttribute("fill", "#808080");
  } else if (target_page_id == "laf") {
    document.getElementById("nav-item-homepage-svg").setAttribute("fill", "#808080");
    document.getElementById("nav-item-laf-svg").setAttribute("fill", "#0078D7");
    document.getElementById("nav-item-me-svg").setAttribute("fill", "#808080");
  } else if (target_page_id == "me") {
    document.getElementById("nav-item-homepage-svg").setAttribute("fill", "#808080");
    document.getElementById("nav-item-laf-svg").setAttribute("fill", "#808080");
    document.getElementById("nav-item-me-svg").setAttribute("fill", "#0078D7");
  }
  current_page_id = target_page_id;
  render_page();
}

function page_refresh(target_page_id) {
  if (!page_stack.isEmpty())
    return;
  if (current_page_id != target_page_id)
    return;
  var content = document.querySelector('.content');
  fetch('/pages/' + target_page_id + '.html')
    .then(function (response) {
      response.text().then(function (data) {
        if (data.length > 0) {
          content.innerHTML = data;
        } else {
          content.innerHTML = "";
          console.log('refresh error: data is null');
        }
      })
    })
  render_page();
}

function page_cover(target_page_path) {
  var content = document.querySelector('.content');
  fetch('/pages/' + target_page_path + '.html')
  .then(function (response) {
    response.text().then(function (data) {
      if (data.length > 0) {
        content.innerHTML = data;
      } else {
        content.innerHTML = "";
        console.log('refresh error: data is null');
      }
    })
  })
}

function push_page_stack(target_page_path) {
  var content = document.querySelector('.content');
  if (page_stack.isEmpty()) {
    // the first element, we need to store the prev stat
    page_dict[current_page_id] = content.innerHTML;
    // document.getElementById("global-footer").toggleAttribute('hidden');
    $("#global-footer").hide();
  }
  page_stack.push(target_page_path);
  let cur = page_stack.peek();
  page_cover(cur);
}

function pop_page_stack() {
  if (page_stack.isEmpty()) {
    console.error('pop_page_stack error: the stack is already empty');
    return;
  }
  page_stack.pop();
  if (page_stack.isEmpty()) {
    page_refresh(current_page_id);
    // document.getElementById("global-footer").toggleAttribute('hidden');
    $("#global-footer").show();
    checkStatus();
  } else {
    let cur = page_stack.peek();
    page_cover(cur);
  }
}

page_initial();
