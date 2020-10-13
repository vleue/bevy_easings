# Examples

## Source PNGs

The PNGs used as sources for the different examples are:

![panel](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/assets/glassPanel_corners.png)
![button](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/assets/blue_button02.png)
![corner panel](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/assets/metalPanel_yellowCorner.png)

All assets are created by [Kenney](https://www.kenney.nl).

## As a plugin

### [plugin.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/plugin.rs)

Adding a simple 9-Patch UI element by using the `NinePatchComponents` component bundle.

![example with plugin](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/plugin.png)

### [change_size.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/change_size.rs)

Changing the size of a 9-Patch UI element by modifying the `Style.size` component.

![changing size of component](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/change_size.gif)

### [content.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/content.rs)

Adding content inside a 9-Patch UI element by modifying the `NinePatchContent` component. This approach only works if there is only one 9-Patch UI element.

![setting content](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/content.png)

### [multi_content.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/multi_content.rs)

Adding content inside a 9-Patch UI element by modifying the `NinePatchContent` component, and adding an `enum` to specify which content is being set.

![setting several contents](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/multi_content.png)

### [full.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/full.rs)

Complete example with:
* a complex 9-Patch UI element with two different content zones and a top bar that has two different parts that can grow
* 9-Patch UI elements inside a 9-Patch UI element inside a 9-Patch UI element
* Some 9-Patch UI elements change size during time

![full example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/full.gif)

## Without plugin

### [no_plugin_simple.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/no_plugin_simple.rs)

Adding two 9-Patch UI elements next to each other.

![simple example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/no_plugin_simple.png)

### [no_plugin_content.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/no_plugin_content.rs)

Adding two 9-Patch UI elements, one inside the other.

![content example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/no_plugin_content.png)

### [no_plugin_full.rs](https://github.com/mockersf/bevy_extra/blob/master/bevy_ninepatch/examples/no_plugin_full.rs)

Adding a 11-patch UI element with 2 zones, and content inside those zones

![full example](https://raw.githubusercontent.com/mockersf/bevy_extra/master/bevy_ninepatch/examples/no_plugin_full.png)

