# Invisible Fun

This project is a small, high-concurrency web server written in Rust that dynamically generates images from HTML content. It's designed to serve a webpage that shows a countdown, render that page using a headless browser, and then serve the resulting image.

The primary purpose is to display dynamic, nicely formatted content (like a countdown) in environments that can only display images, such as e-ink displays or simple image widgets.

## How to Run

1.  **Build the project:**
    ```bash
    cargo build --release
    ```
2.  **Run the server:**
    ```bash
    ./target/release/invisible_fun
    ```
The server will start on `http://localhost:1032`.

## Project Structure & Modules

The core logic is organized into several modules, each with a specific responsibility.

### `src/main.rs`

This is the entry point of the application. Its responsibilities include:
-   **Setting up the HTTP Server**: It initializes and runs a `tiny_http` server to listen for incoming requests on port 1032.
-   **Request Routing**: It handles incoming requests and routes them based on the URL path (`/` or `/content/`).
-   **Concurrency Management**: It spawns new threads for each incoming request to handle them concurrently without blocking the main server loop.
-   **Shared State Management**: It creates and manages the shared `Repository` instance, wrapped in an `Arc<>` to allow safe, concurrent access from multiple threads.

### `src/content_view.rs`

This file acts as a central hub for the `content_view` module.
-   It defines the `ContentView` trait, which provides a common interface for different types of content to be materialized into a renderable format (in this case, HTML).
-   It defines the `Content` enum, which represents the different kinds of content the application can produce.

### `src/content_view/repository.rs`

This module is the heart of the data management and caching logic.
-   **`Repository` Struct**: Holds the application's state, including the URL for the content, a `RwLock` for the cached image (`DisplayContent`), and the `ContentView` implementation (`Countdown`).
-   **Caching**: It implements a time-based caching mechanism. The `update_content` method checks if the cached image is still valid. If not, it re-renders it. This prevents the costly rendering process from running on every request.
-   **Thread-Safe Operations**: All methods on `Repository` take `&self` and use the internal `RwLock` to safely manage read and write access to the cache from multiple threads.

### `src/content_view/countdown.rs`

This module is responsible for generating the dynamic HTML for the countdown page.
-   **`Countdown` Struct**: Holds the title and target date for the countdown.
-   **`ContentView` Implementation**: It implements the `ContentView` trait. Its `materialize` function calculates the number of days remaining and injects the title and days into an HTML template.
-   **HTML Generation**: It uses the `fun-html` crate to programmatically build the HTML structure, which is then returned as a `String`.

### `src/content_view/html_renderer.rs`

This is where the magic of turning HTML into an image happens.

The `render` function in this module orchestrates a headless Chrome browser to take a screenshot of a webpage. This process is essential because the server needs to generate an *image* of the countdown, not just the HTML.

**The flow is as follows:**

1.  **Start the Server**: The server starts running on `0.0.0.0:1032`.
2.  **Initial Request**: A request comes to the `/` endpoint.
3.  **HTML Generation**: The server calls `repository.get_content()`, which in turn calls `countdown.materialize()` to generate the HTML for the countdown page.
4.  **Localhost Call**: The `html_renderer::render` function is invoked with the URL `http://localhost:1032/content/`.
5.  **Headless Browser**: Inside `render`, the `headless_chrome` crate launches a full, but invisible, Chrome browser instance.
6.  **Navigate**: The browser opens a new tab and navigates to the provided local URL (`http://localhost:1032/content/`). This causes the server to serve the countdown HTML it just generated to its own headless browser.
7.  **Wait for Content**: The browser waits for the specific HTML element with the ID `#content` to be present on the page. This ensures the dynamic content has loaded.
8.  **Capture Screenshot**: Once the element is ready, the function instructs the browser to capture a screenshot of that specific element's viewport.
9.  **Return Image Data**: The raw PNG data of the screenshot is returned as a `Vec<u8>`.
10. **Cache and Serve**: This image data is then cached in the `Repository` and sent as the response to the original request.
