import { createContext, useContext, useState } from "react";

const Globals = createContext(null);
export let msg_stack = null;
export let setMsgStack = null;
export let stack_state = null;
export let setStackState = null;

export function GlobalProvider({ children }) {
    [msg_stack, setMsgStack] = useState([]);
    // true: open log history
    [stack_state, setStackState] = useState(false);
    return (
        <Globals.Provider
            value={[
                [msg_stack, setMsgStack],
                [stack_state, setStackState],
            ]}
        >
            {children}
        </Globals.Provider>
    );
}

export function getGlobals() {
    return useContext(Globals);
}

export function clearMsgStack() {
    setMsgStack([]);
}
