import confetti from "canvas-confetti";

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
            if (search_query.value == "") {
                window.location.href = 'index.html';
            } else {
                window.location.href = 'product-search.html?category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
            }
        } else {
            window.location.href = 'product-search.html?category=' + encodeURIComponent(category.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    /*
    add_to_cart_button.addEventListener("click", (event) => {
        event.preventDefault();
        add_to_cart_button.classList.add('heartBeatAnimation');
        setTimeout(function () {
            add_to_cart_button.classList.remove('heartBeatAnimation');
        }, 500);
    });
    */

    wishlist_button.addEventListener("click", (event) => {
        event.preventDefault();
        wishlist_button.classList.add('heartBeatAnimation');
        setTimeout(function () {
            wishlist_button.classList.remove('heartBeatAnimation');
        }, 500);

        const rect = wishlist_button.getBoundingClientRect();
        const x = rect.left + rect.width / 2;
        const y = rect.top + rect.height / 2;

        var scalar = 2;
        var heart = confetti.shapeFromText({ text: '❤️', scalar });

        var defaults = {
            spread: 360,
            ticks: 32,
            gravity: 0,
            decay: 0.7,
            startVelocity: 32,
            shapes: [heart],
            scalar
        };

        confetti({
            shapes: [heart],
            particleCount: 32,
            origin: { x: x / window.innerWidth, y: y / window.innerHeight },
            ...defaults,
        });
    });

    const cartButtons = document.querySelectorAll('.cart-button');

    cartButtons.forEach(button => {
        button.addEventListener('click', cartClick);

    });

    function cartClick() {
        let button = this;
        button.classList.add('clicked');
    }

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