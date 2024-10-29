import {createStore} from "vuex";

export const store = createStore({
    state () {
        return {
            user: {

            },
            asideWidth: "250px",
            menus: [],
            ruleNames: [],
        }
    },
    mutations: {
        SET_USER_INFO(state, user) {
            state.user = user
        },
        // 展开/缩起侧边
        handleAsideWidth(state) {
            state.asideWidth = state.asideWidth == "250px" ? "64px" : "250px"
        },
        SET_MENUS(state, menus) {
            state.menus = menus
        },
        SET_RULE_NAMES(state, ruleNames) {
            state.ruleNames = ruleNames
        },
        // commit("SET_MENUS", res)
        // addRoutes
    }
})