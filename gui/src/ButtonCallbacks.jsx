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
        save_checkbox_utils: { getCheckedSaveIndexs },
    } = getGlobals();
    function error_handle(error) {
        console.log(error);
        pushMsg(error, error.isfatal ? 1 : 2);
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
                    type="text"
                    ref={noitaPathRef}
                    placeholder="Input the path of noita.exe(end with 'noita.exe')"
                />
                <OkCancleKit
                    okCallback={() => {
                        invoke("cmd_setpath", {
                            newPath: noitaPathRef.current.value,
                        }).catch(error_handle);
                    }}
                />
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
            .catch(error_handle);
    }

    function cmd_log_history() {
        setStackState(!stackState);
        setBkgDisability(true);
    }

    const saveNameRef = useRef(null);
    const saveNoteRef = useRef(null);
    function cmd_save() {
        enableQueryWindow(
            "Save",
            <>
                <input
                    type="text"
                    ref={saveNameRef}
                    placeholder="Save name(can't be empty)"
                />
                <input
                    type="text"
                    ref={saveNoteRef}
                    placeholder="Save note(can be empty)"
                />
                <OkCancleKit
                    okCallback={() => {
                        invoke("cmd_save", {
                            name: saveNameRef.current.value,
                            note: saveNoteRef.current.value,
                        }).catch(error_handle);
                    }}
                />
            </>,
        );
        update_save_infos();
    }

    function cmd_qsave() {
        invoke("cmd_qsave").catch(error_handle);
        update_save_infos();
    }

    function cmd_overwrite() {
        invoke("cmd_overwrite").catch(error_handle);
    }

    function cmd_qdelete() {
        invoke("cmd_qdelete").catch(error_handle);
    }

    function cmd_modify_lock(operate) {
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg("please choose at least one save", 2);
        } else {
            invoke("cmd_modify_lock", { indexs: indexs, operate });
            update_save_infos();
        }
    }

    const newSaveNameRef = useRef(null);
    const newSaveNoteRef = useRef(null);
    function cmd_modify() {
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg("please choose at least one save", 2);
        } else if (indexs.length > 1) {
            pushMsg("please choose only one save", 2);
        } else {
            enableQueryWindow(
                "Modify",
                <>
                    <input
                        type="text"
                        ref={newSaveNameRef}
                        placeholder="new save name(leave empty to keep the previous)"
                    />
                    <input
                        type="text"
                        ref={newSaveNoteRef}
                        placeholder="new save note(leave empty to keep the previous)"
                    />
                    <OkCancleKit
                        okCallback={() => {
                            invoke("cmd_modify", {
                                index: indexs[0],
                                newName: newSaveNameRef.current.value,
                                newNote: newSaveNoteRef.current.value,
                            }).catch(error_handle);
                        }}
                    />
                </>,
            );

            update_save_infos();
        }
    }

    return {
        cmd_startgame,
        cmd_setpath,
        cmd_usage,
        cmd_log_history,
        cmd_save,
        cmd_qsave,
        cmd_overwrite,
        cmd_qdelete,
        cmd_modify_lock,
        cmd_modify,
    };
}
