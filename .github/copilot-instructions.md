# Invisible Fun

This project is a Rust-based web server that serves a dynamically generated image.

## How it works

The core of the project is a web server built with the `tiny_http` crate. It listens on port 1032 and handles incoming requests.

The server has two main endpoints:

-   `/`: This is the main endpoint that serves a PNG image. The image is generated by taking a screenshot of a web page rendered by a headless Chrome instance.
-   `/content/`: This endpoint serves the HTML content that is rendered by the headless Chrome instance. The HTML is generated using the `fun-html` crate.

When a request comes in to the `/` endpoint, the server spawns two threads:

1.  One thread to immediately return the currently cached image.
2.  Another thread to update the image in the background. This is done by:
    1.  Making a request to the `/content/` endpoint to get the latest HTML.
    2.  Using the `headless_chrome` crate to render the HTML and take a screenshot.
    3.  The resulting image is then cached in memory for future requests.

This ensures that requests to the main endpoint are always fast, while the image is updated asynchronously.

## How to build

To build and run this project, you need to have Rust and Cargo installed.

1.  **Build the project:**
    ```bash
    cargo build
    ```
2.  **Run the project:**
    ```bash
    cargo run
    ```

This will start the server on port 1032. You can then access the server by navigating to `http://localhost:1032` in your web browser.
