# beamtrace

Code for representing, creating and rendering multi-page line plots.
It uses MPK for serialization and deserialization.

## What use is this?

Think old school HPGL plots, but with multiple page and animation
support.

## Usage

1. Build using `cargo build --release --workspace --examples`
2. Run the example `target/release/examples/averaging_kernels`.  This will produce `traj.mpk`
3. Render that file using `target/release/render_book traj.mpk foo`
4. Look at the resulting `foo-000-000.png`

## Author

Berke DURAK <bd@exhrd.fr>
