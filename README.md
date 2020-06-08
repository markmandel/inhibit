# Inhibit

I needed a small GTK app that would put an icon in my gnome-panel that would inhibit the screensaver when clicked.

Nothing seemed to fit my custom Xmonad + Cinnamon setup, so I wrote this little Rust application.

After building this, I realised there was a native thing in gnome-panel, but I'm still using this, so that I can choose
when I want it in my status bar.

## Install
```shell script
$ make install
```

## Build

```shell script
$ make deps # installs apt dependencies
$ make build # compiles binary and puts assets in the correct places
```

## Run

```shell script
$ ./inhibit
```

## Licence

Apache 2

---

This is not an official Google Product. 