#!/bin/bash
# Create a simple placeholder icon for development
convert -size 1024x1024 xc:transparent \
  -fill '#0ea5e9' -draw 'circle 512,512 512,256' \
  -fill white -pointsize 400 -gravity center -annotate +0+0 'GT' \
  icon.png 2>/dev/null || echo "ImageMagick not available, skipping icon generation"
