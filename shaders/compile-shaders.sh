#!/usr/bin/env bash
shopt -s globstar
for x in *.vert *.frag; do
    glslc -g -O $x -o $x.spv &
done
wait
