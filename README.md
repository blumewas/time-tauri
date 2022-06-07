# time-tauri

This repo contains a test project for the tauri framework.
The tauri framework is build in Rust and uses the Gecko webview
which enables us to compile webapps as native apps. 

This opens the possibility to compile the app cross-origin from
one single codebase. To enable some features on certain platforms
some tweaks are required.

The app us currently only tested on MacOs.

## Known issues

When minimizing the window via the decorator, the hotkey needs to be toggled two times.

## Contribution

### Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run tauri:serve
```

### Compiles and minifies for production
```
npm run tauri:build
```

### Customize Vue.js build configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
