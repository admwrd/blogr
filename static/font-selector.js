
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
        reset all fonts to initial values
    
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

// var cur_body = body_fonts[0];
// var last_font = body_fonts[body_fonts.length-1];
var cur_body = -1;
var last_body_font = body_fonts.length-1;

var cur_logo = -1;
var last_logo_font = logo_fonts.length-1;

var cur_title = -1;
var last_title_font = title_fonts.length-1;

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
    alert("The current fonts are:\n\nlogo: " + logo + "\n\ntitle: " + title + "\n\nbody: " + body);
}

document.addEventListener("keypress", function(event) {
    // Increment All Fonts
    if (event.key == 'a') { // when user presses the 'a' key 65
        inc_body();
        inc_logo();
        inc_title();
        set_body();
        set_logo();
        set_title();
    // Increment Logo Fonts
    } else if (event.key == 'n' || event.key == 'b') { // when user presses the 'n' 78 or 'b' 66 key
        inc_body();
        set_body();
    // Increment Logo Fonts
    } else if (event.key == 'h' ||event.key == 'l') { // when user presses the 'h' 72 or 'L' 76 (lowercase) key
        inc_logo();
        set_logo();
    // Increment Title Fonts
    } else if (event.key == 't') { // when user presses the 't' 84 key
        inc_title();
        set_title();
    // Show Current Fonts
    } else if (event.key == 'c') { // when user presses the 'c' 67 key
        display_fonts();
    } else if (event.key == 'r') { // when user presses the 'r' 87 key
        reset_fonts();
    } else if (event.key == 'q' || event.key == 'i') { // when user presses the 'q' key to quit
        reset_initial();
    }
});


