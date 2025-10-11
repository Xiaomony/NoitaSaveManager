import "./assets/CmdPaneStyle.css";
import { pushMsg } from "./MsgStack.jsx";

function cmd_save() {
    pushMsg(Math.random(), 1);
}

function CommandPane(props) {
    return (
        <div className={props.className} id="cmd_pane">
            <div
                id="buttons_utils"
                className="button_container"
                style={{ gridArea: "A" }}
            >
                <button type="button">Start Noita</button>
                <button type="button">Set Noita Path</button>
                <button type="button">Usage</button>
                <button type="button">Log History</button>
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
