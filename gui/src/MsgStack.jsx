import FloatPane from "./FloatPane.jsx";
import { motion, AnimatePresence } from "framer-motion";
import {
    msg_stack,
    setMsgStack,
    // stack_state,
    // setStackState,
    getGlobals,
    clearMsgStack,
    setStackState,
} from "./Globals.jsx";

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

let msg_id = 0;

export function pushMsg(content, log_grade) {
    setMsgStack([
        ...msg_stack,
        {
            content: content,
            is_showing: true,
            log_grade: log_grade,
            msg_id: msg_id,
        },
    ]);

    setTimeout(msgBoxDisappear, 5000, msg_id);

    msg_id++;
}

/*
message object:
{
    content: "",
    is_displaying: bool,
    log_grade: 1 2 3 4 5,        // fatal  warning  log  log_green  debug
    msg_id: 0 1 3 4 ...
}
 */

export default function MsgStack() {
    const [[stack, ,], [stackState, ,]] = getGlobals();
    function msg_mapper(item) {
        let color = null;
        let title = null;
        switch (item.log_grade) {
            case 1:
                title = "[FATAL]";
                color = "#e06c75";
                break;
            case 2:
                title = "[WARNING]";
                color = "#e5c07b";
                break;
            case 3:
                title = "[LOG]";
                color = "#56b6c2";
                break;
            case 4:
                title = "[LOG]";
                color = "#98c379";
                break;
            case 5:
                title = "[DEBUG]";
                color = "#c678dd";
                break;
        }
        return (
            <motion.div
                key={item.msg_id}
                layout
                initial={{ x: 200, opacity: 0 }}
                animate={{ x: 0, opacity: 1 }}
                exit={{ x: 200, opacity: 0 }}
                transition={{
                    type: "spring",
                    stiffness: 300,
                    damping: 20,
                    layout: { duration: 0.3 },
                }}
                drag="x"
                dragConstraints={{ left: 0, right: 200 }}
                dragElastic={0.3}
                onDragEnd={(_, info) => {
                    if (info.offset.x > 30) {
                        msgBoxDisappear(item.msg_id, stackState);
                    }
                }}
            >
                <FloatPane className="msgbox" title={title} color={color}>
                    {item.content}
                </FloatPane>
            </motion.div>
        );
    }
    const messages = stackState
        ? stack.map(msg_mapper)
        : stack.filter((item) => item.is_showing).map(msg_mapper);
    if (stackState) {
        messages.reverse();
    }

    return (
        <AnimatePresence>
            {stackState ? (
                <motion.div
                    className="log_history_stack pane"
                    key={-1}
                    initial={{ y: +500, opacity: 0 }}
                    animate={{ x: "-50%", y: "-50%", opacity: 1 }}
                    exit={{ opacity: 0 }}
                    transition={{
                        type: "spring",
                        stiffness: 300,
                        damping: 20,
                    }}
                >
                    <div
                        style={{
                            display: "flex",
                            justifyContent: "space-evenly",
                        }}
                    >
                        <button
                            type="button"
                            style={{ width: "45%", height: "50px" }}
                            onClick={clearMsgStack}
                        >
                            Clear History
                        </button>
                        <button
                            type="button"
                            style={{ width: "45%", height: "50px" }}
                            onClick={() => setStackState(false)}
                        >
                            Close
                        </button>
                    </div>
                    {messages.length == 0 ? <p>No history</p> : messages}
                </motion.div>
            ) : (
                <div className="msg_stack">
                    <AnimatePresence>{messages}</AnimatePresence>
                </div>
            )}
        </AnimatePresence>
    );
}
