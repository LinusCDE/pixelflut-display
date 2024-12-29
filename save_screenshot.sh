#!/bin/sh

./target/release/pixelflut-display | ffmpeg -f rawvideo -pixel_format rgb24 -video_size 3840x1080 -i - -vframes 1 $1
