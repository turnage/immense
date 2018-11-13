# immense

A library for describing 3D meshes with L-Systems.

```rust
Rule::new().push(
    vec![
        Replicate::n(36, vec![Tf::rz(10.0), Tf::ty(0.1)]),
        Replicate::n(36, vec![Tf::ry(10.0), Tf::tz(1.2)]),
    ],
    cube(),
)
```

![](https://i.imgur.com/5ccKkpQ.png)