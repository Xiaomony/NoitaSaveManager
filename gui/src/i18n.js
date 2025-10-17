import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';

// ----------------------------------------------
import zhCN from './locales/zh-CN.json';

i18n.use(initReactI18next).init({
    lng : "en-US",
    fallbackLng : [ 'en-US', 'en-GB', 'zh-CN', 'zh-TW', 'ja-JP' ],

    // debug : true,
    interpolation : {
        escapeValue : false,
    },

    resources : {"zh-CN" : zhCN, "zh-TW" : {}, "en-US" : {}, "en-GB" : {}, "ja-JP" : {}}
});

export default i18n;
