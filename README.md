# Inhibit

I needed a small GTK app that would put an icon in my gnome-panel that would inhibit the screensaver when clicked.

Nothing seemed to fit my custom Xmonad + Cinnamon setup, so I wrote this little Rust application.

## Build

```shell script
$ sudo apt install libappindicator3-dev libgtk-3-dev gcc clang
$ cargo build --release
```

## Run

```shell script
$ inhibit
```
inhibit

## Licence

Apache 2

---

This is not an official Google Product. 