# ndarray blas integration for A tranpose dot B

For matrix $A$, calculate

$$
A^TB
$$

or

```rust
let a = Array2::<f64>::zeros((10000, 1000));
let b = Array2::<f64>::zeros((10000, 1000));
let _ = a.t().dot(&b);
```

This can be calculated with `gemm`, however, running the above code doesn't call blas - see [src/bin/no-blas.rs](./src/bin/no-blas.rs).

However, below code does call blas routines, see [src/bin/blas.rs](src/bin/blas.rs).

```rust
let b = Array2::<f64>::zeros((10000, 1000));
let at = Array2::<f64>::zeros((1000, 10000));
let _ = at.dot(&b);
```

Code can be run by

```shell
cargo run --profile release --bin blas && cargo run --profile release --bin no-blas
```

This will generate two profiles in the local directoy, `blas.pb` and `no-blas.pb`, which can be view with [pprof](https://github.com/google/pprof)
