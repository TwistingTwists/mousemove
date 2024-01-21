how to cross compile for windows?

1. install the windows toolchain
 `rustup target add x86_64-pc-windows-gnu`
2. install the linker for C on windows
`brew install mingw-w64 `
3. Add some config in Cargo.toml
4. `cargo build --target x86_64-pc-windows-gnu --release`


## Using cargo dist -- this does not cross compile for windows. 
the msi installer step for windows requires windows OS with wixv3 isntalled on it. That's the only way to produce msi installers. instead just use the cross compile for windows to produce the gnu binary for windows.
1. install cargo dist
2. follow the cli tutorial to choose builds / platforms etc
3. run `cargo dist build` to generate platform binaries for each version

## NOTE: remember to cleanup and invalidate .env file before making this repo public
