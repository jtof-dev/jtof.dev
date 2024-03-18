function initial_setup() {
    if (window.localStorage.getItem("dark") === null) {
        window.localStorage.setItem("dark", true)
    }
    if (window.localStorage.getItem("dark") === "true") {
        document.body.className = "dark-mode"
        document.getElementById("checkbox").checked = 1
    }
    else {
        document.body.className = "light-mode"
        document.getElementById("checkbox").checked = 0
    }
    if (window.localStorage.getItem("notes") === null) {
        window.localStorage.setItem("notes", false)
    }
    if (window.localStorage.getItem("notes") === "true") {
        document.querySelectorAll("span.notes").forEach(Element => Element.className = "notes")
        document.getElementById("notes-button").innerHTML = "show my notes"
    }
    else {
        document.querySelectorAll("span.notes").forEach(Element => Element.className = "notes hide-notes")
        document.getElementById("notes-button").innerHTML = "hide my notes"
    }
}

function switchcolors(event) {
    if (event.target.checked) { // if true
        document.body.className = "dark-mode"
        window.localStorage.setItem("dark", true)
    }
    else { // if false
        document.body.className = "light-mode"
        window.localStorage.setItem("dark", false)
    }
}

function show_hide_notes() {
    let notes = window.localStorage.getItem("notes")
    window.localStorage.setItem("notes", notes === "true" ?"false":"true")
    let updated_notes = notes === "true" ?"false":"true"
    if (updated_notes === "true") {
        document.querySelectorAll("span.notes").forEach(Element => Element.className = "notes")
        document.getElementById("notes-button").innerHTML = "show my notes"
    }
    else {
        document.querySelectorAll("span.notes").forEach(Element => Element.className = "notes hide-notes")
        document.getElementById("notes-button").innerHTML = "hide my notes"
    }
}

document.addEventListener("DOMContentLoaded", initial_setup)