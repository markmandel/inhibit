# Inhibit

I needed a small GTK app that would put an icon in my gnome-panel that would inhibit the screensaver when clicked.

Nothing seemed to fit my custom Xmonad + Cinnamon setup, so I wrote this little Rust application.

(After building this, I realised there was a native thing in gnome-panel, but this was still fun to make)

## Build

```shell script
$ make deps # installs dependencies
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