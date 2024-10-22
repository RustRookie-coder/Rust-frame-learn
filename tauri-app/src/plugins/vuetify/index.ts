import {createVuetify} from "vuetify";
import {components, directives} from "vuetify/dist/vuetify";
import defaults from "./defaults.ts";
import theme from "./theme.ts";
import {useI18n} from "vue-i18n";
import i18n from "../i18n";
import DayJsAdapter from '@date-io/dayjs';
import {createVueI18nAdapter} from "vuetify/locale/adapters/vue-i18n";

export default createVuetify({
    components,
    directives,
    defaults,
    theme,
    locale: {
        adapter: createVueI18nAdapter({ i18n, useI18n })
    },
    date: {
        adapter: DayJsAdapter
    }
})