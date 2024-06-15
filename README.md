A VST3 / CLAP plugin for eartraining. Inspired by [Max Konyi](https://www.youtube.com/@maxkonyi)'s videos [The Secret of Ear Training](https://www.youtube.com/watch?v=u0P7gh789RI), [Feeling the Major Scale](https://www.youtube.com/watch?v=Y6BPB3Cso00), and [Feeling the Minor Scale](https://www.youtube.com/watch?v=F-wUQh1jEA4).

## Dependencies

```shell
sudo apt install libasound2-dev libjack-dev libxcb-shape0-dev libxcb-xfixes0-dev libxcb-icccm4-dev libx11-xcb-dev libxcb-dri2-0-dev freeglut3-dev libxcursor-dev
```

## Build

```shell
cargo xtask bundle eartraining-plugin
cargo xtask bundle tonic_relay
```
