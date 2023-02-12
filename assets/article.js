const generateArticleElement = ({id, title, date, domain, url, readTime}) => {
    return `<div class="flex flex-col justify-around gap rounded-lg border border-slate-300 shadow-lg
    space-y-4 px-4 py-2 divide-slate-200 divide-y">
        <h3><a href=./articles/${id}>${title}</a></h3>
        <div class="flex flex-col gap pt-3">
            <div class="flex flex-row flex-wrap gap-x-2">
                <a href="${url}"
                   class="text-sm text-sky-600 dark:text-sky-300 hover:underline text-ellipsis overflow-hidden ...">
                    ${domain}
                </a>
                <span class="text-sm text-slate-700 dark:text-slate-100">|</span>
                <span class="text-sm text-slate-700 dark:text-slate-100">${readTime}</span>
            </div>
            <span class="text-sm text-slate-700 dark:text-slate-100">Added ${date}</span>
        </div>
    </div>`
}

const generatePaginationElement = ({pages, currentPage}, callBack) => {
    let firstPageElement = `<button onclick="${callBack}(1)" class="h-full rounded-md border border-slate-300 flex items-center">
                <svg xmlns="http://www.w3.org/2000/svg"
                     class="scale-90 stroke-sky-600 icon icon-tabler icon-tabler-chevrons-left"
                     width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none"
                     stroke-linecap="round" stroke-linejoin="round">
                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                    <path d="M11 7l-5 5l5 5"></path>
                    <path d="M17 7l-5 5l5 5"></path>
                </svg>
            </button>`;

    let lastPageElement = `<button onclick="${callBack}(${pages.length})" class="h-full rounded-md border border-slate-300 flex items-center">
                                <svg xmlns="http://www.w3.org/2000/svg"
                                     class="scale-90 stroke-sky-600 icon icon-tabler icon-tabler-chevrons-right"
                                     width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none"
                                     stroke-linecap="round" stroke-linejoin="round">
                                    <path stroke="none" d="M0 0h24v24H0z" fill="none"></path>
                                    <path d="M7 7l5 5l-5 5"></path>
                                    <path d="M13 7l5 5l-5 5"></path>
                                </svg>
                            </button>`;

    let pagesElement = pages.map(page => {
        if (page === currentPage) {
            return `<span class="text-center underline align-middle text-lg pb-0.5 pt-1 px-2 rounded-md border border-slate-300">
                ${currentPage}
            </span>`;
        } else if ((page > currentPage - 3) && (page < currentPage + 3)) {
            return `<button class="text-center align-middle text-lg text-sky-600 pb-0.5 pt-1 px-2 rounded-md border border-slate-300"
                onclick="${callBack}(${page})">
                ${page}
            </button>`;
        } else {
            return "";
        }
    }).join('');

    return `${firstPageElement}${pagesElement}${lastPageElement}`
}