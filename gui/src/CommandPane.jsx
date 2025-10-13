import "./assets/CmdPaneStyle.css";
import useButtonCb from "./ButtonCallbacks.jsx";
import { getGlobals } from "./Globals.jsx";

function CommandPane(props) {
    const {
        bkg_disability_utils: { isBkgDisabled },
    } = getGlobals();
    const { cmd_startgame, cmd_setpath, cmd_usage, cmd_log_history, cmd_save } =
        useButtonCb();

    return (
        <div
            className={`${props.className || ""} ${isBkgDisabled ? "disabled" : ""}`}
            id="cmd_pane"
            // style={{ filter: stackState ? "blur(5px)" : null }}
        >
            <div
                id="buttons_utils"
                className="button_container"
                style={{ gridArea: "A" }}
            >
                <button type="button" onClick={cmd_startgame}>
                    Start Noita
                </button>
                <button type="button" onClick={cmd_setpath}>
                    Set Noita Path
                </button>
                <button type="button" onClick={cmd_usage}>
                    Usage
                </button>
                <button type="button" onClick={cmd_log_history}>
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
