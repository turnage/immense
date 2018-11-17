# immense

![](https://docs.rs/immense/badge.svg) ![crates.io](https://img.shields.io/crates/v/immense.svg) ![](https://travis-ci.org/turnage/immense.svg?branch=master)

A library for describing 3D meshes with simple composable rules.

```rust
rule![
    tf![
        Tf::saturation(0.8),
        Tf::hue(160.0),
        Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
        Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2), Tf::hue(3.4)]),
    ] => cube()
]
```

![](https://i.imgur.com/1Emik4Z.png)