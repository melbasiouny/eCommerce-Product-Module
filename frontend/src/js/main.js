document.addEventListener("DOMContentLoaded", function () {
    const query_string = window.location.search;
    const product_container = document.getElementById("product_container");
    const previous_button = document.getElementById("previous_button");
    const search_button = document.getElementById("search_button");
    const search_query = document.getElementById("search_query");
    const category = document.getElementById("category_button");
    const page_number = document.getElementById("page_number");
    const next_button = document.getElementById("next_button");
    const url_params = new URLSearchParams(query_string);
    const current_page = parseInt(url_params.get('page')) || 1;

    category.textContent = "All";

    function fetch_products(page) {
        return fetch(`http://172.105.25.146:8080/api/product/view?page=${page}`)
            .then(response => response.json())
            .catch(error => {
                window.location.href = '404.html';
                console.error("Error fetching products: ", error);
                return [];
            });
    }

    product_container.addEventListener('click', function (event) {
        const product_element = event.target.closest('.product');
        if (product_element) {
            const product_ID = product_element.getAttribute('data-product-id');
            go_to_product(product_ID);
        }
    });

    function display_products(page) {
        fetch_products(page)
            .then(products => {
                product_container.innerHTML = "";

                products.forEach(product => {
                    const is_low_stock = product.stock <= 10;
                    const product_html =
                        `<div class="product rounded" style="margin-bottom: 40px; margin-left: 10px; margin-right: 10px; margin-top: 10px; width: 180px; height: auto; position: relative; overflow: hidden;" data-product-id="${product.pid}">
                            <img class="rounded" style="padding: 12px; object-fit: contain; margin-bottom: 0px;" src="${product.image}" width="180" height="120">
                            <div class="status" style="position: absolute; top: 0px; left: 4px;">    
                                ${is_product_selling_fast(product) ? '<p style="color: black;"><i class="bi bi-fire"></i></p>' : ''}
                            </div>
                            <div class="rating" style="position: absolute; top: 0px; right: 4px;">
                                <span style="color: black; font-weight: bold; font-size: 14px;">${product.rating}</span>
                                <i class="bi bi-star-fill"></i>
                            </div>
                            <div class="text-uppercase fw-bold product-name" style="padding-left: 4px; padding-right: 4px; overflow: hidden; display: -webkit-box; -webkit-line-clamp: 2; -webkit-box-orient: vertical; white-space: normal; text-overflow: ellipsis; overflow-wrap: break-word; z-index: 2;">
                                ${is_low_stock ? '<i class="bi bi-exclamation-circle-fill"></i>' : ''}
                                ${product.name}
                            </div>
                            <div class="product-price" style="padding-left: 4px; padding-right: 4px; z-index: 2;">C$ ${product.price}</div>
                        </div>`;

                    const productElement = document.createElement('div');
                    var hover_start_duration = null;
                    productElement.innerHTML = product_html;


                    productElement.addEventListener('mouseenter', function () {
                        this.style.boxShadow = 'inset 0 0 8px 1px rgba(255, 178, 82, 0.6), 0 0 16px 4px rgba(255, 200, 82, 0.3)';
                        this.style.borderRadius = '16px';
                        this.style.transform = 'scale(1.05)';
                        hover_start_duration = Date.now();
                    });

                    productElement.addEventListener('mouseleave', function () {
                        this.style.boxShadow = 'none';
                        this.style.transform = 'scale(1)';

                        const hover_duration = Date.now() - hover_start_duration;
                        analytics_on_mouse_hover(product.pid, hover_duration)
                    });

                    product_container.appendChild(productElement);
                });

                page_number.textContent = current_page.toString();

                if (products.length == 0) {
                    window.location.href = '404.html';
                }

                if (products.length < 16) {
                    next_button.classList.add("disabled");
                } else {
                    next_button.classList.remove("disabled");
                }

                previous_button.classList.toggle("disabled", current_page <= 1);
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

    async function analytics_on_mouse_hover(product_id, hover_duration) {
        try {
            const response = await fetch(
                `https://analysisreportingdatabasemodulegroup1.azurewebsites.net/Group1/DatabaseController/POST/minhnguyen/Connhenbeo1/group1/0`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        "UserView_ID": Math.floor(Math.random() * 1000),
                        "User_ID": "69",
                        "Product_ID": product_id,
                        "Time_Count": hover_duration,
                        "Date_Access": `${new Date().toISOString().slice(0, 19)}`
                    })
                }
            );

            if (!response.ok) {
                console.error('Error, unable to send product hover data to analysis module.', product_id);
            }
        } catch (error) {
            console.error('Error while sending POST request:', error);
        }
    }


    function go_to_product(product_id) {
        const request_options = {
            method: 'POST',
        };

        fetch(`http://172.105.25.146:8080/api/analytics/${product_id}/clicks/increment`, request_options)
            .then(response => {
                if (!response.ok) {
                    console.error('Error incrementing clicks for pid:', product_id);
                } else {
                    window.location.href = 'detailed-view.html?product=' + encodeURIComponent(product_id);
                }
            })
            .catch(error => {
                console.error('Error while sending POST request:', error);
            });
    }

    search_button.addEventListener("click", (event) => {
        event.preventDefault();

        if (category.textContent == "All") {
            if (search_query.value == "") {
                window.location.href = 'index.html?page=1';
            } else {
                window.location.href = 'product-search.html?category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
            }
        } else {
            window.location.href = 'product-search.html?category=' + encodeURIComponent(category.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    previous_button.addEventListener("click", (event) => {
        event.preventDefault();
        if (current_page > 1) {
            window.location.href = 'index.html?page=' + (current_page - 1).toString();
        }
    });

    next_button.addEventListener("click", (event) => {
        event.preventDefault();
        window.location.href = 'index.html?page=' + (current_page + 1).toString();
    });

    display_products(current_page);
});
