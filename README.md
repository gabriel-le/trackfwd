# Trackfwd - A command line application to broadcast OpenVR tracking data over OSC

This is a simple rust command line application that broadcasts OpenVR tracking data over OSC. I was frustrated with the LiveLinkXR plugin in Unreal Engine, so rather than relying on it, I decided to write my own solution.
This will only broadcast generic trackers (vive trackers, tundra trackers, etc) and not controllers/HMD, although it should be trivial to modify the code to include them.

## Usage

```Powershell
trackfwd <port>
```

For a detailed guide, see the application specific tutorials:

-   [Unreal Engine](tutorials/unreal.md)
-   [Unity](tutorials/unity.md)

### Arguments

-   `<port>`: The port to broadcast the OSC messages on.
-   `--target <target>`: The IP address of the OSC target. Default is `127.0.0.1`.
-   `--coords <coords>`: The coordinate system to use. Options are `open-vr`, `blender`, `unity` and `unreal`. Default is `open-vr`.
-   `--help`: Print help information.

### Coordinate Systems

Different applications use different coordinate systems. The `--coords` flag allows you to specify which coordinate system to use.

-   `open-vr`: The default coordinate system used by OpenVR. It uses a right-handed coordinate system with the y-axis pointing up. 1 unit is equal to 1 meter.
-   `blender`: The coordinate system used by Blender. It uses a right-handed coordinate system with the z-axis pointing up. 1 unit is equal to 1 meter.
-   `unity`: The coordinate system used by Unity. It uses a left-handed coordinate system with the y-axis pointing up. 1 unit is equal to 1 meter.
-   `unreal`: The coordinate system used by Unreal Engine. It uses a left-handed coordinate system with the z-axis pointing up. 1 unit is equal to 1 centimeter.

In the future, it may be useful to add a custom coordinate system option. For now, if you need a different unit scale than the ones provided, you can either modify the code or scale the values in the receiving application.

## OSC Message Format

The OSC message format is as follows:

```
/tracker/<index> <x> <y> <z> <qx> <qy> <qz> <qw>
```

`<index>` is the device index of the tracker. `<x>`, `<y>`, and `<z>` are the position of the tracker in meters. `<qx>`, `<qy>`, `<qz>`, and `<qw>` are the quaternion rotation of the tracker.

## Building

To build the application, you will need to have Rust installed. You can install Rust by following the instructions on the [official website](https://www.rust-lang.org/tools/install). Once you have Rust installed, you can build the application by running the following command:

```Powershell
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## What's Next?

There's a few features that could be added to this application to make it more useful:

-   Adding support for VRChat's OSC standard
-   Customizing the OSC message format
-   Support for controllers and HMDs
-   Adding arguments to choose the coordinate system and unit scale independently
-   Making a simple GUI for configuration
-   Recording and exporting tracking data
-   Playback of recorded tracking data
