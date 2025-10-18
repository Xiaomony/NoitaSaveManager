import { invoke } from "@tauri-apps/api/core";
import { getGlobals } from "./Globals.jsx";
import { useRef } from "react";
import { useTranslation } from "react-i18next";
import { openUrl } from "@tauri-apps/plugin-opener";
import CmdExplainTable from "./CmdExplainTable.jsx";

function OkCancleKit(props) {
    const {
        query_window_utils: { disableQueryWindow },
        backend_state_utils: { setBackendState },
    } = getGlobals();

    const { t } = useTranslation("common");

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
                {t("ok")}
            </button>
            <button
                type="button"
                onClick={cancleCallback}
                style={{ width: "35%" }}
            >
                {t("cancle")}
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
        query_window_utils: { enableQueryWindow, disableQueryWindow },
        save_checkbox_utils: { getCheckedSaveIndexs },
        backend_state_utils: { backendLocked, setBackendState },
    } = getGlobals();
    const { t } = useTranslation("common");

    function error_handle(error) {
        const err_string =
            typeof error.explanation === "object"
                ? error.explanation.join("\n")
                : error.explanation;
        setBackendState(false);
        pushMsg(err_string, error.isfatal ? 1 : 2);
    }

    function operation_success() {
        pushMsg(t("succeed"), 3);
    }

    function check_backend_state() {
        if (backendLocked) {
            pushMsg(t("message.backend_locked"), 2);
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
                t("setpath_title"),
                <>
                    <input
                        type="text"
                        ref={noitaPathRef}
                        placeholder={t("setpath_placeholder")}
                    />
                    <OkCancleKit
                        okCallback={() => {
                            invoke("cmd_setpath", {
                                newPath: noitaPathRef.current.value,
                            })
                                .then(operation_success)
                                .catch(error_handle);
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

    function cmd_instruction() {
        enableQueryWindow(
            t("instruction_title"),
            <>
                <p style={{ whiteSpace: "pre", margin: 0 }}>
                    {t("instruction")}
                </p>
                <CmdExplainTable />
                <button
                    type="button"
                    onClick={disableQueryWindow}
                    style={{
                        width: "20%",
                        minHeight: "50px",
                        flexShrink: 0,
                        alignSelf: "center",
                    }}
                >
                    {t("close")}
                </button>
            </>,
        );
    }

    function cmd_github_link() {
        openUrl("https://github.com/Xiaomony/NoitaSaveManager");
    }

    const saveNameRef = useRef(null);
    const saveNoteRef = useRef(null);
    function cmd_save() {
        if (check_backend_state()) {
            enableQueryWindow(
                t("save_title"),
                <>
                    <input
                        type="text"
                        ref={saveNameRef}
                        placeholder={t("savename_placeholder")}
                    />
                    <input
                        type="text"
                        ref={saveNoteRef}
                        placeholder={t("savenote_placeholder")}
                    />
                    <OkCancleKit
                        okCallback={() => {
                            invoke("cmd_save", {
                                name: saveNameRef.current.value,
                                note: saveNoteRef.current.value,
                            })
                                .then(() => {
                                    update_save_infos();
                                    operation_success();
                                })
                                .catch(error_handle);
                        }}
                    />
                </>,
            );
        }
    }

    function cmd_qsave() {
        if (check_backend_state()) {
            invoke("cmd_qsave")
                .then(() => {
                    update_save_infos();
                    operation_success();
                })
                .catch(error_handle);
        }
    }

    function cmd_overwrite() {
        if (check_backend_state()) {
            invoke("cmd_overwrite")
                .then(() => {
                    update_save_infos();
                    operation_success();
                })
                .catch(error_handle);
        }
    }

    function cmd_load() {
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg(t("message.choose_a_save"), 2);
        } else if (indexs.length > 1) {
            pushMsg(t("message.choose_only_one_save"), 2);
        } else {
            if (check_backend_state()) {
                invoke("cmd_load", { index: indexs[0] })
                    .then(operation_success)
                    .catch(error_handle);
            }
        }
    }

    function cmd_qload() {
        if (check_backend_state()) {
            invoke("cmd_qload").then(operation_success).catch(error_handle);
        }
    }

    function cmd_delete() {
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg(t("message.choose_a_save"), 2);
        } else {
            if (check_backend_state()) {
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
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg(t("message.choose_a_save"), 2);
        } else {
            if (check_backend_state()) {
                invoke("cmd_modify_lock", { indexs: indexs, operate })
                    .then(update_save_infos)
                    .catch(error_handle);
            }
        }
    }

    const newSaveNameRef = useRef(null);
    const newSaveNoteRef = useRef(null);
    function cmd_modify() {
        const indexs = getCheckedSaveIndexs();
        if (indexs.length == 0) {
            pushMsg(t("message.choose_a_save"), 2);
        } else if (indexs.length > 1) {
            pushMsg(t("message.choose_only_one_save"), 2);
        } else {
            if (check_backend_state()) {
                enableQueryWindow(
                    t("modify_title"),
                    <>
                        <input
                            type="text"
                            ref={newSaveNameRef}
                            placeholder={t("newname_placeholder")}
                        />
                        <input
                            type="text"
                            ref={newSaveNoteRef}
                            placeholder={t("newnote_placeholder")}
                        />
                        <OkCancleKit
                            okCallback={() => {
                                invoke("cmd_modify", {
                                    index: indexs[0],
                                    newName: newSaveNameRef.current.value,
                                    newNote: newSaveNoteRef.current.value,
                                })
                                    .then(() => {
                                        update_save_infos();
                                        operation_success();
                                    })
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
        cmd_instruction,
        cmd_github_link,
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
