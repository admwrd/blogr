
var resizeInput = document.getElementById("inputTitle");



// var chWidth;
// function getChWidth() {
//     var tmp = document.createElement("span");
//     tmp.innerHTML = 'm';
    
//     document.body.appendChild(tmp);
//     var width = tmp.getBoundingClientRect().width;
//     document.body.removeChild(tmp);
//     return width;
// }
// chWidth = getChWidth();


// if (resizeInput) {
//     $("#inputTitle").keyup(function(){
    
//     var container = document.getElementById('v-footer');
    
//     var width;
//     var extra = 10;
//     if(this.value.length>0){
//         // this.style.width = ((this.value.length + 1) * 8) + 'px';
//         // this.style.width = this.value.length + 5 + 'ch';
//         width = this.value.length;
//     }
    
//     if (width < 20) {
//         width = 20;
//     } else if (width+extra > 66) {
//         width = 66-extra;
//     }
    
    
//     var contWidth = container.getBoundingClientRect().width;
//     this.style.width = width + extra + 'ch';
//     if (this.getBoundingClientRect().width + 127 > contWidth) {
//         this.width = contWidth-127;
//     }
    
//     // console.log("The width is: " + this.style.width);
//     // else{
//       // this.style.width = ((this.getAttribute('placeholder').length + 1) * 8) + 'px';
//       // this.style.width = (this.getAttribute('placeholder').length + 5) + 'ch';
//     // }

// });
// }



// https://stackoverflow.com/a/3395975/7891095
if (resizeInput) {
    adjustWidthOfInput();
    resizeInput.onkeyup = adjustWidthOfInput;
}

function getWidthOfInput() {
    if (!resizeInput) return;
    
    var tmp = document.createElement("span");
    
    tmp.className = "input-element tmp-element";
    tmp.innerHTML = resizeInput.value.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');
    
    tmp.style.visibility = 'hidden';
    tmp.style.fontFamily = resizeInput.style.fontFamily;
    tmp.style.fontSize = resizeInput.style.fontSize;
    tmp.style.letterSpacing = resizeInput.style.letterSpacing;
    tmp.style.padding = resizeInput.style.padding;
    tmp.style.paddingLeft = resizeInput.style.paddingLeft;
    tmp.style.paddingRight = resizeInput.style.paddingRight;
    tmp.style.paddingTop = resizeInput.style.paddingTop;
    tmp.style.paddingBottom = resizeInput.style.paddingBottom;
    
    document.body.appendChild(tmp);
    var theWidth = tmp.getBoundingClientRect().width;
    document.body.removeChild(tmp);
    
    return theWidth;
}

function adjustWidthOfInput() {
    if (!resizeInput) return;
    var w = getWidthOfInput();
    var extra = 180;
    var min = 200;
    var max = 735;
    var width;
    // if (w < 300) {
        // width = 300;
    // } else {
        // width = w + 150;
    // }
    
    if (w < min) {
        width = min;
    } else if (w+extra > max) {
        width = max-extra;
    } else {
        width = w;
    }
    
    resizeInput.style.width = width + extra + "px";
    // resizeInput.style.width = getWidthOfInput() + "px";
}






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

var manage = document.getElementById('v-am-list');
if (manage) {
    
    $(function () {
        $('[data-toggle="tooltip"]').tooltip()
    })
}




