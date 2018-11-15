# immense

A library for describing 3D meshes with simple composable rules.

```rust
Rule::new().push(
    vec![
        Replicate::n(1, vec![Tf::saturation(0.8), Tf::hue(160.0)]),
        Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
        Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2)]),
    ],
    cube(),
)
```

![](https://i.imgur.com/1Emik4Z.png)