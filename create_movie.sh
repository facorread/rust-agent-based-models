#!/bin/bash

if [[ -n "$(ls -1 img_dark/*.png 2>/dev/null)" ]]; then
  # Framerate: -r 20
	ffmpeg -r 20 -i 'img_dark/%d.png' "animation$(date '+%F %Hh%Mm').mkv"
else
	echo 'Please make sure there are png files in the img_dark/ folder.'
fi
