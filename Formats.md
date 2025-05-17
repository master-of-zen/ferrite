# TODO: Image Format Support for Ferrite

This document tracks the image formats currently supported by Ferrite and those planned for future support to enhance its capabilities as a flagship image viewer.

## Currently Supported (Based on `ferrite-image/src/formats.rs`)

These formats are recognized by their extensions. Support level (e.g., animation, metadata) needs verification for some.

- [x] **JPEG/JPG** (`.jpg`, `.jpeg`)
- [x] **PNG** (`.png`)
- [x] **GIF** (`.gif`)
    - [ ] **Note:** Verify if animation is supported or only the first frame. `image` crate's `DynamicImage` typically loads the first frame. Full animation support is a separate task.
- [x] **BMP** (`.bmp`)
- [x] **ICO** (`.ico`)
- [x] **TIFF** (`.tiff`, `.tif`)
    - [ ] **Note:** TIFF can be complex (multi-page, various compressions). Verify depth of support.
- [x] **TGA** (`.tga`)
- [x] **WebP** (`.webp`)
    - [ ] **Note:** Verify if animated WebP is supported or only static. The `image` crate has WebP animation support, but the UI needs to handle it.

## To Be Supported (High Priority - Modern Essentials)

These formats are crucial for a modern, competitive image viewer.

- [ ] **AVIF** (`.avif`)
    - **Why:** Successor to HEIC, excellent compression, HDR, wide color gamut. Royalty-free.
    - **Potential Crates:** `libavif-rs` (bindings to libavif), `rav1e` (for encoding, but might expose decoding components or relevant knowledge), or direct integration with `image` crate if/when its support matures.
- [ ] **JPEG XL (JXL)** (`.jxl`)
    - **Why:** Excellent for both lossy and lossless, progressive decoding, animation, HDR. Royalty-free.
    - **Potential Crates:** `jxl-oxide` (pure Rust), `libjxl-rs` (bindings to libjxl).
- [ ] **HEIC/HEIF** (`.heic`, `.heif`)
    - **Why:** Common on Apple devices, good compression.
    - **Potential Crates:** `libheif-rs` (bindings to libheif). Note potential patent/licensing considerations for distribution if using certain decoders.
- [ ] **RAW Image Formats** (Various extensions: `.cr2`, `.cr3`, `.nef`, `.arw`, `.dng`, `.raf`, etc.)
    - **Why:** Essential for photographers.
    - **Potential Crates:** `rawloader` (uses libraw), `kamadak-exif` (might include raw decoding capabilities or point to them). This usually involves more than just decoding pixels (demosaicing, color profiles).

## To Be Supported (Medium Priority - Enhanced Capabilities)

- [ ] **SVG (Scalable Vector Graphics)** (`.svg`)
    - **Why:** Common vector format for web and design.
    - **Potential Crates:** `resvg` (excellent Rust library). Requires a different rendering pipeline than raster images.
- [ ] **Animated PNG (APNG)** (`.png` when animated)
    - **Why:** Lossless animation, an alternative to GIF.
    - **Potential Crates:** The `image` crate supports APNG decoding. UI needs to handle animation loop.
- [ ] **QOI (Quite OK Image Format)** (`.qoi`)
    - **Why:** Simple, fast, lossless format. Gaining some traction.
    - **Potential Crates:** `qoi` crate.
- [ ] **PSD (Adobe Photoshop Document)** (`.psd`)
    - **Why:** Widely used in design. Even supporting a flattened preview or basic layer visibility would be useful.
    - **Potential Crates:** `psd` crate. Full support is complex.

## To Be Supported (Lower Priority / Specialized)

- [ ] **OpenEXR** (`.exr`)
    - **Why:** Professional HDR format used in VFX and CG.
    - **Potential Crates:** `exr` crate.
- [ ] **Radiance HDR** (`.hdr`, `.rgbe`)
    - **Why:** Another HDR format.
    - **Potential Crates:** `image::codecs::hdr`.
- [ ] **DDS (DirectDraw Surface)** (`.dds`)
    - **Why:** Common in game development for textures.
    - **Potential Crates:** `image::codecs::dds`.
- [ ] **JPEG 2000** (`.jp2`, `.j2k`)
    - **Why:** Used in some archival and medical imaging.
    - **Potential Crates:** OpenJPEG bindings if available, or `image` crate if support is added.

## Considerations for Animated Formats

For formats that support animation (GIF, WebP, APNG, AVIF, JXL):

- [ ] **Animation Playback:** Implement a mechanism to decode and display frames sequentially.
- [ ] **Playback Controls:** (Optional, but good for flagship) Play/pause, frame stepping.
- [ ] **Looping Behavior:** Respect format's loop count.
- [ ] **Performance:** Efficiently decode and update textures for smooth animation.

## General Implementation Notes

-   For each new format, ensure:
    -   It's added to `ferrite-image/src/formats.rs::SupportedFormats::EXTENSIONS`.
    -   The `image` crate (or a specialized crate) is used for decoding.
    -   Error handling for corrupted or unsupported variants of the format is robust.
    -   Consider performance implications, especially for large images or complex formats.
    -   Test with a variety of sample files for each format.
-   **Color Management:** For a true flagship viewer, supporting embedded ICC profiles and displaying colors accurately is essential across all relevant formats. This is a larger, cross-cutting concern.
-   **Metadata:** Extracting and displaying metadata (EXIF, IPTC, XMP) is also a key feature that often ties into format support.

---

This list should give you a good roadmap. Remember to prioritize based on what you think will provide the most value to your target users. Good luck!