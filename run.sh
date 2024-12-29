#!/bin/sh

./target/release/pixelflut-display | ffplay -f rawvideo -pixel_format rgb24 -video_size 3840x1080 -i - -loglevel quiet -hide_banner
