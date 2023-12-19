#!/bin/zsh

sys="$(~/.config/hypr/sys/target/debug/sys)"
notify-send -t 3000 -a Resources -c Resources "$sys"
