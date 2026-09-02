import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Quickshell
import Quickshell.Wayland

ShellWindow {
    id: root

    WlrLayershell.namespace: "avro-bar"
    WlrLayershell.layer: WlrLayer.Top

    color: "#e0e0e0"
    width: 300
    height: 40

    anchors {
        top: true
        bottom: false
        left: false
        right: false
    }

    margins.top: 10
    exclusionMode: ExclusionMode.Ignore

    property string currentMode: "EN"
    property string currentLayout: "Avro Phonetic"

    Process {
        id: monitorProc
        command: ["avro-cli", "monitor"]
        running: true

        onStdoutChanged: {
            var lines = stdout.trim().split("\n")
            for (var i = 0; i < lines.length; i++) {
                if (lines[i].length > 0) {
                    try {
                        var state = JSON.parse(lines[i])
                        if (state.mode) root.currentMode = state.mode
                        if (state.layout) root.currentLayout = state.layout
                    } catch (e) {
                        console.log("Error parsing monitor output:", e)
                    }
                }
            }
        }
    }

    Process {
        id: toggleProc
        command: ["avro-cli", "toggle"]
    }

    Process {
        id: setLayoutProc
        property string layoutName: ""
        command: ["avro-cli", "set-layout", layoutName]
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 5
        spacing: 10

        Rectangle {
            Layout.preferredWidth: 60
            Layout.fillHeight: true
            color: root.currentMode === "BN" ? "#4caf50" : "#f44336"
            radius: 5

            Text {
                anchors.centerIn: parent
                text: root.currentMode === "BN" ? "বাং" : "EN"
                color: "white"
                font.bold: true
            }

            MouseArea {
                anchors.fill: parent
                onClicked: toggleProc.running = true
            }
        }

        ComboBox {
            Layout.fillWidth: true
            Layout.fillHeight: true
            model: ["Avro Phonetic", "Probhat", "Munir", "National (Jatiya)"]
            currentIndex: model.indexOf(root.currentLayout) !== -1 ? model.indexOf(root.currentLayout) : 0

            onActivated: (index) => {
                setLayoutProc.layoutName = model[index]
                setLayoutProc.running = true
            }
        }

        ToolButton {
            Layout.preferredWidth: 50
            Layout.fillHeight: true
            text: "⚙"

            onClicked: toolsMenu.open()

            Menu {
                id: toolsMenu
                y: parent.height

                MenuItem {
                    text: "Settings..."
                }
                MenuItem {
                    text: "Phonetic Help Sheet"
                }
                MenuItem {
                    text: "About"
                }
            }
        }
    }
}
