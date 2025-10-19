import i18n from 'i18next';
import {initReactI18next} from 'react-i18next';

// ----------------------------------------------
import enGB from './locales/en-GB.json';
import enUS from './locales/en-US.json';
import jaJP from './locales/ja-JP.json';
import zhCN from './locales/zh-CN.json';
import zhTW from './locales/zh-TW.json';

i18n.use(initReactI18next).init({
    lng : "en-US",
    fallbackLng : [ 'en-US', 'en-GB', 'zh-CN', 'zh-TW', 'ja-JP' ],

    // debug : true,
    interpolation : {
        escapeValue : false,
    },

    resources : {"zh-CN" : zhCN, "zh-TW" : zhTW, "en-US" : enUS, "en-GB" : enGB, "ja-JP" : jaJP}
});

export default i18n;
