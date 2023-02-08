const gotToArticle = (id) => {
    window.location.href = "/articles/" +id
}

const search = instantsearch({
    indexName: "articles",
    searchClient: instantMeiliSearch(
        "http://localhost:7700",
        'fMuxK9AIppcbb6H08T]gm>YAz',
        {
            primaryKey: 'id',
            finitePagination: true,
        }
    )
});
search.addWidgets([
    instantsearch.widgets.searchBox({
        container: "#searchbox",
        autofocus: true
    }),
    instantsearch.widgets.configure({
            hitsPerPage: 8,
            attributesToSnippet: ['text:50'],
        }),
    instantsearch.widgets.hits({
        container: "#hits",
        templates: {
            item: `
            <div class="hit-item" onclick="gotToArticle({{#helpers.highlight}}{ "attribute": "id" }{{/helpers.highlight}})">
                <div class="hit-title"> 
                      {{#helpers.highlight}}{ "attribute": "title" }{{/helpers.highlight}}
                </div>
                    <p class="hit-text">{{#helpers.snippet}}{ "attribute": "text" }{{/helpers.snippet}}</p>
            </div>
          `
        }
    })
]);

search.start();