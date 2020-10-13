# 0.1.4 (2020/10/12)

* Add 9-Patch UI element `Entity` to [`NinePatchContent`](https://docs.rs/bevy_ninepatch/0.1.3/bevy_ninepatch/struct.NinePatchContent.html)
* Hide manual way to add a 9-Patch UI element behind a feature
* Do not recreate materials when re-using a 9-Patch UI element builder
* Remove component `NinePatchSize` to use the `size` attribute of component `Style`
