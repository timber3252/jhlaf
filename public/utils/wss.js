function intial(){
    ws = new WebSocket("ws://");
    ws.onopen=function(){
        socket.send("connected.");
    }
    ws.onmessage=function(evt){
        
    }
    $("#homepage").on("click",);
}

function request(type)

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
