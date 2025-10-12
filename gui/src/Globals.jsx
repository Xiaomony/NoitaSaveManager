import { invoke } from "@tauri-apps/api/core";
import { createContext, useContext, useState } from "react";

const Globals = createContext(null);

export function GlobalProvider({ children }) {
    const [stack, setMsgStack] = useState([]);
    // true: open log history
    const [stackState, setStackState] = useState(false);
    const [saveInfos, setInfos] = useState([]);

    const [msg_id, setMsgId] = useState(0);
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
                msg_id: msg_id,
            },
        ]);

        setTimeout(msgBoxDisappear, 2000, msg_id, false);

        setMsgId(msg_id + 1);
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
            }}
        >
            {children}
        </Globals.Provider>
    );
}

export function getGlobals() {
    return useContext(Globals);
}
