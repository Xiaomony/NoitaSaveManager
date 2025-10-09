import "./assets/CmdPaneStyle.css";

function CommandPane(props) {
    return (
        <div className={props.className} id="cmd_pane">
            <button
                type="button"
                id="button_log_history"
                style={{ gridArea: "A" }}
            >
                Log History
            </button>
            <button
                type="button"
                id="button_start_noita"
                style={{ gridArea: "B" }}
            >
                Start Noita
            </button>
            <button
                type="button"
                id="button_set_path"
                style={{ gridArea: "C" }}
            >
                Set Noita Path
            </button>

            <div
                id="buttons_save"
                className="button_container"
                style={{ gridArea: "D" }}
            >
                <span>Save</span>
                <button type="button" id="button_save">
                    Save
                </button>
                <button type="button" id="button_qsave">
                    Quick Save
                </button>
                <button type="button" id="button_overwrite">
                    Overwrite
                </button>
                <button type="button" id="button_autosave">
                    Auto Save
                </button>
            </div>

            <div
                id="buttons_load"
                className="button_container"
                style={{ gridArea: "E" }}
            >
                <span>Load</span>
                <button type="button" id="button_load">
                    Load
                </button>
                <button type="button" id="button_qload">
                    Quick Load
                </button>
            </div>

            <div
                id="buttons_delete"
                className="button_container"
                style={{ gridArea: "F" }}
            >
                <span>Delete</span>
                <button type="button" id="button_delete">
                    Delete
                </button>
                <button type="button" id="button_qdelete">
                    Quick Delete
                </button>
            </div>

            <div
                id="buttons_modify"
                className="button_container"
                style={{ gridArea: "G" }}
            >
                <span>Modify</span>
                <button type="button" id="button_lock">
                    Lock
                </button>
                <button type="button" id="button_unlock">
                    Unlock
                </button>
                <button type="button" id="button_modify">
                    Modify
                </button>
            </div>

            <button type="button" id="button_usage" style={{ gridArea: "H" }}>
                Usage
            </button>
        </div>
    );
}

export default CommandPane;
