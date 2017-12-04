


// var searchform = document.getElementById('search-form').element["q"].style.display = 'none';


var searchfield = document.getElementById('search-form').q;
var searchbtn = document.getElementById('search-form').lastElementChild;
if (searchfield && searchbtn) {
    hide_search();
    searchfield.addEventListener('change', function() { show_search(); });
    searchfield.addEventListener('focus', function() { show_search(); });
    searchfield.addEventListener('blur', function() { hide_search(); });
} else {
    console.log("No search field or button");
}

function hide_search() {
    // commented out code to make the search button stay visible if
    //     there is text in the search field
    // if (searchfield.value == "") {
        searchbtn.style.display = 'none';
    // } else {
        // searchbtn.style.display = 'block';
    // }
}

function show_search() {
    searchbtn.style.display = 'block';
}


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

