# ATTENTION!!
This is a fork from [Dusk-Labs/dim](https://github.com/Dusk-Labs/dim)

At first I was thinking about just contributing to the project but I just kept finding errors and stuff I didn't like.
Also their repo seems dead now so this will probably ended being its own project.

# Warning!
Right now I will focus on Linux only (I have no respect for windows or macos), feel free to contribute changes to change that as long as it doesn't make it more complex.

# Goals
 * Simplify streaming pipeline (gstreamer) []
 * Replace UI components with components from a library (shadcn) []
 * Replace sass with tailwindcss (no need for .scss files) []
 * Delete redux and dispatch nonsense with Context (this app is not that complex) []
 * Use fetch directly in the pages, for all API calls (no need for jumping around files) []
 * Replace all typescript with javascript []

<h1 align="center">Dim</h1>

![Dashboard](docs/design/dashboard.jpg)

Dim is a self-hosted media manager. With minimal setup, Dim will organize and beautify your media collections, letting you access and play them anytime from anywhere.

### Dependencies

* libva2
* libva-drm2
* libharfbuzz
* libfontconfig
* libfribidi
* libtheora
* libvorbis
* libvorbisenc
* libtheora0

## Running from source

### Dependencies

To run from source, you'll first need to install the following dependencies on your system:

* sqlite
* cargo, rustc
* yarn, npm
* libssl-dev
* libva2 (only if you're using Linux)
* libva-dev (only if you're using Linux)
* libva-drm2 (only if you're using Linux)
* ffmpeg

Once the dependencies are installed, clone the repository and build the project:

### OS requirements


```
git clone https://github.com/FelipeSPerez/dim
```

If you're on Linux, run dim with:

```
cargo run --features vaapi --release
```

On other platforms where libva isn't available, run dim with:

```
cargo run --release
```

## License

Dim is licensed under the AGPLv3 license (see [LICENSE.md](LICENSE.md) or https://opensource.org/licenses/AGPL-3.0)

## Screenshots

![Login_Page](docs/design/login_page.png)
![Add_Library Modal](docs/design/add_library.png)
![Media_Page](docs/design/media_page.jpg)
