import { invoke } from "@tauri-apps/api/core";
import { getGlobals } from "./Globals.jsx";
import { useRef } from "react";

function OkCancleKit(props) {
    const {
        query_window_utils: { disableQueryWindow },
    } = getGlobals();

    const cancleCallback = props.cancleCallback
        ? props.cancleCallback
        : disableQueryWindow;

    return (
        <div
            style={{
                display: "flex",
                gap: "15px",
                justifyContent: "space-around",
                margin: 0,
                padding: 0,
                height: "50px",
            }}
        >
            <button
                type="button"
                onClick={() => {
                    if (props.okCallback) {
                        props.okCallback();
                    }
                    disableQueryWindow();
                }}
                style={{ width: "35%" }}
            >
                Confirm
            </button>
            <button
                type="button"
                onClick={cancleCallback}
                style={{ width: "35%" }}
            >
                Cancle
            </button>
        </div>
    );
}

export default function useButtonCb() {
    const {
        msg_stack_utils: { pushMsg },
        stack_state_utils: { stackState, setStackState },
        save_info_utils: { update_save_infos },
        bkg_disability_utils: { setBkgDisability },
        query_window_utils: { enableQueryWindow },
    } = getGlobals();
    function error_handle(error) {
        pushMsg(error.explanation, error.isfatal ? 1 : 2);
    }

    function cmd_startgame() {
        invoke("cmd_startgame").catch(error_handle);
    }

    const noitaPathRef = useRef(null);
    function cmd_setpath() {
        enableQueryWindow(
            "Set noita.exe path",
            <>
                <input
                    ref={noitaPathRef}
                    placeholder="Input the path of noita.exe(end with 'noita.exe')"
                />
                <OkCancleKit />
            </>,
        );
    }

    function cmd_usage() {
        invoke("cmd_usage")
            .then((usage) => {
                const msg =
                    usage <= 1024
                        ? `${usage.toFixed(2)} MB`
                        : `${(usage / 1024).toFixed(2)} GB`;
                pushMsg(msg, 4);
            })
            .catch(error_handle());
    }

    function cmd_log_history() {
        setStackState(!stackState);
        setBkgDisability(true);
    }

    function cmd_save() {
        invoke("cmd_save", { name: "aaa", note: "bbb" }).catch(error_handle);
        update_save_infos();
    }

    return {
        cmd_startgame,
        cmd_setpath,
        cmd_usage,
        cmd_log_history,
        cmd_save,
    };
}
