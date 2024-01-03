const FPS = 10;

setInterval(function() {
    $.get("last",
        function (data) {
            $(".frame").attr("src", "data:image/png;base64,"+data);
        }
    );
}, 1000/FPS);
