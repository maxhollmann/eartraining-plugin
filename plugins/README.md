VST3 / CLAP plugins to relay what's being played to the main app.

## Dependencies

```shell
sudo apt install libasound2-dev libjack-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-icccm4-dev libx11-xcb-dev libxcb-dri2-0-dev freeglut3-dev libxcursor-dev
```

## Build

* Build & install all packages: `just`
* Build & install a package and watch the logs: `just try <package>`

Available packages:
* `eartraining-plugin`
* `tonic_relay`
