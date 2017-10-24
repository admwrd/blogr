
// https://stackoverflow.com/a/18120786/7891095
Element.prototype.remove = function() {
    this.parentElement.removeChild(this);
}
NodeList.prototype.remove = HTMLCollection.prototype.remove = function() {
    for(var i = this.length - 1; i >= 0; i--) {
        if(this[i] && this[i].parentElement) {
            this[i].parentElement.removeChild(this[i]);
        }
    }
}

function confirm_action(text) {
    var agree = confirm(text);
    if (agree)
        return true;
    else
        return false;
}

function add_error(error="") {
    var ins = document.getElementById("mainWrapper");
    var div = document.createElement("div");
    var mp = document.createTextNode(error);
    div.appendChild(mp);
    ins.insertBefore(div, ins.childNodes[0]);
    var newmsg = ins.childNodes[0];
    newmsg.className = "alert alert-danger alert-dismissible fade show form-auth-alert";
    newmsg.setAttribute("role", "alert");
}

function add_error_before(error="", before="") {
    var div = document.createElement("div");
    // div.className = "alert alert-danger alert-dismissible fade show";
    div.className = "alert alert-danger alert-dismissible form-auth-alert";
    div.setAttribute("role", "alert");
    var mp = document.createTextNode(error);
    var btn = document.createElement("button");
    btn.setAttribute("aria-label", "close");
    btn.dataset.dismiss = "alert";
    var span = document.createElement("span");
    span.className = "fa fa-times";
    span.setAttribute("aria-hidden", "true");
    btn.className = "close";
    btn.setAttribute("type", "button");
    // var x = document.createTextNode("&times;");
    // span.appendChild(x);
    btn.appendChild(span);
    div.appendChild(btn);
    div.appendChild(mp);
    var newmsg;
    var ins = document.getElementById("mainWrapper");
    if (before == "") {
        ins.insertBefore(div, ins.childNodes[0]);
        newmsg = ins.childNodes[0];
    } else if (typeof before == "string") {
        var elm = document.getElementById(before);
        ins = elm.parentNode;
        ins.insertBefore(div, elm);
        newmsg = elm.previousSibling;
    }
    // newmsg.childNodes[0].dataset.dismiss = "alert";
    // newmsg.className = "alert alert-danger alert-dismissible fade show";
    // newmsg.setAttribute("role", "alert");
    // newmsg.childNodes[0].setAttribute("aria-label", "close");
}

function add_error_after(error="", after="") {
    var div = document.createElement("div");
    // div.className = "alert alert-danger alert-dismissible fade show";
    div.className = "alert alert-danger alert-dismissible form-auth-alert";
    div.setAttribute("role", "alert");
    var mp = document.createTextNode(error);
    var btn = document.createElement("button");
    btn.setAttribute("aria-label", "close");
    btn.dataset.dismiss = "alert";
    var span = document.createElement("span");
    span.className = "fa fa-times";
    span.setAttribute("aria-hidden", "true");
    btn.className = "close";
    btn.setAttribute("type", "button");
    // var x = document.createTextNode("&times;");
    // span.appendChild(x);
    btn.appendChild(span);
    div.appendChild(btn);
    div.appendChild(mp);
    var ins = document.getElementById("mainWrapper");
    if (after == "") {
        ins.insertBefore(div, ins.childNodes[0]);
    } else if (typeof after == "string") {
        ins = document.getElementById(after);
        ins.appendChild(div);
    }
    // newmsg.childNodes[0].dataset.dismiss = "alert";
    // newmsg.className = "alert alert-danger alert-dismissible fade show";
    // newmsg.setAttribute("role", "alert");
    // newmsg.childNodes[0].setAttribute("aria-label", "close");
}


function add_error_after_sibling(error="", after="") {
    var div = document.createElement("div");
    // div.className = "alert alert-danger alert-dismissible fade show";
    div.className = "alert alert-danger alert-dismissible form-auth-alert";
    div.setAttribute("role", "alert");
    var mp = document.createTextNode(error);
    var btn = document.createElement("button");
    btn.setAttribute("aria-label", "close");
    btn.dataset.dismiss = "alert";
    var span = document.createElement("span");
    span.className = "fa fa-times";
    span.setAttribute("aria-hidden", "true");
    btn.className = "close";
    btn.setAttribute("type", "button");
    // var x = document.createTextNode("&times;");
    // span.appendChild(x);
    btn.appendChild(span);
    div.appendChild(btn);
    div.appendChild(mp);
    var ins = document.getElementById("mainWrapper");
    if (after == "") {
        ins.insertBefore(div, ins.childNodes[0]);
    } else if (typeof after == "string") {
        b4 = document.getElementById(after).nextSibling;
        ins = document.getElementById(after).parentNode;
        ins.insertBefore(div, b4);
    }
    // newmsg.childNodes[0].dataset.dismiss = "alert";
    // newmsg.className = "alert alert-danger alert-dismissible fade show";
    // newmsg.setAttribute("role", "alert");
    // newmsg.childNodes[0].setAttribute("aria-label", "close");
}

function switch_pass() {
    // document.getElementById("passwordField").style.display = "none";
    // document.getElementById("passwordHidden").style.display = "none";
    
    var pass_one = document.getElementById("passwordHidden");
    var pass_two = document.getElementById("passwordField");
    
    pass_one.style.display = "block";
    pass_two.style.display = "none";
}

function disable_submit() {
    var submit = document.getElementById("submit-button-id");
    submit.disabled = true;
}


function validate_form() {
    // var dismiss = document.getElementsByClassName("alert-dismissable").remove();
    var dismiss = document.getElementsByClassName("alert").remove();
    // alert("Counted " + dismiss.length + " dismiss elements");
    // for(var i = 0; i<dismiss.length; i++) {
    //     alert("Removing dismiss " + i);
    //     dismiss[i].remove();
    // }
    // $(".alert-dismissable").remove();
    var user = document.getElementById("usernameField");
    var pass = document.getElementById("passwordField");
    if (user.value != "" && pass.value != "") {
        disable_submit();
        switch_pass();
        pass.value = Sha256.hash(pass.value);
        return true;
    } else {
        if(pass.value == "") {
            // add_error_before("No password was entered.", "passGroup");
            add_error_after("No password was entered.", "passGroup");
        } if(user.value == "") {
            // add_error_before("Please enter your username.", "usernameField");
            // add_error_before("Please enter your username.", "userGroup");
            add_error_after_sibling("Please enter your username.", "usernameField");
        }
        return false;
    }
}

// https://stackoverflow.com/questions/454202/creating-a-textarea-with-auto-resize
// https://stackoverflow.com/a/25621277/7891095
function ChangeHeight() {
    this.style.height = 'auto';
    this.style.height = (this.scrollHeight) + 'px';
}

function StartText() {
    var txt = document.getElementsByTagName('textarea');
    for (var i = 0; i < txt.length; i++) {
        txt[i].setAttribute('style', 'height:' + (txt[i].scrollHeight) + 'px;overflow-y:hidden;');
        txt[i].addEventListener("input", ChangeHeight, false);
    }
}
function StartTextJQuery() {
    $('textarea').each(function () {
      this.setAttribute('style', 'height:' + (this.scrollHeight) + 'px;overflow-y:hidden;');
    }).on('input', function () {
      this.style.height = 'auto';
      this.style.height = (this.scrollHeight) + 'px';
    });
}
