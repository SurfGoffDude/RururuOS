#!/bin/bash
# RururuOS Creative Workspace Setup Script
# Launches applications on appropriate workspaces

set -e

# Wait for sway to be ready
sleep 2

launch_on_workspace() {
    local workspace="$1"
    local app="$2"
    shift 2
    
    swaymsg "workspace $workspace; exec $app $@"
}

# Parse arguments
case "${1:-default}" in
    photo)
        # Photography workflow
        launch_on_workspace "2:  Edit" darktable
        launch_on_workspace "3:  Preview" "imv -f"
        ;;
    
    video)
        # Video editing workflow
        launch_on_workspace "2:  Edit" kdenlive
        launch_on_workspace "4:  Render" "foot -e htop"
        launch_on_workspace "5:  Audio" ardour
        ;;
    
    3d)
        # 3D modeling workflow
        launch_on_workspace "6:  3D" blender
        launch_on_workspace "2:  Edit" gimp
        ;;
    
    illustration)
        # Digital art workflow
        launch_on_workspace "2:  Edit" krita
        launch_on_workspace "3:  Preview" "imv -f"
        ;;
    
    design)
        # Graphic design workflow
        launch_on_workspace "2:  Edit" inkscape
        launch_on_workspace "2:  Edit" gimp
        ;;
    
    dev)
        # Development workflow
        launch_on_workspace "7:  Code" "code ."
        launch_on_workspace "8:  Web" firefox
        launch_on_workspace "7:  Code" "foot -e lazygit"
        ;;
    
    default)
        # Default creative setup
        launch_on_workspace "1:  Files" nautilus
        launch_on_workspace "2:  Edit" gimp
        launch_on_workspace "8:  Web" firefox
        ;;
    
    *)
        echo "Usage: $0 {photo|video|3d|illustration|design|dev|default}"
        exit 1
        ;;
esac

echo "Creative workspace setup complete!"
