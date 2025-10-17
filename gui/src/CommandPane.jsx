import { useTranslation } from "react-i18next";
import "./assets/CmdPaneStyle.css";
import useButtonCb from "./ButtonCallbacks.jsx";
import { getGlobals } from "./Globals.jsx";

function CommandPane(props) {
    const { t } = useTranslation("CommandNames");

    const {
        bkg_disability_utils: { isBkgDisabled },
    } = getGlobals();
    const {
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
    } = useButtonCb();

    return (
        <div
            className={`${props.className || ""} ${isBkgDisabled ? "disabled" : ""}`}
            id="cmd_pane"
        >
            <div
                id="buttons_utils"
                className="button_container"
                style={{ gridArea: "A" }}
            >
                <button type="button" onClick={cmd_startgame}>
                    {t("startgame")}
                </button>
                <button type="button" onClick={cmd_setpath}>
                    {t("setpath")}
                </button>
                <button type="button" onClick={cmd_usage}>
                    {t("usage")}
                </button>
                <button type="button" onClick={cmd_log_history}>
                    {t("loghistory")}
                </button>
                <button type="button" onClick={cmd_instruction}>
                    {t("instruction")}
                </button>
                <button type="button" onClick={cmd_github_link}>
                    {t("github_link")}
                </button>
            </div>

            <div
                id="buttons_save"
                className="button_container"
                style={{ gridArea: "D" }}
            >
                <span>{t("op_class.save")}</span>
                <button type="button" onClick={cmd_save}>
                    {t("save")}
                </button>
                <button type="button" onClick={cmd_qsave}>
                    {t("qsave")}
                </button>
                <button type="button" onClick={cmd_overwrite}>
                    {t("overwrite")}
                </button>
                <button type="button">{t("autosave")}</button>
            </div>

            <div
                id="buttons_load"
                className="button_container"
                style={{ gridArea: "E" }}
            >
                <span>{t("op_class.load")}</span>
                <button type="button" onClick={cmd_load}>
                    {t("load")}
                </button>
                <button type="button" onClick={cmd_qload}>
                    {t("qload")}
                </button>
            </div>

            <div
                id="buttons_delete"
                className="button_container"
                style={{ gridArea: "F" }}
            >
                <span>{t("op_class.delete")}</span>
                <button type="button" onClick={cmd_delete}>
                    {t("delete")}
                </button>
                <button type="button" onClick={cmd_qdelete}>
                    {t("qdelete")}
                </button>
            </div>

            <div
                id="buttons_modify"
                className="button_container"
                style={{ gridArea: "G" }}
            >
                <span>{t("op_class.modify")}</span>
                <button
                    type="button"
                    onClick={() => {
                        cmd_modify_lock(true);
                    }}
                >
                    {t("lock")}
                </button>
                <button
                    type="button"
                    onClick={() => {
                        cmd_modify_lock(false);
                    }}
                >
                    {t("unlock")}
                </button>
                <button type="button" onClick={cmd_modify}>
                    {t("modify")}
                </button>
            </div>
        </div>
    );
}

export default CommandPane;
