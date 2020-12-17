TRAGET=x86_64-unknown-linux-musl
#TRAGET=i686-unknown-linux-musl
FLAGS="" #-Z build-std
cross build --release --target $TRAGET $FLAGS
strip ./target/$TRAGET/release/gugugu
ls -lh ./target/$TRAGET/release/gugugu
