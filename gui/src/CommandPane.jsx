import "./assets/CmdPaneStyle.css";
import { getGlobals } from "./Globals.jsx";
import { invoke } from "@tauri-apps/api/core";
import { add_listeners } from "./msgListener.jsx";

function CommandPane(props) {
    const {
        msg_stack_utils: { pushMsg },
        stack_state_utils: { stackState, setStackState },
        save_info_utils: { update_save_infos },
    } = getGlobals();

    function error_handle(error) {
        pushMsg(error.explanation, error.isfatal ? 1 : 2);
    }
    function cmd_save() {
        invoke("cmd_save", { name: "aaa", note: "bbb" }).catch(error_handle);
        update_save_infos();
    }
    add_listeners(pushMsg);

    return (
        <div
            className={`${props.className || ""} ${stackState ? "disabled" : ""}`}
            id="cmd_pane"
            style={{ filter: stackState ? "blur(5px)" : null }}
        >
            <div
                id="buttons_utils"
                className="button_container"
                style={{ gridArea: "A" }}
            >
                <button type="button">Start Noita</button>
                <button type="button">Set Noita Path</button>
                <button type="button">Usage</button>
                <button
                    type="button"
                    onClick={() => setStackState(!stackState)}
                >
                    Log History
                </button>
                <button type="button">Instructions</button>
                <button type="button">GitHub Link</button>
            </div>

            <div
                id="buttons_save"
                className="button_container"
                style={{ gridArea: "D" }}
            >
                <span>Save</span>
                <button type="button" onClick={cmd_save}>
                    Save
                </button>
                <button type="button">Quick Save</button>
                <button type="button">Overwrite</button>
                <button type="button">Auto Save</button>
            </div>

            <div
                id="buttons_load"
                className="button_container"
                style={{ gridArea: "E" }}
            >
                <span>Load</span>
                <button type="button">Load</button>
                <button type="button">Quick Load</button>
            </div>

            <div
                id="buttons_delete"
                className="button_container"
                style={{ gridArea: "F" }}
            >
                <span>Delete</span>
                <button type="button">Delete</button>
                <button type="button">Quick Delete</button>
            </div>

            <div
                id="buttons_modify"
                className="button_container"
                style={{ gridArea: "G" }}
            >
                <span>Modify</span>
                <button type="button">Lock</button>
                <button type="button">Unlock</button>
                <button type="button">Modify</button>
            </div>
        </div>
    );
}

export default CommandPane;
