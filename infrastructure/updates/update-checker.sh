#!/bin/bash
# RururuOS Update Checker
# Checks for system updates and notifies user

set -e

CACHE_DIR="${XDG_CACHE_HOME:-$HOME/.cache}/rururu"
LAST_CHECK_FILE="${CACHE_DIR}/last-update-check"
CHECK_INTERVAL=86400  # 24 hours

mkdir -p "$CACHE_DIR"

check_updates() {
    # Update package database
    sudo pacman -Sy > /dev/null 2>&1
    
    # Get available updates
    local updates
    updates=$(pacman -Qu 2>/dev/null)
    
    if [ -n "$updates" ]; then
        local count
        count=$(echo "$updates" | wc -l)
        
        # Check for RururuOS specific updates
        local rururu_updates
        rururu_updates=$(echo "$updates" | grep "^rururu-" || true)
        
        if [ -n "$rururu_updates" ]; then
            notify-send -u normal \
                "RururuOS Updates Available" \
                "$count packages can be updated, including RururuOS components" \
                -i system-software-update
        else
            notify-send -u low \
                "System Updates Available" \
                "$count packages can be updated" \
                -i system-software-update
        fi
        
        echo "$updates" > "${CACHE_DIR}/available-updates"
    fi
    
    date +%s > "$LAST_CHECK_FILE"
}

should_check() {
    if [ ! -f "$LAST_CHECK_FILE" ]; then
        return 0
    fi
    
    local last_check
    last_check=$(cat "$LAST_CHECK_FILE")
    local now
    now=$(date +%s)
    local diff=$((now - last_check))
    
    [ $diff -ge $CHECK_INTERVAL ]
}

main() {
    case "${1:-check}" in
        check)
            if should_check; then
                check_updates
            fi
            ;;
        force)
            check_updates
            ;;
        list)
            if [ -f "${CACHE_DIR}/available-updates" ]; then
                cat "${CACHE_DIR}/available-updates"
            else
                echo "No cached update information. Run: $0 force"
            fi
            ;;
        apply)
            sudo pacman -Syu
            rm -f "${CACHE_DIR}/available-updates"
            ;;
        *)
            echo "Usage: $0 {check|force|list|apply}"
            exit 1
            ;;
    esac
}

main "$@"
