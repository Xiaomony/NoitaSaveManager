import "./assets/TableStyle.css";
import { getGlobals } from "./Globals.jsx";

function SavesPane(props) {
    const {
        save_info_utils: { saveInfos },
        bkg_disability_utils: { isBkgDisabled },
        save_checkbox_utils: { saveCheckboxState },
    } = getGlobals();

    return (
        <div
            className={`${props.className || ""} ${isBkgDisabled ? "disabled" : ""}`}
            id="saves_pane"
        >
            <table className="saves_table">
                <caption>Saves Information</caption>
                <colgroup>
                    <col style={{ width: "7%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "15%" }} />
                    <col style={{ width: "28%" }} />
                    <col style={{ width: "35%" }} />
                </colgroup>
                <thead>
                    <tr>
                        <th scope="col">Select</th>
                        <th scope="col">Date</th>
                        <th scope="col">Time</th>
                        <th scope="col">Name</th>
                        <th scope="col">Note</th>
                    </tr>
                </thead>
                <tbody>
                    {saveInfos.map((save, i) => {
                        return (
                            <tr
                                key={save.m_name}
                                className={
                                    save.m_islocked ? "locked_save" : null
                                }
                            >
                                <th scope="col">
                                    <input
                                        type="checkbox"
                                        ref={(el) => {
                                            saveCheckboxState.current[i] = el;
                                        }}
                                    />
                                </th>
                                <th scope="col">{save.m_date}</th>
                                <th scope="col">{save.m_time}</th>
                                <th scope="col">{save.m_name}</th>
                                <th scope="col">{save.m_note}</th>
                            </tr>
                        );
                    })}
                </tbody>
            </table>
        </div>
    );
}

export default SavesPane;
