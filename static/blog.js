
function confirm_action(text) {
    var agree = confirm(text);
    if (agree)
        return true;
    else
        return false;
}

function validate_form() {
    var user = document.getElementById("usernameField");
    var pass = document.getElementById("passwordField");
    if user.value != "" && pass.value != "" {
        pass.value = Sha256.hash(pass.value);
        return true;
    } else {
        return false;
    }
}