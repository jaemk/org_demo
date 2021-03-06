# This script takes care of building your crate and packaging it for release

set -ex

main() {
    local src=$(pwd) \
          stage=

    case $TRAVIS_OS_NAME in
        linux)
            stage=$(mktemp -d)
            ;;
        osx)
            stage=$(mktemp -d -t tmp)
            ;;
    esac

    test -f Cargo.lock || cargo generate-lockfile

    # Build artifacts
    ./build.py web

    cross rustc --bin org_demo --target $TARGET --release -- -C lto
    cp target/$TARGET/release/org_demo bin/

    mkdir -p $stage/org_demo
    cp -r * $stage/org_demo

    cd $stage
    rm -rf org_demo/target/
    rm -rf org_demo/web/node_modules
    rm -rf org_demo/web/build
    tar czf $src/$CRATE_NAME-$TRAVIS_TAG-$TARGET.tar.gz *
    cd $src

    rm -rf $stage
}

main
