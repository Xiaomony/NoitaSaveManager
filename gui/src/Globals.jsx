import { invoke } from "@tauri-apps/api/core";
import { createContext, useContext, useState } from "react";

const Globals = createContext(null);

export function GlobalProvider({ children }) {
    const [stack, setMsgStack] = useState([]);
    // true: open log history
    const [stackState, setStackState] = useState(false);
    const [saveInfos, setInfos] = useState([]);
    const [msgId, setMsgId] = useState(0);
    const [isBkgDisabled, setBkgDisability] = useState(false);

    function msgBoxDisappear(id, is_delete) {
        setMsgStack((msg_stack) => {
            if (is_delete) {
                return msg_stack.filter((item) => item.msg_id != id);
            } else {
                return msg_stack.map((item) =>
                    item.msg_id == id ? { ...item, is_showing: false } : item,
                );
            }
        });
    }
    function pushMsg(content, log_grade) {
        setMsgStack([
            ...stack,
            {
                content: content,
                is_showing: true,
                log_grade: log_grade,
                msg_id: msgId,
            },
        ]);

        setTimeout(msgBoxDisappear, 2000, msgId, false);

        setMsgId(msgId + 1);
    }

    async function update_save_infos() {
        setInfos(await invoke("get_saves"));
    }

    update_save_infos();

    return (
        <Globals.Provider
            value={{
                msg_stack_utils: {
                    stack,
                    setMsgStack,
                    pushMsg,
                    msgBoxDisappear,
                },
                stack_state_utils: {
                    stackState,
                    setStackState,
                },
                save_info_utils: {
                    saveInfos,
                    update_save_infos,
                },
                bkg_disability_utils: {
                    isBkgDisabled,
                    setBkgDisability,
                },
            }}
        >
            {children}
        </Globals.Provider>
    );
}

export function getGlobals() {
    return useContext(Globals);
}
