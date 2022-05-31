# time-tauri

## Known issues

### Saving settings

Currently there is a bug with saving the settings. You can fix this by creating a `settings.txt` inside the Applications Spport folder. You can find it under path `/Users/andreasschneider/Library/Application Support/blumewas.timetauri`. Inside the `settings.txt` put the following content:

```
{"apiKey":"YOUR_MITE_APIKEY","stared":[],"miteApp":"YOUR_MITE_APP"}
```

1. `YOUR_MITE_APIKEY` - replace with your mite api key. You cann find it inside your profile on *mite.yo.lk*
2. `YOUR_MITE_APP` - first part of the URL you use to access mite. `e.g. company.mite.yo.lk here it would be "company"`

### Search

The search is currently not implemented.

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Lints and fixes files
```
npm run lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).
