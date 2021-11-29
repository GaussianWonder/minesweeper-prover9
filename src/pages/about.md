---
title: About
---

<div class="text-center">
  <!-- You can use Vue components inside markdown -->
  <carbon-dicom-overlay class="text-4xl -mb-6 m-auto" />
  <h3>About</h3>
</div>

This is an opinionated [Vite](https://github.com/vitejs/vite) starter template for [Tauri](https://tauri.studio/en/) apps. With **file-based routing**, **components auto importing**, **markdown support**, I18n, PWA and uses **WindiCSS** for UI. Check out the [template](https://github.com/antfu/vitesse) this started from

```cpp
// syntax highlight example
template<typename T>
concept Hashable = requires(T a) {
  { std::hash<T>{}(a) } -> std::convertible_to<std::size_t>;
};
```
