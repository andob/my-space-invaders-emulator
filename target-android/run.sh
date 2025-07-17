#!/bin/bash
set -e #fail on first error

device_count=$(adb devices | tail -n +2 | grep -w "device" | wc -l)

if [ "$device_count" -eq 0 ]
then
    echo "No Android device connected!"
    exit 1
fi

adb shell wm size 2240x2560
echo "Changed screen resolution"

cargo apk run
