#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"

if [[ -n "$(ls -1 img_dark/*.png 2>/dev/null)" ]]; then
  # Framerate: -r 20
  # Do NOT use x265; it takes more time and results in a larger file. -c:v libx265
	ffmpeg -r 20 -i 'img_dark/%d.png' "animation$(date '+%Y%m%dT%Hh%Mm%Ss').mkv"
else
	echo 'Please make sure there are png files in the img_dark/ folder.'
fi
