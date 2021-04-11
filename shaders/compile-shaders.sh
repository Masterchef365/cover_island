#!/usr/bin/env bash
for x in *.vert *.frag; do
    glslc -g -O $x -o $x.spv &
done
wait
