const gotToArticle = (id) => {
    window.location.href = "/articles/" +id
}

const search = instantsearch({
    indexName: "articles",
    searchClient: instantMeiliSearch(
        "http://localhost:7700",
        'fMuxK9AIppcbb6H08T]gm>YAz',
        {
            placeholderSearch: false,
            primaryKey: 'id',
            finitePagination: true,
        }
    )
});
search.addWidgets([
    instantsearch.widgets.searchBox({
        container: "#searchbox"
    }),
    instantsearch.widgets.configure({hitsPerPage: 6}),
    instantsearch.widgets.hits({
        container: "#hits",
        templates: {
            item: `
            <div>
            <div class="hit-name" onclick="gotToArticle({{#helpers.highlight}}{ "attribute": "id" }{{/helpers.highlight}})">
                  {{#helpers.highlight}}{ "attribute": "title" }{{/helpers.highlight}}
            </div>
            </div>
          `
        }
    })
]);

search.start();

window.addEventListener('keydown', (event) => {
    const keyName = event.key;
    console.log(event.key);
    let search = document.getElementsByClassName("ais-SearchBox-input");
    let searchInput = search.item(0);
    if (event.ctrlKey && keyName === "k") {
        event.preventDefault();
        searchInput.focus();
    } else if (event.key === "Escape") {
        searchInput.blur();
    } else {
        return true;
    }
});

let searchInput = document.getElementsByClassName("ais-SearchBox-input").item(0);
searchInput.onfocus = () => {
    let searchIcon = document.getElementsByClassName("ais-SearchBox-submit").item(0);
};