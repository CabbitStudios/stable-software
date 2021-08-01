🐴Stable Software🦄
====================

This software is custom-built to keep track of tending to your horses.

It is currently being planned will need numerous updates to be useable.


Technical Overview
-----------------

"Stable Software" is a classical approach to website building. With one solid, fast and performant backend and a rather boring frontend, consisting of mostly HTML+CSS and JS for simple interactivity.

The following directories are important:

- `src` -- The backend, which is a rust application
- `assets` -- All assets that will be copied/transformed to their final representation which are later needed for deployment



Getting started
---------------

To get started, you will need to install both an up-to-date version of Rust and Node.JS. 
Then, run:

```
cargo make build
```

This will create the complete application in the `build` directory.

Simply run `start_stable_software.sh`.