function initial_setup() {
  if (window.localStorage.getItem("dark") === null) {
    window.localStorage.setItem("dark", true);
  }
  if (window.localStorage.getItem("dark") === "true") {
    document.body.className = "dark-mode";
    document.getElementById("checkbox").checked = 1;
  } else {
    document.body.className = "light-mode";
    document.getElementById("checkbox").checked = 0;
  }
  if (window.localStorage.getItem("notes") === null) {
    window.localStorage.setItem("notes", true);
  }
  if (window.localStorage.getItem("notes") === "true") {
    document
      .querySelectorAll("span.notes")
      .forEach((Element) => (Element.className = "notes"));
    document.getElementById("notes-button").innerHTML = "hide my notes";
  } else {
    document
      .querySelectorAll("span.notes")
      .forEach((Element) => (Element.className = "notes hide-notes"));
    document.getElementById("notes-button").innerHTML = "show my notes";
  }
}

function switchcolors(event) {
  if (event.target.checked) {
    // if true
    document.body.className = "dark-mode";
    window.localStorage.setItem("dark", true);
  } else {
    // if false
    document.body.className = "light-mode";
    window.localStorage.setItem("dark", false);
  }
}

function show_hide_notes() {
  let notes = window.localStorage.getItem("notes");
  window.localStorage.setItem("notes", notes === "true" ? "false" : "true");
  let updated_notes = notes === "true" ? "false" : "true";
  if (updated_notes === "true") {
    document
      .querySelectorAll("span.notes")
      .forEach((Element) => (Element.className = "notes"));
    document.getElementById("notes-button").innerHTML = "hide my notes";
  } else {
    document
      .querySelectorAll("span.notes")
      .forEach((Element) => (Element.className = "notes hide-notes"));
    document.getElementById("notes-button").innerHTML = "show my notes";
  }
}

function search_recipes(event) {
  let links = Array.from(document.querySelectorAll("a.recipe-link"));
  links
    .filter((link) => !link.innerHTML.includes(event.target.value))
    .forEach((Element) => (Element.className = "recipe-link hide-recipe"));
  links
    .filter((link) => link.innerHTML.includes(event.target.value))
    .forEach((Element) => Element.classList.remove("hide-recipe"));
}

document.addEventListener("DOMContentLoaded", initial_setup);

document.addEventListener("DOMContentLoaded", function () {
  let links = Array.from(document.querySelectorAll("a.recipe-link"));

  links.sort((a, b) => a.textContent.localeCompare(b.textContent));

  let sortedHTML = links.map((link) => link.outerHTML).join("");

  document.getElementById("recipe-links").innerHTML = sortedHTML;
});
