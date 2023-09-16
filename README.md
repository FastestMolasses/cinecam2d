# CineCam2D

<!-- Centered Project Logo -->
<p align="center">
  <img src="logo.png" alt="CineCam2D Logo" width="200"/>
</p>

CineCam2D is a 2D camera library for games and other interactive applications built on top of [Bevy](https://bevyengine.org/). Add features like focus, bounding box, panning, shake, and zoom to your Bevy application with ease.

<!-- Demo GIF -->
<p align="center">
  <img src="demo.gif" alt="CineCam2D Demo" />
</p>

## Features

- **Focus**: Camera focus on a single or multiple entities, with optional smoothing.
- **Bounding Box** (`bound` feature): Constrain camera within a bounding box.
- **Panning** (`pan` feature): Manual camera panning.
- **Zooming** (`zoom` feature): Manual camera zooming.
- **Shake** (`shake` feature): Camera shake effects using random noise.

## Installation

Add `cinecam2d` to your `Cargo.toml`:

```toml
[dependencies]
cinecam2d = "0.1.0"
```

## Optional Features
You can optionally include specific features:

```toml
[dependencies]
cinecam2d = { version = "0.0.1", features = ["bound", "pan", "shake", "zoom"] }
```

## Quickstart
Add `cinecam2d` to your Bevy app:

```rs
mod character;

use bevy::prelude::*;
use cinecam2d::{focus::FocusTarget, CineCam2DPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CineCam2DPlugin)
        .add_systems(Startup, world_setup)
        .run();
}

fn world_setup(commands: Commands) {
    cinecam2d::init(commands, Transform::from_xyz(0.0, 0.0, 10.0));
}
```

## Focus Camera on an Entity
```rs
```

Focusing on multiple entities work the same way, just add the `FocusTarget` component to each entity.

## Apply Bounding Box
```rs
```

## Panning
```rs
```

## Zooming
```rs
```

## Shake Camera
```rs
```

## License
This project is under the MIT License - see the LICENSE.md file for details.
