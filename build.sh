TRAGET=x86_64-unknown-linux-musl
#TRAGET=i686-unknown-linux-musl
cross build --release --target $TRAGET $FLAGS
strip ./target/$TRAGET/release/gugugu
ls -lh ./target/$TRAGET/release/gugugu
