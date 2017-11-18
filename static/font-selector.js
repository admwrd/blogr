
/* KEYS
    l or h (lowercase L or h)
        next logo font
    
    t
        next title font
    
    b or n
        next body font
    
    c
        show current fonts
        
    a
        increment all fonts
        
    r
        reset all fonts
    
    q or i
        reset all fonts back to initial values
    
*/

var title_fonts = [
    "arev",
    "mesmerizeseul",
    "oldrepublic",
    "progress",
    "roundedelegance",
    "wolf",
    "arev",
    "mesmerizeseul",
    "oldrepublic",
    "progress",
    "roundedelegance",
    "wolf",
    "opensanscond",
    "opensanslight",
    "opensansregular",
    "aliquam",
    "aliquamregular",
    "oldrepublic",
    "opensansbold",
    "opensansregular",
    "raleway",
    "cabin",
    "essence",
    "essencebold",
    "exo2regular",
    "halfmarks",
    "mesmerizesb",
    "mesmerizese",
    "opensansbold"
];
// title_fonts[-1] = "vishusBlogTitle";
title_fonts[-1] = "Tahoma, Verdana, Geneva";

var logo_fonts = [
    "automatica",
    "commodore",
    "convoy",
    "cyberspace",
    "Demonized",
    "engebrechtre",
    "expansiva",
    "freeagent",
    "grammara",
    "groovy",
    "mesmerize",
    "mrb",
    "Quicksand_Book",
    "strikelord"
];
logo_fonts[-1] = "vishusBlog";

var body_fonts = [
    "aspergit_bold",
    "aspergit_light",
    "essence",
    "exo2",
    "gravitybook",
    "gravitylight",
    "halfmarks",
    "hurufo",
    "mesmerizesb",
    "mesmersizese",
    "opensanscond",
    "opensanslight",
    "opensansregular",
    "raleway",
    "timeburner",
    "cabin",
    "arvo",
    
    "aliquam",
    "aliquamregular",
    "oldrepublic",
    "opensansbold",
    "opensansregular",
    "raleway",
    "cabin",
    "essence",
    "essencebold",
    "exo2regular",
    "halfmarks",
    "mesmerizesb",
    "mesmerizese",
    "opensansbold"
];
body_fonts[-1] = "vishusBlogText";

// var cur_body = body_fonts[0];
// var last_font = body_fonts[body_fonts.length-1];
var cur_body = -1;
var last_body_font = body_fonts.length-1;

var cur_logo = -1;
var last_logo_font = logo_fonts.length-1;

var cur_title = -1;
var last_title_font = title_fonts.length-1;

var cur_target = 'all';

/* maybe add:
    increment_body()
    increment_logo()
    increment_title()
    set_body()
    set_logo()
    set_title()
    reset_fonts()
    
*/


function inc_body() {
    if(cur_body != last_body_font) {
        cur_body += 1;
    } else {
        cur_body = 0;
    }
}
function inc_logo() {
    if(cur_logo != last_logo_font) {
        cur_logo += 1;
    } else {
        cur_logo = 0;
    }
}
function inc_title() {
    if(cur_title != last_title_font) {
        cur_title += 1;
    } else {
        cur_title = 0;
    }
}
function set_body() {
    document.getElementById("v-body").style.fontFamily = body_fonts[cur_body];
}
function set_logo() {
    // document.getElementById("header").style.fontFamily = logo_fonts[cur_logo];
    $('.blog-logo').css('font-family', logo_fonts[cur_logo]);
}
function set_title() {
    $('.v-article-title a').css('font-family', title_fonts[cur_title]);
}
function reset_fonts() {
    cur_body = 0;
    cur_logo = 0;
    cur_title = 0;
    set_body();
    set_logo();
    set_title();
}
function reset_initial() {
    cur_body = -1;
    cur_logo = -1;
    cur_title = -1;
    $('.v-article-title a').css('font-family', 'Tahoma, Verdana, Geneva');
    $('.blog-logo').css('font-family', 'vishusBlog');
    document.getElementById("v-body").style.fontFamily = 'vishusBlogText';
}
function display_fonts() {
    var body, logo, title;
    if(cur_body != -1) {
        body = body_fonts[cur_body];
    } else {
        body = "initial";
    }
    if(cur_logo != -1) {
        logo = logo_fonts[cur_logo];
    } else {
        logo = "initial";
    }
    if(cur_title != -1) {
        title = title_fonts[cur_title];
    } else {
        title = "initial";
    }
    // alert("The current fonts are:\n\nlogo: " + logo + "\n\ntitle: " + title + "\n\nbody: " + body);
    alert("The current font settings are:\n\ntarget: " + cur_target + "\n\nlogo: " + logo + "\n\ntitle: " + title + "\n\nbody: " + body);
}

function display_settings() {
    document.getElementById('cfg-target').innerHTML = 'Target: ' + cur_target;
    document.getElementById('body-font').innerHTML = 'Body Font: ' + body_fonts[cur_body];
    document.getElementById('logo-font').innerHTML = 'Logo Font: ' + logo_fonts[cur_logo];
    document.getElementById('title-font').innerHTML = 'Title Font: ' + title_fonts[cur_title];
}


function body_add(value) {
    if (value < 0) {
        if (value + cur_body < 0) { cur_body = last_body_font; } else { cur_body = cur_body+value; }
    } else {
        if (value + cur_body > last_body_font) { cur_body = 0; } else { cur_body = cur_body+value; }
    }
    set_body();
}

function logo_add(value) {
    if (value < 0) {
        if (value + cur_logo < 0) { cur_logo = last_logo_font; } else { cur_logo = cur_logo+value; }
    } else {
        if (value + cur_logo > last_logo_font) { cur_logo = 0; } else { cur_logo = cur_logo+value; }
    }
    set_logo();
}

function title_add(value) {
    if (value < 0) {
        if (value + cur_title < 0) { cur_title = last_title_font; } else { cur_title = cur_title+value; }
    } else {
        if (value + cur_title > last_title_font) { cur_title = 0; } else { cur_title = cur_title+value; }
    }
    set_title();
}

function body_reset(initial = false) {
    if (!initial) {
        cur_body = 0
    } else {
        cur_body = -1;
    }
    set_body();
}


function logo_reset(initial = false) {
    if (!initial) {
        cur_logo = 0
    } else {
        cur_logo = -1;
    }
    set_logo();
}


function title_reset(initial = false) {
    if (!initial) {
        cur_title = 0
    } else {
        cur_title = -1;
    }
    set_title();
}

function reset_target(hard = false) {
    if (cur_target == 'all') {
        body_reset(hard);
        logo_reset(hard);
        title_reset(hard);
    } else if (cur_target == 'body') { body_reset(hard); }
      else if (cur_target == 'logo') { logo_reset(hard); }
      else if (cur_target == 'title') { title_reset(hard); }
}


// only works correctly if value is 1 or -1, otherwise when underflowing/overflowing it will go to first/last
// of the array instead of wrapping the exact number of items
function target_select(targ, value) {
    if (targ == 'all') {
        body_add(value);
        logo_add(value);
        title_add(value);
    } else if (targ == 'body') { body_add(value); }
      else if (targ == 'logo') { logo_add(value); }
      else if (targ == 'title') { title_add(value); }
}

function next_target() {
    target_select(cur_target, 1);
}
function prev_target() {
    target_select(cur_target, -1);
}
function set_target(targ) {
    if        (targ == 'all') { cur_target = 'all'; } 
    else if  (targ == 'body') { cur_target = 'body'; }
    else if  (targ == 'logo') { cur_target = 'logo'; }
    else if (targ == 'title') { cur_target = 'title'; }
}

document.addEventListener("keypress", function(event) {
    // enter: 13 - Up: 38 - Down: 40 - Right: 39 - Left: 37
    if (event.key == 'n' || event.which == 39) { 
        next_target();
    } else if (event.key == 'p' || event.which == 37) {
        prev_target();
    } else if (event.which == 38) {
        if (cur_target == 'all') { cur_target = 'body'; }
        else if (cur_target == 'body') { cur_target = 'logo'; }
        else if (cur_target == 'logo') { cur_target = 'title'; }
        else if (cur_target == 'title') { cur_target = 'all'; }
    } else if (event.which == 40) {
        if (cur_target == 'all') { cur_target = 'title'; }
        else if (cur_target == 'body') { cur_target = 'all'; }
        else if (cur_target == 'logo') { cur_target = 'body'; }
        else if (cur_target == 'title') { cur_target = 'logo'; }
    } else if (event.key == 'a') {
        set_target('all');
    } else if (event.key == 'b') {
        set_target('body');
    } else if (event.key == 'l') { 
        set_target('logo');
    } else if (event.key == 't') { 
        set_target('title');
    } else if (event.key == 'c') { 
        display_fonts();
    } else if (event.key == 'r') { 
        reset_target(false);
    } else if (event.key == 'q' || event.key == 'i') { 
        reset_target(true);
    }
    display_settings();
    




    // // Increment All Fonts
    // if (event.key == 'a') { // when user presses the 'a' key 65
    //     inc_body();
    //     inc_logo();
    //     inc_title();
    //     set_body();
    //     set_logo();
    //     set_title();
    // // Increment Logo Fonts
    // } else if (event.key == 'n' || event.key == 'b') { // when user presses the 'n' 78 or 'b' 66 key
    //     inc_body();
    //     set_body();
    // // Increment Logo Fonts
    // } else if (event.key == 'h' ||event.key == 'l') { // when user presses the 'h' 72 or 'L' 76 (lowercase) key
    //     inc_logo();
    //     set_logo();
    // // Increment Title Fonts
    // } else if (event.key == 't') { // when user presses the 't' 84 key
    //     inc_title();
    //     set_title();
    // // Show Current Fonts
    // } else if (event.key == 'c') { // when user presses the 'c' 67 key
    //     display_fonts();
    // } else if (event.key == 'r') { // when user presses the 'r' 87 key
    //     reset_fonts();
    // } else if (event.key == 'q' || event.key == 'i') { // when user presses the 'q' key to quit
    //     reset_initial();
    // }
});


