function swapDiv(e1, e2){
    var e1p = e1.parentNode;
    var e2p = e2.parentNode;
    var e2s = e2.nextSibling;

    e1p.replaceChild(e2, e1);
    if (e2s) {
        e2p.insertBefore(e1, e2s);
    } else {
        e2p.appendChild(e1);
    }
}

function shuffle() {
    var friends = document.querySelectorAll(".friend");
    for (var i = 0; i < friends.length; i++) {
        var r = Math.floor(Math.random() * (i + 1));
        swapDiv(friends[r], friends[i]);
    }
}

shuffle();
