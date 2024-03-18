function switchcolors(event) {
    if (event.target.checked) { // if true
        document.body.className = "dark-mode";
    }
    else { // if false
        document.body.className = "light-mode";
    }
}