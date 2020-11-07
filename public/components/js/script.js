var _current_button = 0;
function switch_button_change(x) {
  if (x != _current_button) {
    if (x == 1) {
      let lb = document.getElementsByClassName('left-button');
      let rb = document.getElementsByClassName('right-button');
      lb[0].classList.remove("enable");
      lb[0].classList.add("disable");
      rb[0].classList.remove("disable");
      rb[0].classList.add("enable");
    } else if (x == 0) {
      let lb = document.getElementsByClassName('left-button');
      let rb = document.getElementsByClassName('right-button');
      lb[0].classList.remove("disable");
      lb[0].classList.add("enable");
      rb[0].classList.remove("enable");
      rb[0].classList.add("disable");
    }
    _current_button = x;
  }
}

var _current_button1 = 0;
function switch_button_change1(x) {
  if (x != _current_button1) {
    if (x == 1) {
      let lb = document.getElementsByClassName('left-button1');
      let rb = document.getElementsByClassName('right-button1');
      lb[0].classList.remove("enable");
      lb[0].classList.add("disable");
      rb[0].classList.remove("disable");
      rb[0].classList.add("enable");
    } else if (x == 0) {
      let lb = document.getElementsByClassName('left-button1');
      let rb = document.getElementsByClassName('right-button1');
      lb[0].classList.remove("disable");
      lb[0].classList.add("enable");
      rb[0].classList.remove("enable");
      rb[0].classList.add("disable");
    }
    _current_button1 = x;
  }
}