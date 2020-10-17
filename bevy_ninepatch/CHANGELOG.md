# 0.2.0 (TBD)

* Added the 9-Patch UI element `Entity` to [`NinePatchContent`](https://docs.rs/bevy_ninepatch/0.1.3/bevy_ninepatch/struct.NinePatchContent.html)
* Removed manual way to add a 9-Patch UI element
* Do not recreate materials when re-using a 9-Patch UI element builder
* Removed component `NinePatchSize` to use the `size` attribute of component `Style`
* Removed custom types for patch declaration, instead use `Size<i32>` and `Size<Val>` from bevy
* Optimised to reduce the hierarchy of entities created
* Added `content` field in `NinePatchData` that will list entities to use as contents for element
