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
    const uid = url_params.get('uid') || null;
    
    let selectedProduct = null;

    if (uid == null)
    {
        window.location.href = '404.html';
        console.log("Invalid uid.");
    }

    const anchorElement = document.getElementById("Index");
    anchorElement.href = `index.html?uid=${uid}&page=1`;

    const CartButton = document.getElementById("CartButton");
    CartButton.href = `https://shoppingcart.honeybeeks.net/${uid}`;

    const WishlistButton = document.getElementById("WishlistButton");
    WishlistButton.href = `https://shoppingcart.honeybeeks.net/wishlist/${uid}`;

    if (!pid) {
        console.log('No product ID found in the query parameters.');
    } else {
        try {
            const product = await fetch(`http://172.105.25.146:8080/api/product/${pid}/data`)
                .then(response => response.json());

            selectedProduct = product;
            display_product_details(product);
        } catch (error) {
            window.location.href = '404.html';
            console.error("Error fetching product data: ", error);
        }
    }

    search_button.addEventListener("click", (event) => {
        event.preventDefault();

        if (category.textContent == "All") {
            if (search_query.value == "") {
                window.location.href = 'index.html?uid=' + uid + '&page=1';
            } else {
                window.location.href = 'product-search.html?uid=' + uid + '&category=' + encodeURIComponent('') + '&query=' + encodeURIComponent(search_query.value);
            }
        } else {
            window.location.href = 'product-search.html?uid=' + uid + '&category=' + encodeURIComponent(category.textContent) + '&query=' + encodeURIComponent(search_query.value);
        }
    });

    add_to_cart_button.addEventListener("click", (event) => {
        event.preventDefault();
    });

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

        // TODO: Add to wishlist.
        try {
            const response = fetch(
                `http://172.105.25.146:8080/api/frontend/addtowishlist/${uid}`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        "id": selectedProduct.pid,
                        "sellerid": selectedProduct.sid,
                        "name": selectedProduct.name,
                        "description": selectedProduct.description,
                        "imgurl": selectedProduct.image,
                        "cost": selectedProduct.price
                    })
                }
            );

            if (!response.ok) {
                // console.error('Error, unable to add product to wishlist.');
            }
        } catch (error) {
            console.error('Error while sending POST request:', error);
        }
    });

    const cartButtons = document.querySelectorAll('.cart-button');

    cartButtons.forEach(button => {
        button.addEventListener('click', cartClick);

    });

    function cartClick() {
        let button = this;
        button.classList.add('clicked');

        // TODO: Add to cart.
        try {
            const response = fetch(
                `http://172.105.25.146:8080/api/frontend/addtocart/${uid}`,
                {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify({
                        "id": selectedProduct.pid,
                        "sellerid": selectedProduct.sid,
                        "name": selectedProduct.name,
                        "description": selectedProduct.description,
                        "imgurl": selectedProduct.image,
                        "cost": selectedProduct.price
                    })
                }
            );

            if (!response.ok) {
                // console.error('Error, unable to add product to cart.');
            }
        } catch (error) {
            console.error('Error while sending POST request:', error);
        }
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

    function display_product_details(product) {
        document.title = product.pid + " | " + product.name;
        category.textContent = product.category;
        document.getElementById("product_image").src = product.image;
        document.getElementById("product_name").textContent = product.name;
        document.getElementById("product_rating").textContent = product.rating;
        document.getElementById("product_sales").textContent = product.sales + " sold";
        document.getElementById("product_description").textContent = product.description;
        document.getElementById("product_price").textContent = "C$ " + product.price;
        document.getElementById("product_category").textContent = product.category;
        document.getElementById("product_sid").textContent = "Seller: " + product.sid;
        document.getElementById("product_stock").textContent = product.stock + " left in stock";

        if (product.stock <= 10) {
            document.getElementById("product_stock").classList.add("low-stock");
        }

        if (!is_product_selling_fast(product)) {
            document.getElementById("product_status").remove();
        }

        if (product.price <= 0 || product.stock <= 0) {
            add_to_cart_button.classList.add("gray-out");
            add_to_cart_button.classList.remove("cart-button");

            add_to_cart_button.querySelector(".bi-cart-fill").remove();
            add_to_cart_button.querySelector(".bi-box2-fill").remove();
            add_to_cart_button.querySelector(".add-to-cart").remove();
        }
    }
});