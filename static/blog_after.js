

// var tag_form = document.forms.insert_form.elements["tags"];
var tag_form = document.getElementById("insert-tags");
if (tag_form) {
    tag_form.addEventListener('input', function(event) {
        checkTags();
    }, false);
    tag_form.addEventListener('change', function(event) {
        checkTags();
    }, false);
    tag_form.addEventListener('mouseout', function(event) {
        checkTags();
    }, false);
}

