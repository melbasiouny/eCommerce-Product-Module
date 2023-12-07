document.addEventListener("DOMContentLoaded", function () {
    const category_button = document.getElementById("category_button");
    const search_button = document.getElementById("search_button");
    const search_status = document.getElementById("search_status");
    const search_query = document.getElementById("search_query");
    const query_string = window.location.search;
    const url_params = new URLSearchParams(query_string);
    const category = url_params.get('category');
    const query = url_params.get('query');
    const uid = url_params.get('uid') || null;

    if (uid == null)
    {
        window.location.href = '404.html';
        console.log("Invalid uid.");
    }

    const anchorElement = document.getElementById("Index");
    anchorElement.href = `index.html?uid=${uid}&page=1`;

    const CartButton = document.getElementById("CartButton");
    CartButton.href = `https://shoppingcart.honeybeeks.net/api/${uid}`;

    const WishlistButton = document.getElementById("WishlistButton");
    WishlistButton.href = `https://shoppingcart.honeybeeks.net/api/wishlist/${uid}`;

    if (category == "") {
        category_button.textContent = "All";
    } else {
        category_button.textContent = category;
    }

    function query_products(query) {
        return fetch(`http://172.105.25.146:8080/api/product?category=${category}&search=${query}`)
            .then(response => response.json())
            .catch(error => {
                window.location.href = '404.html';
                console.error("Error fetching products: ", error);
                return [];
            });
    }

    function display_products(query) {
        query_products(query)
            .then(products => {
                product_container.innerHTML = "";

                products.forEach(product => {
                    const is_low_stock = product.stock <= 10;
                    const product_html =
                        `<div class="product rounded" style="margin-bottom: 40px; margin-left: 10px; margin-right: 10px; margin-top: 10px; width: 180px; height: auto; position: relative; overflow: hidden;" data-product-id="${product.pid}">
                            <img class="rounded" style="padding: 12px; object-fit: contain; margin-bottom: 0px;" src="${product.image}" width="180" height="120">
                            <div class="status" style="position: absolute; top: 0px; left: 4px;">    
                                ${is_product_selling_fast(product) ? '<p style="color: black;"><i class="bi bi-fire" style="color: darkorange"></i></p>' : ''}
                            </div>
                            <div class="rating" style="position: absolute; top: 0px; right: 4px;">
                                <span style="color: black; font-weight: bold; font-size: 14px;">${product.rating}</span>
                                <i class="bi bi-star-fill" style="color: gold"></i>
                            </div>
                            <div class="text-uppercase fw-bold product-name" style="padding-left: 4px; padding-right: 4px; overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; white-space: normal; text-overflow: ellipsis; overflow-wrap: break-word; z-index: 2;">
                                ${is_low_stock ? '<i class="bi bi-exclamation-circle-fill" style="color: darkred"></i>' : ''}
                                ${product.name}
                            </div>
                            <div class="product-price" style="padding-left: 4px; padding-right: 4px; z-index: 2;">C$ ${product.price}</div>
                        </div>`;

                    const productElement = document.createElement('div');
                    productElement.innerHTML = product_html;


                    productElement.addEventListener('mouseenter', function () {
                        this.style.boxShadow = 'inset 0 0 8px 1px rgba(255, 178, 82, 0.6), 0 0 16px 4px rgba(255, 200, 82, 0.3)';
                        this.style.borderRadius = '16px';
                        this.style.transform = 'scale(1.05)';
                    });

                    productElement.addEventListener('mouseleave', function () {
                        this.style.boxShadow = 'none';
                        this.style.transform = 'scale(1)';
                    });

                    product_container.appendChild(productElement);
                });

                if (products.length > 0) {
                    if (query == "") {
                        search_status.textContent = "Showing all results in " + category.toLowerCase();
                    } else {
                        if (category == "") {
                            search_status.textContent = "Showing all results for \"" + query + "\"";
                        } else {
                            search_status.textContent = "Showing results for \"" + query + "\"" + " in " + category.toLowerCase();
                        }
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

    function is_product_selling_fast(product) {
        const RATING_THRESHOLD = 4.5;
        const STOCK_THRESHOLD = 1;
        const SALES_THRESHOLD = 100;
        const CLICKS_THRESHOLD = product.sales * .2;

        return (
            product.rating >= RATING_THRESHOLD &&
            product.stock >= STOCK_THRESHOLD &&
            product.sales >= SALES_THRESHOLD &&
            product.clicks >= CLICKS_THRESHOLD
        );
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

        window.location.href = 'detailed-view.html?uid=' + uid + '&product=' + encodeURIComponent(product_id);
    }

    search_button.addEventListener("click", (event) => {
        event.preventDefault();

        if (category_button.textContent == "All") {
            if (search_query.value == "") {
                window.location.href = 'index.html?uid=' + uid + '&page=1';
            } else {
                window.location.href = 'product-search.html?uid=' + uid + '&category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
            }
        } else {
            window.location.href = 'product-search.html?uid=' + uid + '&category=' + encodeURIComponent(category_button.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    display_products(query);


});