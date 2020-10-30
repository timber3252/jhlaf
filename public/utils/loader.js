function page_initial(){
    page_dict = {"homepage" : null, "laf" : null, "me" : null, "login" : null, "publish" : null, "result" : null, };
    back = null;
    current_page_id = "login";
    page_update("login");
}
page_initial();

function page_update(target_page_id) {
    var main = document.getElementById("content");
    var temp_child = main.firstChild;
    if(page_dict[target_page_id] != null){
        main.replaceChild(page_dict[target_page_id],main.firstChild);
        if(target_page_id == "result"){
            page_refresh("result", arguments[1]);
        }
    }
    else{
        var temp = document.createElement("div");
        fetch('pages/'+target_page_id+'.html')
            .then(function(response){response.text().then(function(data){temp.innerHTML = data;});})
        if(main.firstChild != null){
            main.replaceChild(temp.firstChild,main.firstChild);
        }
        else{
            main.prepend(temp.firstChild);
        }
    }
    page_dict[current_page_id] = temp_child;
    if(current_page_id != target_page_id){
        page_dict["back"] = temp_child;
    }
    current_page_id = target_page_id;
}


function page_refresh(target_page_id){
    switch(target_page_id){
        case "homepage" : {
            
            break;
        }
        case "laf" : {

            break;
        }
        case "me" : {

            break;
        }
        case "login" : {
            
            break;
        }
        case "publish" : {
            
            break;
        }
        case "result" : {
            ws_send()
            break;
        }
    }
}


function toggleReg() {
    document.getElementById("login_box").hidden = true;
    document.getElementById("register_box").hidden = false;
}
  
function toggleLogin() {
    document.getElementById("register_box").hidden = true;
    document.getElementById("login_box").hidden = false;
}

function login() {
    username = document.getElementById("username1").value;
    password = document.getElementById("password1").value;
    ws_send("login", )
}
//$('.post').on('click',page_update('result'))

