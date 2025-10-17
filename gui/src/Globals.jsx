import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createContext, useContext, useState, useRef, useEffect } from "react";
import "./assets/MessagePaneStyle.css";
import { useTranslation } from "react-i18next";

const Globals = createContext(null);

export function GlobalProvider({ children }) {
    const [stack, setMsgStack] = useState([]);
    // true: open log history
    const [stackState, setStackState] = useState(false);
    const [saveInfos, setInfos] = useState([]);
    const msgIdRef = useRef(0);
    const [isBkgDisabled, setBkgDisability] = useState(false);
    const [queryWindowState, setQueryWindowState] = useState({
        enabled: false,
        child: null,
    });
    const [saveCheckboxState, setCheckboxState] = useState([]);
    const [backendLocked, setBackendState] = useState(false);
    const { i18n } = useTranslation();

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
        const id = msgIdRef.current++;

        setMsgStack((prevStack) => [
            ...prevStack,
            { content, is_showing: true, log_grade, msg_id: id },
        ]);

        setTimeout(() => msgBoxDisappear(id, false), 2000);
    }

    async function update_save_infos() {
        setInfos(await invoke("get_saves"));
        setCheckboxState(Array(saveInfos.length).fill(false));
    }

    function enableQueryWindow(title, children) {
        setQueryWindowState({
            enabled: true,
            child: (
                <>
                    <p className="message_pane_title">{title}</p>
                    {children}
                </>
            ),
        });
        setBkgDisability(true);
    }
    function disableQueryWindow() {
        setQueryWindowState({
            enabled: false,
            child: null,
        });
        setBkgDisability(false);
    }

    function getCheckedSaveIndexs() {
        console.log(saveCheckboxState);
        return saveCheckboxState
            .map((ref, index) => (ref ? index : null))
            .filter((index) => index !== null);
    }

    useEffect(() => {
        update_save_infos();

        const unlistenBackendLog = listen("backend_log", (event) => {
            pushMsg(event.payload.message, event.payload.log_grade);
        });

        const unlistenRelease = listen("release_backend_lock", () => {
            setBackendState(false);
        });

        invoke("get_locale").then((locale) => i18n.changeLanguage(locale));

        // React Strict Mode will cause the Components to be mounted twice
        // which will register two listener
        // So its necessary to clean the previous listeners with the following code:
        return () => {
            unlistenBackendLog.then((f) => f());
            unlistenRelease.then((f) => f());
        };
    }, []);
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
                query_window_utils: {
                    queryWindowState,
                    setQueryWindowState,
                    enableQueryWindow,
                    disableQueryWindow,
                },
                save_checkbox_utils: {
                    saveCheckboxState,
                    setCheckboxState,
                    getCheckedSaveIndexs,
                },
                backend_state_utils: {
                    backendLocked,
                    setBackendState,
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
