(function () {
    const container = document.querySelector(".index");
    if (!container) return;

    const links = Array.from(container.querySelectorAll("a"));

    let parentLink = null;
    const others = [];

    for (const a of links) {
        if (a.getAttribute("href") === "..") {
            parentLink = a;
        } else {
            others.push(a);
        }
    }

    // sort alphabetically by visible text
    others.sort((a, b) => {
        return a.textContent.localeCompare(b.textContent);
    });

    // clear container
    container.innerHTML = "";

    // only add ".." if not root
    if (parentLink && window.location.pathname !== "/") {
        container.appendChild(parentLink);
    }

    // append sorted items
    for (const a of others) {
        container.appendChild(a);
    }
})();
