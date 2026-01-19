import QtQuick 2.15
import QtQuick.Layouts 1.15
import org.kde.plasma.plasmoid 2.0
import org.kde.plasma.core 2.0 as PlasmaCore
import org.kde.plasma.components 3.0 as PlasmaComponents
import org.kde.kirigami 2.20 as Kirigami

PlasmoidItem {
    id: root
    
    preferredRepresentation: compactRepresentation
    
    property var apps: [
        { name: "GIMP", icon: "gimp", command: "gimp", shortcut: "Super+G" },
        { name: "Inkscape", icon: "inkscape", command: "inkscape", shortcut: "Super+I" },
        { name: "Krita", icon: "krita", command: "krita", shortcut: "Super+P" },
        { name: "Blender", icon: "blender", command: "blender", shortcut: "Super+Shift+B" },
        { name: "Kdenlive", icon: "kdenlive", command: "kdenlive", shortcut: "Super+Shift+K" },
        { name: "Ardour", icon: "ardour", command: "ardour7", shortcut: "Super+Shift+A" },
        { name: "Darktable", icon: "darktable", command: "darktable", shortcut: "" },
        { name: "FreeCAD", icon: "freecad", command: "freecad", shortcut: "" }
    ]
    
    compactRepresentation: PlasmaComponents.ToolButton {
        icon.name: "applications-graphics"
        onClicked: root.expanded = !root.expanded
        
        PlasmaComponents.ToolTip {
            text: "Creative Apps"
        }
    }
    
    fullRepresentation: ColumnLayout {
        Layout.minimumWidth: Kirigami.Units.gridUnit * 12
        Layout.minimumHeight: Kirigami.Units.gridUnit * 16
        
        PlasmaComponents.Label {
            text: "Creative Apps"
            font.bold: true
            Layout.alignment: Qt.AlignHCenter
            Layout.topMargin: Kirigami.Units.smallSpacing
        }
        
        ListView {
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: root.apps
            spacing: Kirigami.Units.smallSpacing
            
            delegate: PlasmaComponents.ItemDelegate {
                width: parent.width
                
                contentItem: RowLayout {
                    Kirigami.Icon {
                        source: modelData.icon
                        Layout.preferredWidth: Kirigami.Units.iconSizes.medium
                        Layout.preferredHeight: Kirigami.Units.iconSizes.medium
                    }
                    
                    ColumnLayout {
                        spacing: 0
                        
                        PlasmaComponents.Label {
                            text: modelData.name
                            Layout.fillWidth: true
                        }
                        
                        PlasmaComponents.Label {
                            text: modelData.shortcut || "No shortcut"
                            font.pointSize: Kirigami.Theme.smallFont.pointSize
                            opacity: 0.7
                            Layout.fillWidth: true
                        }
                    }
                }
                
                onClicked: {
                    executable.exec(modelData.command)
                    root.expanded = false
                }
            }
        }
        
        PlasmaComponents.Button {
            text: "Open All"
            icon.name: "system-run"
            Layout.fillWidth: true
            Layout.bottomMargin: Kirigami.Units.smallSpacing
            
            onClicked: {
                for (var i = 0; i < root.apps.length; i++) {
                    executable.exec(root.apps[i].command)
                }
                root.expanded = false
            }
        }
    }
    
    PlasmaCore.DataSource {
        id: executable
        engine: "executable"
        connectedSources: []
        
        function exec(cmd) {
            connectSource(cmd)
        }
        
        onNewData: {
            disconnectSource(sourceName)
        }
    }
}
