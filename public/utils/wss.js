function intial(){
    socket.onopen=function(){
        socket.send("connected.");
    }
    socket.onmessage=function(evt){
        homepage=evt.data;
        var body=document.getElementById("content");
        body.appendChild(homepage);
    }
    $("#homepage").on("click",);
}
function page_changed(page_name) {

}

function page_refresh(page_name){
    
}




function t2(){
    var socket=new WebSocket("ws://");
    socket.onopen=function(){
        
    }
    socket.onmessage=function(evt){
        var json_obj=JSON.parse(evt.data);
        switch(json_obj.type){
            case "lost_post":add_lost_post(evt.data);
            break;
            case "found_post":add_found_post(evt.data);
            break;
        }
    }
}
