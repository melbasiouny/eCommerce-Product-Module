document.addEventListener("DOMContentLoaded", function () {
    const product_container = document.getElementById("product_container");
    const previous_button = document.getElementById("previous_button");
    const search_button = document.getElementById("search_button");
    const search_query = document.getElementById("search_query");
    const category = document.getElementById("category_button");
    const page_number = document.getElementById("page_number");
    const next_button = document.getElementById("next_button");

    let current_page = 1;
    category.textContent = "All";

    function fetch_products(page) {
        return fetch(`http://172.105.25.146:8080/api/product/view?page=${page}`)
            .then(response => response.json())
            .catch(error => {
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
                    const productHtml =
                        `<div class="product rounded" style="margin-bottom: 40px; margin-left: 10px; margin-right: 10px; margin-top: 10px; width: 180px; height: auto; position: relative;" data-product-id="${product.pid}" background-color: #3498db; color: #fff; text-align: center; line-height: 200px; cursor: pointer; transition: box-shadow 0.3s;" onmouseover="this.style.boxShadow='0 0 8px #005052';" onmouseout="this.style.boxShadow='none';">
                            <img class="rounded" style="object-fit: contain; margin-bottom: 0px;" src="${product.image}" width="180" height="120">
                            <div class="rating" style="position: absolute; top: 0px; right: 6px;">
                                <span style="color: black; font-weight: bold; font-size: 14px;">${product.rating}</span>
                                <i class="bi bi-star-fill"></i>
                            </div>
                            <div class="text-uppercase fw-bold product-name" style="overflow-wrap: break-word;">${product.name}</div>
                            <div class="product-price">C$ ${product.price}</div>
                        </div>`;

                    product_container.innerHTML += productHtml;
                });

                page_number.textContent = current_page.toString();

                if (products.length < 16) {
                    next_button.classList.add("disabled");
                } else {
                    next_button.classList.remove("disabled");
                }

                previous_button.classList.toggle("disabled", current_page <= 1);
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
                window.location.href = 'index.html';
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
            current_page--;
            display_products(current_page);
        }
    });

    next_button.addEventListener("click", (event) => {
        event.preventDefault();
        console.log("click");
        current_page++;
        display_products(current_page);
    });

    display_products(current_page);
});
