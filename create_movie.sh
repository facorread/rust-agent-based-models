#!/bin/bash

cd "$( dirname "${BASH_SOURCE[0]}" )"
set -eEu

imgdir='img_dark'

if [[ -n "$(ls -1 "$imgdir"/*.png 2>/dev/null)" ]]; then
  # Framerate: -r 20
  # Do NOT use x265; it takes more time and results in a larger file. -c:v libx265
  filename="$(date '+%Y%m%d %Hh%Mm%Ss').mkv"
	ffmpeg -r 20 -i "$imgdir"/'%d.png' "$filename"
  echo "$filename"
else
	echo "Please make sure there are png files in the ${imgdir}/ folder."
fi
