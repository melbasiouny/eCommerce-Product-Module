document.addEventListener("DOMContentLoaded", function () {
    const category_button = document.getElementById("category_button");
    const search_button = document.getElementById("search_button");
    const search_status = document.getElementById("search_status");
    const search_query = document.getElementById("search_query");
    const query_string = window.location.search;
    const url_params = new URLSearchParams(query_string);
    const category = url_params.get('category');
    const query = url_params.get('query');

    if (category == "") {
        category_button.textContent = "All";
    } else {
        category_button.textContent = category;
    }

    function query_products(query) {
        return fetch(`http://172.105.25.146:8080/api/product?category=${category}&search=${query}`)
            .then(response => response.json())
            .catch(error => {
                console.error("Error fetching products: ", error);
                return [];
            });
    }

    function display_products(query) {
        query_products(query)
            .then(products => {
                product_container.innerHTML = "";

                products.forEach(product => {
                    const productHtml =
                        `<div class="product rounded" style="margin-bottom: 40px; margin-left: 10px; margin-right: 10px; margin-top: 10px; width: 180px; height: auto; position: relative;" data-product-id="${product.pid}" background-color: #3498db; color: #fff; text-align: center; line-height: 200px; cursor: pointer; transition: box-shadow 0.3s;" onmouseover="this.style.boxShadow='0 0 8px #005052';" onmouseout="this.style.boxShadow='none';">
                            <img class="rounded" style="object-fit: contain; margin-bottom: 0px;" src="${product.image}" width="180" height="120">
                            <div class="rating" style="position: absolute; top: 0px; right: 6px;">
                                <span style="color: black; font-weight: bold; font-size: 14px;">${product.rating}</span>
                                <span style="color: black; font-size: 18px;">⭐</span>
                            </div>
                            <div class="text-uppercase fw-bold product-name" style="overflow-wrap: break-word;">${product.name}</div>
                            <div class="product-price">C$ ${product.price}</div>
                        </div>`;

                    product_container.innerHTML += productHtml;
                });

                if (products.length > 0) {
                    if (query == "") {
                        if (category == "") {
                            search_status.textContent = "Showing all results";
                        } else {
                            search_status.textContent = "Showing all results in " + category.toLowerCase();
                        }
                    } else {
                        search_status.textContent = "Showing results for \"" + query + "\"" + " in " + category.toLowerCase();
                    }
                } else {
                    search_status.textContent = "No products found for \"" + query + "\"";
                }

                product_container.addEventListener('click', function (event) {
                    const product_element = event.target.closest('.product');
                    if (product_element) {
                        const product_ID = product_element.getAttribute('data-product-id');
                        go_to_product(product_ID);
                    }
                });
            });
    }

    function go_to_product(product_id) {
        const request_options = {
            method: 'POST',
        };

        fetch(`http://172.105.25.146:8080/api/analytics/${product_id}/clicks/increment`, request_options)
            .then(response => {
                if (!response.ok) {
                    console.error('Error incrementing clicks for pid:', product_id);
                }
            })
            .catch(error => {
                console.error('Error while sending POST request:', error);
            });

        window.location.href = 'detailed-view.html?product=' + encodeURIComponent(product_id);
    }

    search_button.addEventListener("click", (event) => {
        event.preventDefault();

        if (category_button.textContent == "All") {
            window.location.href = 'product-search.html?category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
        } else {
            window.location.href = 'product-search.html?category=' + encodeURIComponent(category_button.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    display_products(query);
});