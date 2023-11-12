<h1 align="center"> Product Module Specification</h1>
This specification document is a work in progress and may undergo revisions and updates. It is provided for informational and planning purposes only. The information contained within this document is subject to change without notice. The document represents a snapshot of our current understanding and intentions regarding the project's requirements, design, and functionality. Please consider this document as a starting point for discussion and clarification.<br><br>

*Any significant changes to this specification will be communicated and documented accordingly. It is advisable to regularly review this document for updates and to maintain open channels of communication with the product module team to stay informed about any changes or clarifications.*

## Product
The product is defined using the following structure:
```rust
struct Product {
    /// The product ID formatted as (P0000...).
    pid: String,

    /// The seller ID formatted as (S0000...).
    sid: String,

    /// The product name.
    name: String,

    /// The product description.
    description: String,

    /// The product image as a URL.
    image: String,

    /// The product category.
    category: String,

    /// The product price.
    price: f32,

    /// The product stock level.
    stock: u32,

    /// The product total number of sales.
    sales: u32,

    /// The product rating using a five-star system.
    rating: f32,

    /// The product number of clicks.
    clicks: u32,
}

```

## API
The functionality of the product module depends on the following set of APIs. APIs labeled as *(internal)* are provided by us, whereas those labeled as *(external)* are provided by their respective modules and invoked within our own module.

### Product
- **GET** `/api/product/{pid}/data` *(internal)*
    - Retrieves the product data using the specified product ID.
    - Response **(JSON)**:
        ```json
        {
            "pid": "",
            "sid": "",
            "name": "",
            "description": "",
            "image": "",
            "category": "",
            "price": 0.0,
            "stock": 0,
            "sales": 0,
            "rating": 0.0
        }
        ```

- **GET** `/api/product?category=&search=` *(internal)*
    - Retrieves a list of products that match the search query.
    - Response **(JSON)**:
        ```json
        [
            {
                "pid": "",
                "sid": "",
                "name": "",
                "description": "",
                "image": "",
                "category": "",
                "price": 0.0,
                "stock": 0,
                "sales": 0,
                "rating": 0.0,
                "clicks": 0
            },
        ]
        ```

- **GET** `/api/product/view?page=` *(internal)*
    - Retrieves a list of products based on the page number. *The page index starts at 1 and each page can contain a maximum of 16 products.*
    - Response **(JSON)**:
        ```json
        [
            {
                "pid": "",
                "sid": "",
                "name": "",
                "image": "",
                "price": 0.0,
                "stock": 0,
                "sales": 0,
                "rating": 0.0,
                "clicks": 0
            },
        ]
        ```

- **PATCH** `/api/product/{pid}?image=&price=&stock=&sales=&rating=` *(internal)*
    - Updates the specified product.
    - Response **(HTML)** status code.

### Analytics
- **GET** `/api/analytics/{pid}` *(internal)*
    - Retrieves analytical data for the specified product.
    - Response **(JSON)**:
        ```json
        {
            "pid": "",
            "sid": "",
            "stock": 0,
            "sales": 0,
            "rating": 0.0,
            "clicks": 0
        }
        ```

- **POST** `/api/analytics/{pid}/clicks/increment` *(internal)*
    - Increments the number of clicks for the specified product.
    - Response **(HTML)** status code.


### Advertisements
- **GET** `/api/advertisements` *(external)*
    - Retrieves a list of advertisement products by their IDs.
    - Response **(JSON)**:
        ```json
        {
            "ads": [""]
        }
        ```

### Profile
- **GET** `/api/profile/id` *(external)*
    - Retrieves the user ID.
    - Response **(JSON)**:
        ```json
        {
            "id": ""
        }
        ```

- **GET** `/api/profile/{sid}` *(external)*
    - Retrieves the seller information using the specified seller ID.
    - Response **(JSON)**:
        ```json
        {
            "sid": "",
            "name": ""
        }
        ```

- **POST** `/api/profile/seller/add/product?pid=&sid=&name=&description=&image=&category=&price=&stock=` *(internal)*
    - Adds a new product to the product module database.
    - Response **(HTML)** status code.

- **DELETE** `/api/profile/seller/remove/product/{pid}` *(internal)*
    - Removes the specified product from the product module database.
    - Response **(HTML)** status code.

### Cart
- **GET** `/api/cart/products/{id}` *(external)*
    - Retrieves the list of products in the user's cart. *Duplicate product IDs can be used to show different quantities for said product.*
    - Response **(JSON)**:
        ```json
        {
            "id": "",
            "pid": [""]
        }
        ```

- **POST** `/api/cart/add/{id}/{pid}` *(external)*
    - Adds the specified product to the user's cart.
    - Response **(HTML)** status code.

- **DELETE** `/api/cart/remove/{id}/{pid}` *(external)*
    - Removes the specified product from the user's cart.
    - Response **(HTML)** status code.

- **POST** `/api/cart/wishlist/add/{id}/{pid}` *(external)*
    - Adds the specified product to the user's wishlist.
    - Response **(HTML)** status code.

- **DELETE** `/api/cart/wishlist/remove/{id}/{pid}` *(external)*
    - Removes the specified product from the user's wishlist.
    - Response **(HTML)** status code.