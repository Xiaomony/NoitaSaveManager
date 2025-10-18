import { useCallback } from "react";
import "./assets/TableStyle.css";
import { getGlobals } from "./Globals.jsx";
import { useTranslation } from "react-i18next";

function MyCheckbox({ checked }) {
    return checked ? <span>✔</span> : null;
}

function SavesPane(props) {
    const {
        save_info_utils: { saveInfos },
        bkg_disability_utils: { isBkgDisabled },
        save_checkbox_utils: { saveCheckboxState, setCheckboxState },
    } = getGlobals();
    const { t } = useTranslation("common");

    const toggleRow = useCallback((i) => {
        setCheckboxState((prev) => {
            const newState = [...prev];
            newState[i] = prev[i] === undefined ? true : !prev[i];
            return newState;
        });
    }, []);

    return (
        <div
            className={`${props.className || ""} ${isBkgDisabled ? "disabled" : ""}`}
            id="saves_pane"
        >
            <table className="my_table">
                <caption>
                    {t("savepane.title")}
                    <br />
                    {t("savepane.notification")}
                </caption>
                <colgroup>
                    <col style={{ width: "7%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "28%" }} />
                    <col style={{ width: "35%" }} />
                </colgroup>
                <thead>
                    <tr>
                        <th scope="col">{t("savepane.select")}</th>
                        <th scope="col">{t("savepane.date")}</th>
                        <th scope="col">{t("savepane.time")}</th>
                        <th scope="col">{t("savepane.name")}</th>
                        <th scope="col">{t("savepane.note")}</th>
                    </tr>
                </thead>
                <tbody>
                    {saveInfos.map((save, i) => {
                        const isSelected = saveCheckboxState[i];
                        return (
                            <tr
                                key={save.m_name}
                                className={
                                    save.m_islocked ? "locked_save" : null
                                }
                                onClick={() => toggleRow(i)}
                                style={{ cursor: "pointer" }}
                            >
                                <td scope="col">
                                    <MyCheckbox checked={isSelected} />
                                </td>
                                <td scope="col">{save.m_date}</td>
                                <td scope="col">{save.m_time}</td>
                                <td scope="col">{save.m_name}</td>
                                <td scope="col">{save.m_note}</td>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </div>
    );
}

export default SavesPane;
