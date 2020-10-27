function page_initial(){
    page_dict = {"homepage":null,"laf":null,"me":null,"login":null,"publish":null,"result":null,"back":null};
    current_page_id = "homepage";
}

function page_update(target_page_id) {
    var main = document.getElementById("main_content");
    page_dict[current_page_id] = main.firstChild;
    if(page_dict[target_page_id] != null){
        main.replaceChild(page_dict[target_page_id],main.firstChild);
    }
    else{
        var temp = document.createElement("div");
        fetch("********************************/"+target_page_id+".html")
            .then(function(response){response.text().then(function(data){temp.innerHTML = data;});})
        if(main.firstChild != null){
            main.replaceChild(temp.firstChild,main.firstChild);
        }
        else{
            main.prepend(temp.firstChild);
        }
        page_refresh(target_page_id);
    }
    page_dict["back"] = current_page_id;
    current_page_id = target_page_id;
}

function page_refresh(target_page_id){
    
}

