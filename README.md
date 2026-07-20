# vortex-mod-captcha-browser

Official human-assisted fallback for Vortex image and text CAPTCHAs.

The plugin has no host capabilities. It returns `interaction_required`, and
Vortex opens a constrained local Tauri WebView where the user can submit the
answer. JavaScript challenges such as reCAPTCHA and hCaptcha remain out of scope.

```bash
cargo test --lib
cargo build --target wasm32-wasip1 --release
cargo test
```

GPL-3.0.
