# Ferrite Image Viewer: Roadmap to "Default Rust Image Viewer"

This roadmap outlines the key development phases and goals for Ferrite to establish itself as a leading, potentially "default," image viewer within the Rust ecosystem and beyond.

## Guiding Principles

*   **Performance:** Lightning-fast loading, smooth interaction, efficient resource usage.
*   **Stability & Reliability:** Robust, crash-resistant, handles diverse image files gracefully.
*   **Extensibility:** Modular design to facilitate new features and contributions.
*   **User Experience (UX):** Intuitive, configurable, and aesthetically pleasing.
*   **Cross-Platform:** Excellent support for Windows, macOS, and Linux.
*   **Community-Driven:** Open to contributions, feedback, and evolving with user needs.

## Phase 1: Solid Foundation & Core Experience (Current - Q3 2024)

**Goal:** Achieve a highly performant, stable, and usable core image viewing experience. Address immediate architectural improvements.

*   **Core Performance & Stability:**
    *   [ ] **Cache Optimization:** Implement `lru::LruCache` for image data and potentially thumbnails. (High Priority)
    *   [ ] **Refactor Cache Manager:** Consolidate image loading logic and clarify API contracts.
    *   [ ] **Asynchronous Image Decoding:** Ensure image decoding (especially for large/complex formats) doesn't block cache threads or the UI thread. (e.g., `tokio::spawn_blocking` or `rayon`).
    *   [ ] **Error Handling Overhaul:** Comprehensive error reporting to the user (not just logs) for file loading, decoding, and caching failures. Avoid panics.
    *   [ ] **Memory Management Review:** Profile and optimize memory usage, especially for large images and long sessions.
*   **Configuration & UI Polish:**
    *   [ ] **Resolve Config Duplications:** Consolidate `NavigationConfig` and ensure `ZoomHandler` uses global config for defaults.
    *   [ ] **Consistent UI Indicator Positioning:** Use `IndicatorConfig.position` for all on-screen indicators.
    *   [ ] **Configurable Sorting:** Implement sorting options (name, date, size) in `ferrite-navigation` driven by `FerriteConfig`.
    *   [ ] **Refine CLI:** Ensure error messages from CLI operations are user-friendly.
*   **Basic Feature Parity:**
    *   [ ] **Basic Animated GIF/WebP Support:** Display first frame correctly (already likely) or basic animation loop if `image` crate supports it easily.
    *   [ ] **Full-Screen Mode:** Implement a toggle for a true full-screen experience (beyond just borderless).
*   **Build & CI:**
    *   [ ] **Robust CI:** Automated builds and tests for major platforms (Linux, Windows, macOS).
    *   [ ] **Fix Workspace Metadata:** Correct `repository` and `description` in `Cargo.toml`.

## Phase 2: Feature Expansion & Format Support (Q4 2024 - Q1 2025)

**Goal:** Broaden format support and add essential features expected from a modern image viewer.

*   **Expanded Image Format Support:** (Refer to `TODO_IMAGE_FORMATS.md`)
    *   [ ] **High Priority Formats:** AVIF, JPEG XL, HEIC.
    *   [ ] **RAW Image Support:** Initial support for common RAW formats (e.g., via `rawloader`).
    *   [ ] **Animated Formats:** Robust support for animated GIF, WebP, APNG (playback, basic controls).
*   **Key Features:**
    *   [ ] **Thumbnail Browser:**
        *   [ ] Asynchronous thumbnail generation and caching.
        *   [ ] Basic directory navigation within the app.
        *   [ ] Configurable thumbnail size.
    *   [ ] **Metadata Display:**
        *   [ ] Show basic EXIF data (dimensions, camera, date).
        *   [ ] Expand to more detailed EXIF, IPTC.
    *   [ ] **Color Management (v1):**
        *   [ ] Attempt to read and apply embedded ICC profiles for common formats (sRGB, Display P3).
    *   [ ] **Basic Image Operations (Non-Destructive):**
        *   [ ] Rotation (90-degree increments).
        *   [ ] Flipping (horizontal/vertical).
    *   [ ] **Slideshow Mode:** Basic automated image advancement with configurable delay.
*   **UX Enhancements:**
    *   [ ] **"Open With..." Functionality:** Allow opening the current image in an external editor.
    *   [ ] **Copy to Clipboard:** Copy current image to system clipboard.
    *   [ ] **Customizable Mouse Controls:** Allow users to configure actions for mouse buttons/wheel.

## Phase 3: Advanced Features & Polish (Q2 2025 - Q4 2025)

**Goal:** Introduce advanced capabilities, refine UX, and build community engagement.

*   **Advanced Image Features:**
    *   [ ] **Advanced Color Management:** More comprehensive ICC profile support, monitor profile awareness (if feasible with `egui`/`wgpu`).
    *   [ ] **Pixel Inspector/Color Picker.**
    *   [ ] **Histogram Display.**
    *   [ ] **Image Comparison (Side-by-Side).**
    *   [ ] **SVG Support** (via `resvg`).
    *   [ ] **Basic Image Adjustments:** Brightness, contrast, saturation (non-destructive).
*   **Application Polish:**
    *   [ ] **Theming:** Allow users to select from a few predefined themes or customize UI colors more deeply.
    *   [ ] **Internationalization (i18n) / Localization (L10n):** Prepare the app for translation.
    *   [ ] **Accessibility (A11y) Review:** Improve keyboard navigation and screen reader compatibility.
    *   [ ] **Drag & Drop:** Support dragging image files onto the application window to open them.
    *   [ ] **Command Palette** (e.g., Ctrl+P) for quick access to actions.
*   **Community & Ecosystem:**
    *   [ ] **Website/User Manual:** Create a simple website with downloads, features, and a user guide.
    *   [ ] **Contribution Guidelines:** Clear guidelines for code contributions, bug reports, and feature requests.
    *   [ ] **Packaging:** Provide easy-to-install packages for major platforms (e.g., `.deb`, `.rpm`, `.msi`, Homebrew, Flatpak, Snap).
    *   [ ] **Plugin System (Ambitious Long-Term):** Consider a plugin architecture for third-party extensions (e.g., new format support, custom tools).

## Phase 4: "Default" Status & Continued Evolution (2026+)

**Goal:** Become a widely recognized, highly recommended image viewer in the Rust community and a strong contender generally.

*   **Deepening Platform Integration:**
    *   [ ] File associations (setting Ferrite as default viewer).
    *   [ ] Shell extensions (e.g., thumbnails in file explorer - platform specific and complex).
*   **Performance Leadership:** Continuously benchmark against other viewers and optimize.
*   **Advanced Features (User-Driven):**
    *   [ ] Batch processing.
    *   [ ] Advanced RAW processing controls.
    *   [ ] Print preview and printing.
    *   [ ] Multi-window/tabbed interface.
*   **Sustained Maintenance & Community Growth:** Active bug fixing, feature development based on feedback, fostering a healthy contributor base.
*   **Marketing & Outreach:** (Subtle) Blog posts, presentations (Rust meetups), ensuring Ferrite is listed in relevant software directories.

## Cross-Cutting Concerns (Ongoing)

*   **Documentation:** Keep API docs (`docs.rs`), internal code comments, and user-facing documentation up-to-date.
*   **Testing:** Continuously expand unit, integration, and (if possible) UI tests.
*   **Benchmarking:** Regularly benchmark critical paths.
*   **Code Quality:** Maintain high code quality through reviews, linting, and `rustfmt`.
*   **Dependency Management:** Keep dependencies updated and audit for security.

---

This roadmap is ambitious and will require sustained effort. The key is to deliver value incrementally, focus on quality, and listen to user feedback. Good luck making Ferrite the go-to image viewer for Rustaceans and beyond!