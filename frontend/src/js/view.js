document.addEventListener('DOMContentLoaded', async function () {
    const add_to_cart_button = document.getElementById("product_add_to_cart");
    const wishlist_button = document.getElementById("product_wishlist");
    const search_button = document.getElementById("search_button");
    const search_query = document.getElementById("search_query");
    const category = document.getElementById("category_button");
    const query_string = window.location.search;
    const url_params = new URLSearchParams(query_string);
    const pid = url_params.get('product');

    if (!pid) {
        console.log('No product ID found in the query parameters.');
    } else {
        try {
            const product = await fetch(`http://172.105.25.146:8080/api/product/${pid}/data`)
                .then(response => response.json());
            display_product_details(product);
        } catch (error) {
            console.error("Error fetching product data: ", error);
        }
    }

    search_button.addEventListener("click", (event) => {
        event.preventDefault();

        if (category.textContent == "All") {
            window.location.href = 'product-search.html?category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
        } else {
            window.location.href = 'product-search.html?category=' + encodeURIComponent(category_button.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    add_to_cart_button.addEventListener("click", (event) => {
        event.preventDefault();
        alert("Added product to cart.");
    });

    wishlist_button.addEventListener("click", (event) => {
        event.preventDefault();
        alert("Added product to wishlist.");
    });

    function display_product_details(product) {
        document.title = product.pid + " | " + product.name;
        category.textContent = product.category;
        document.getElementById("product_image").src = product.image;
        document.getElementById("product_name").textContent = product.name;
        document.getElementById("product_rating").textContent = product.rating;
        document.getElementById("product_sales").textContent = product.sales + " sold";
        document.getElementById("product_description").textContent = product.description;
        document.getElementById("product_price").textContent = "C$ " + product.price;
        document.getElementById("product_sid").textContent = "Seller: " + product.sid;
        document.getElementById("product_stock").textContent = product.stock + " left in stock";
    }
});