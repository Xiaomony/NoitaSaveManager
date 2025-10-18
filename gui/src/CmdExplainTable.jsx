import { useTranslation } from "react-i18next";

export default function CmdExplainTable() {
    const { t: t_cmdname } = useTranslation("CommandNames");
    const { t: t_cmdexp } = useTranslation("CommandExplanation");
    const { t } = useTranslation("common");

    const commandKeys = [
        "startgame",
        "setpath",
        "usage",
        "loghistory",
        "instruction",
        "github_link",
        "save",
        "qsave",
        "overwrite",
        "autosave",
        "load",
        "qload",
        "delete",
        "qdelete",
        "lock",
        "unlock",
        "modify",
    ];

    return (
        <table className="my_table" style={{ width: "70%" }} align="center">
            <thead>
                <tr>
                    <th
                        style={{
                            width: "30%",
                        }}
                    >
                        {t("ins_cmdname_title")}
                    </th>
                    <th
                        style={{
                            width: "70%",
                        }}
                    >
                        {t("ins_cmdexp_title")}
                    </th>
                </tr>
            </thead>
            <tbody>
                {commandKeys.map((key) => (
                    <tr key={key}>
                        <td align="center">{t_cmdname(key)}</td>
                        <td align="center">{t_cmdexp(key)}</td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}
