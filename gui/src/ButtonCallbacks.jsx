import { invoke } from "@tauri-apps/api/core";
import { getGlobals } from "./Globals.jsx";
import { useRef } from "react";

function OkCancleKit(props) {
    const {
        query_window_utils: { disableQueryWindow },
        backend_state_utils: { setBackendState },
    } = getGlobals();

    const cancleCallback = props.cancleCallback
        ? props.cancleCallback
        : () => {
              setBackendState(false);
              disableQueryWindow();
          };

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
        backend_state_utils: { backendLocked, setBackendState },
    } = getGlobals();
    function error_handle(error) {
        const err_string = error.explanation.join("\n");
        setBackendState(false);
        pushMsg(err_string, error.isfatal ? 1 : 2);
    }

    function check_backend_state() {
        if (backendLocked) {
            pushMsg("有操作正在进行", 2);
            return false;
        } else {
            setBackendState(true);
            return true;
        }
    }

    function cmd_startgame() {
        if (check_backend_state()) {
            invoke("cmd_startgame").catch(error_handle);
        }
    }

    const noitaPathRef = useRef(null);
    function cmd_setpath() {
        if (check_backend_state()) {
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
    }

    function cmd_usage() {
        if (check_backend_state()) {
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
    }

    function cmd_log_history() {
        setStackState(!stackState);
        setBkgDisability(true);
    }

    const saveNameRef = useRef(null);
    const saveNoteRef = useRef(null);
    function cmd_save() {
        if (check_backend_state()) {
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
                            })
                                .then(update_save_infos)
                                .catch(error_handle);
                        }}
                    />
                </>,
            );
        }
    }

    function cmd_qsave() {
        if (check_backend_state()) {
            invoke("cmd_qsave").then(update_save_infos).catch(error_handle);
        }
    }

    function cmd_overwrite() {
        if (check_backend_state()) {
            invoke("cmd_overwrite").then(update_save_infos).catch(error_handle);
        }
    }

    function cmd_load() {
        if (check_backend_state()) {
            const indexs = getCheckedSaveIndexs();
            if (indexs.length == 0) {
                pushMsg("please choose at least one save", 2);
            } else if (indexs.length > 1) {
                pushMsg("please choose only one save", 2);
            } else {
                invoke("cmd_load", { indexs: indexs[0] }).catch(error_handle);
            }
        }
    }

    function cmd_qload() {
        if (check_backend_state()) {
            invoke("cmd_load").catch(error_handle);
        }
    }

    function cmd_delete() {
        if (check_backend_state()) {
            const indexs = getCheckedSaveIndexs();
            if (indexs.length == 0) {
                pushMsg("please choose at least one save", 2);
            } else {
                invoke("cmd_delete", { indexs: indexs })
                    .then(update_save_infos)
                    .catch(error_handle);
            }
        }
    }

    function cmd_qdelete() {
        if (check_backend_state()) {
            invoke("cmd_qdelete").then(update_save_infos).catch(error_handle);
        }
    }

    function cmd_modify_lock(operate) {
        if (check_backend_state()) {
            const indexs = getCheckedSaveIndexs();
            if (indexs.length == 0) {
                pushMsg("please choose at least one save", 2);
            } else {
                invoke("cmd_modify_lock", { indexs: indexs, operate })
                    .then(update_save_infos)
                    .catch(error_handle);
            }
        }
    }

    const newSaveNameRef = useRef(null);
    const newSaveNoteRef = useRef(null);
    function cmd_modify() {
        if (check_backend_state()) {
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
                                })
                                    .then(update_save_infos)
                                    .catch(error_handle);
                            }}
                        />
                    </>,
                );
            }
        }
    }

    return {
        // Utils
        cmd_startgame,
        cmd_setpath,
        cmd_usage,
        cmd_log_history,
        // Save
        cmd_save,
        cmd_qsave,
        cmd_overwrite,
        // Load
        cmd_load,
        cmd_qload,
        // Delete
        cmd_delete,
        cmd_qdelete,
        // Lock
        cmd_modify_lock,
        cmd_modify,
    };
}
