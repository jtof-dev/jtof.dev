# jtof.dev
- welcome to the github repo for [jtof.dev](https://jtof.dev)!
- `jtof.dev` is powered by github pages through a handful of domain records (you can find the documentation for this [here](https://docs.github.com/en/pages/configuring-a-custom-domain-for-your-github-pages-site/about-custom-domains-and-github-pages)):

| type  | domain name                               | content                          |
|-------|-------------------------------------------|----------------------------------|
| A     | jtof.dev                                  | 185.199.108.153                  |
| A     | jtof.dev                                  | 185.199.109.153                  |
| A     | jtof.dev                                  | 185.199.110.153                  |
| A     | jtof.dev                                  | 185.199.111.153                  |
| AAAA  | jtof.dev                                  | 2606:50c0:8000::153              |
| AAAA  | jtof.dev                                  | 2606:50c0:8001::153              |
| AAAA  | jtof.dev                                  | 2606:50c0:8002::153              |
| AAAA  | jtof.dev                                  | 2606:50c0:8003::153              |
| ANAME | jtof.dev                                  | jjtofflemire.github.io           |
| TXT   | `challenge subdomain`.jtof.dev            | `verification code`              |
| CNAME | www.jtof.dev                              | jjtofflemire.github.io           |

- as well as a `CNAME` file containing the `jtof.dev` url

## jtof.dev/cookbook
- I store all of my recipes in obsidian, in a standard markdown format
- I convert all my recipes to `html` pages with all the custom formatting using `src/main.rs`
  - this script can be called with `cargo run` from the `cookbook/` folder
  - this script also gets [automatically called](.git/hooks/pre-commit) using `git hook`'s

### jtof.dev/cookbook/src
- this folder holds the `main.rs` script, and the `.html` files that the script pulls in
  - the html headers and footers are used to cobble together html pages for each recipe

## light and dark mode
- light and dark mode is controlled by a `localStorage` javascript variable which is checked at load time
  - light and dark mode respectively set a `light-mode` or `dark-mode` class on the `<body>`
  - all the color changes are handled by css snippets that check this body class:

```
body.light/dark-mode class {
    css
}
```

## mobile view
- a section of `styles.css` checks the screen width, and applies css changes when there are less than a certain number of pixels:

```
@media screen and (max-width: {number}px) {
    class {
        css
    }
}
```

# to do
- [ ] add a tertiary color for `h3` and smaller headers
- [ ] clean up all `css` files, so that they are reordered in a more straightforward way
  - I will probably use a pastel pink for dark mode, and a pink from [the gruvbox light theme](https://github.com/morhetz/gruvbox)
- [ ] add a download button to the header of the recipes
      - now the header will probably look like: `home button`, `theme toggle`, a gap, `download button`, and `notes button` on one line, and the title of the recipe on the next line
      - the download should link to a `recipe.md` that has the `tags` section removed (probably in a `recipes/` folder inside `cookbook`)
- [ ] update `main.rs` to add `target="_blank"` to the `source` links at the bottom of recipes (if possible)
- [ ] bugfix `localStorage` not always remembering correctly on page back and forward
