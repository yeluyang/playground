function addMsgInList(l, m) {
    var me = document.createElement("li");
    me.textContent = m;
    l.appendChild(me);
}

// document.addEventListener("mousemove", () => addMsg(document.getElementById("second"), "Event: mousemove"));
document.addEventListener("click", () => addMsgInList(document.getElementById("second"), "Event: click"));

var first = document.getElementById("first");
first.textContent = "Page Ready"
