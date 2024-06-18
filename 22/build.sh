#!/usr/bin/env bash

build() {
    if [ ! -e "src/$1.java" ]; then
        echo "Build target ($1) not found."
        return
    fi

    if [ ! -e "build/$1.class" ]; then
        echo "Building target $1 for the first time."
        javac --target 21 -d build "src/$1.java"
        return
    fi

    if [ "src/$1.java" -nt "build/$1.class" ]; then
        echo "Rebuilding target $1."
        rm "build/$1.class"
        javac --target 21 -d build "src/$1.java"
        return
    fi
    echo "Nothing to be done for $1."
}

mkdir -p build

for d in $(seq 8); do
    build "$(printf "day%02d\n" $d)"
done

# java --class-path build $1
