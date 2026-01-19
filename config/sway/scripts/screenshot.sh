#!/bin/bash
# RururuOS Screenshot Script
# Enhanced screenshot utility for creative work

set -e

SCREENSHOT_DIR="${HOME}/Pictures/Screenshots"
mkdir -p "$SCREENSHOT_DIR"

timestamp=$(date +%Y%m%d_%H%M%S)

case "${1:-screen}" in
    screen)
        # Full screen
        filename="${SCREENSHOT_DIR}/screenshot_${timestamp}.png"
        grim "$filename"
        notify-send "Screenshot" "Saved to $filename" -i "$filename"
        ;;
    
    area)
        # Selected area
        filename="${SCREENSHOT_DIR}/area_${timestamp}.png"
        grim -g "$(slurp)" "$filename"
        notify-send "Screenshot" "Saved to $filename" -i "$filename"
        ;;
    
    window)
        # Active window
        filename="${SCREENSHOT_DIR}/window_${timestamp}.png"
        grim -g "$(swaymsg -t get_tree | jq -r '.. | select(.focused?) | .rect | "\(.x),\(.y) \(.width)x\(.height)"')" "$filename"
        notify-send "Screenshot" "Saved to $filename" -i "$filename"
        ;;
    
    clipboard)
        # Full screen to clipboard
        grim - | wl-copy
        notify-send "Screenshot" "Copied to clipboard"
        ;;
    
    clipboard-area)
        # Area to clipboard
        grim -g "$(slurp)" - | wl-copy
        notify-send "Screenshot" "Copied to clipboard"
        ;;
    
    color)
        # Pick color
        color=$(grim -g "$(slurp -p)" -t ppm - | convert - -format '%[pixel:p{0,0}]' txt:- | tail -1 | cut -d ' ' -f 4)
        echo -n "$color" | wl-copy
        notify-send "Color Picker" "Copied: $color"
        ;;
    
    ocr)
        # OCR from area
        text=$(grim -g "$(slurp)" -t png - | tesseract - - 2>/dev/null)
        echo -n "$text" | wl-copy
        notify-send "OCR" "Text copied to clipboard"
        ;;
    
    record-start)
        # Start screen recording
        filename="${SCREENSHOT_DIR}/recording_${timestamp}.mp4"
        wf-recorder -f "$filename" &
        echo $! > /tmp/wf-recorder.pid
        notify-send "Recording" "Started recording..."
        ;;
    
    record-area-start)
        # Start area recording
        filename="${SCREENSHOT_DIR}/recording_${timestamp}.mp4"
        wf-recorder -g "$(slurp)" -f "$filename" &
        echo $! > /tmp/wf-recorder.pid
        notify-send "Recording" "Started recording area..."
        ;;
    
    record-stop)
        # Stop recording
        if [ -f /tmp/wf-recorder.pid ]; then
            kill $(cat /tmp/wf-recorder.pid) 2>/dev/null || true
            rm /tmp/wf-recorder.pid
            notify-send "Recording" "Recording saved"
        fi
        ;;
    
    *)
        echo "Usage: $0 {screen|area|window|clipboard|clipboard-area|color|ocr|record-start|record-area-start|record-stop}"
        exit 1
        ;;
esac
