#!/usr/bin/env bash

JAVA_FLAGS="--target 21 -Xlint:unchecked -Xdiags:verbose"

build() {
    if [ ! -e "src/$1.java" ]; then
        echo "Build target ($1) not found."
        return
    fi

    if [ ! -e "build/$1.class" ]; then
        echo "Building target $1 for the first time."
        javac $JAVA_FLAGS -d build "src/$1.java"
        return
    fi

    if [ "src/$1.java" -nt "build/$1.class" ]; then
        echo "Rebuilding target $1."
        javac $JAVA_FLAGS -d build "src/$1.java"
        return
    fi
    echo "Nothing to be done for $1."
}

mkdir -p build

for d in $(seq 25); do
    build "$(printf "day%02d\n" $d)"
done

# java --class-path build $1
