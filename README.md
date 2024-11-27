# Oklch Color Picker

<img src="https://github.com/user-attachments/assets/7d809284-3c2b-48ed-a558-ed1c7cc41dd5" width="100%" alt="screenshot">

[![Crates.io](https://img.shields.io/crates/v/oklch-color-picker)](https://crates.io/crates/oklch-color-picker)

Try the web demo: https://oklch.eerolehtinen.fi/

## Features

- Takes an input color as a cli argument and ouputs the edited color to stdout
- Uses a perceptual colorspace (Oklch) to allow intuitive editing
  - Consists of lightness, chroma and hue
  - Motivation: [An article by the Oklab creator](https://bottosson.github.io/posts/oklab/)
  - Oklch uses the same theory as Oklab, but uses parameters that are easier to understand
  - L<sub>r</sub> estimate is used instead of L as specified in [another article by the same guy](https://bottosson.github.io/posts/colorpicker/#intermission---a-new-lightness-estimate-for-oklab)
- Supports many color formats for input and output (editing uses only Oklch):
  - Hex (`#RGB`, `#RGBA`, `#RRGGBB`, `#RRGGBBAA`)
  - Other common CSS formats (`rgb(..)`, `hsl(..)`, `oklch(..)`)
  - Hex literal (`0xRRGGBB`, `0xAARRGGBB`)
  - Any list of 3 or 4 numbers can be used as a color (e.g. `0.5, 0.5, 0.5` or `120, 120, 120, 255`)
- Hardware accelerated for maximum smoothness and high resolutions

## Installation

Download from [Releases](https://github.com/eero-lehtinen/oklch-color-picker/releases).

If you have **cargo**, you can also install with:

```sh
cargo install oklch-color-picker --locked
```

---

Check out the neovim plugin that this picker was made for [eero-lehtinen/oklch-color-picker.nvim](https://github.com/eero-lehtinen/oklch-color-picker.nvim).

Inspired by https://oklch.com/.
