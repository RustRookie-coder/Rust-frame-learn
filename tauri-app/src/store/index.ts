import {createStore} from "vuex";

export const store = createStore({
    state () {
        return {
            user: {

            }
        }
    },
    mutations: {
        SET_USER_INFO(state, user) {
            state.user = user
        }
    }
})