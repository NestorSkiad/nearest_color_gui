# Nearest Color GUI

This is a **proof-of-concept project** for a feature I'd like included in DaVinci Resolve. I've recently taken up some video
production projects ([available here]()), which have involved doing a fair amount of color work.

In the color tab in Resolve, you use nodes to apply different effects. The nodes are small and it's easy to create a complicated
web, so you're supposed to name each node. Naming nodes isn't very creative, for example if you use a color space transform
from ACES-CCT to Rec.709, you'd usually name the node something like "acescct to rec709". The practice becomes tedious,
so I've stopped naming my nodes.

This obviously wouldn't be very difficult to automate, but what might be more difficult is auto-naming nodes that involve
colors. For those nodes, you'd name them something like "teal lows & orange highs", but color selection is done through
color wheels, so we'd have to get the color name from the RGB values.

With AI and ML, vector similarity is a massive problem. Fortunately, because there's not that many named colors out there,
we can implement a relatively simple O(N) algorithm to find the nearest color.

For a given color that the user inputs, we calculate its Euclidean distance from each of the ~850 named colors we've got and
find the minimum.

This happens extremely quickly, so much so that I've developed a non-GUI version of this app, which finds the nearest named
color for all 16777216 8-bit RGB colors in only a couple of minutes.

### Frameworks used

* EGui - with the EFrame template. It's an intermediate level UX framework, one level above Win32 and OpenGL. There's a color picker in the big example project, but it was impossible to adapt to my needs.
* csv - used to read the color names and RGB values from [this file from Codebrainz](https://github.com/codebrainz/color-names/blob/master/output/colors.csv).

### Testing locally

Make sure you are using the latest version of stable rust by running `rustup update`.

`cargo run --release`

On Linux you need to first run:

`sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`

On Fedora Rawhide you need to run:

`dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel`

### Web Locally

You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

We use [Trunk](https://trunkrs.dev/) to build for web target.
1. Install the required target with `rustup target add wasm32-unknown-unknown`.
2. Install Trunk with `cargo install --locked trunk`.
3. Run `trunk serve` to build and serve on `http://127.0.0.1:8080`. Trunk will rebuild automatically if you edit the project.
4. Open `http://127.0.0.1:8080/index.html#dev` in a browser. See the warning below.

> `assets/sw.js` script will try to cache our app, and loads the cached version when it cannot connect to server allowing your app to work offline (like PWA).
> appending `#dev` to `index.html` will skip this caching, allowing us to load the latest builds during development.

### Web Deploy
1. Just run `trunk build --release`.
2. It will generate a `dist` directory as a "static html" website
3. Upload the `dist` directory to any of the numerous free hosting websites including [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).
4. we already provide a workflow that auto-deploys our app to GitHub pages if you enable it.
> To enable Github Pages, you need to go to Repository -> Settings -> Pages -> Source -> set to `gh-pages` branch and `/` (root).
>
> If `gh-pages` is not available in `Source`, just create and push a branch called `gh-pages` and it should be available.

You can test the template app at <https://emilk.github.io/eframe_template/>.