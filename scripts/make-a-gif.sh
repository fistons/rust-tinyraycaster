#!/bin/bash

rm *.ppm
rm *.mp4

time cargo run --release
time ffmpeg -framerate 60 -i out_%03d.ppm -c:v libx264 -crf 25 -vf "format=yuv1080p" -movflags +faststart wulgenstein.mp4
