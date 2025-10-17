import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';

// ----------------------------------------------
import zhCN_CommandNames from "./locales/zh-CN/CommandNames.json";
import zhCN_common from './locales/zh-CN/common.json';

i18n.use(initReactI18next).init({
    lng : "en-US",
    fallbackLng : [ 'en-US', 'en-GB', 'zh-CN', 'zh-TW', 'ja-JP' ],

    debug : true,
    interpolation : {
        escapeValue : false,
    },

    resources : {"zh-CN" : {CommandNames : zhCN_CommandNames, common : zhCN_common}}
});

export default i18n;
