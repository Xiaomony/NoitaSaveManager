import { listen } from "@tauri-apps/api/event";

export function add_listeners(pushMsg) {
    listen("warning", (event) => {
        pushMsg(event.payload.message, event.payload.log_grade);
    });
}
