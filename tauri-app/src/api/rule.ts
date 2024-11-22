import {invoke} from "@tauri-apps/api/core";

export const ruleList = async () => {
    return await invoke("rule_command", {})
}

export const createRule = async (data = {}) => {

}
export const updateRule = async (id, data = {}) => {

}

export const updateRuleStatus = async (id, status) => {

}

export const deleteRule = async (id) => {
    return id
}