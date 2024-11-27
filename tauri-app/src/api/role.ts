import {invoke} from "@tauri-apps/api/core";

export const roleList = async(page = {}) => {
    return await invoke("role_command", {})
}
export const deleteRole = async (id) => {
    return id
}

export const createRole = async (data = {}) => {

}
export const updateRole = async (id, data = {}) => {

}

export const updateRoleStatus = async (id, status) => {

}

export const setRoleRules = async (id, rule_ids) => {
    // console.log("id:" + id + ", rule_ids:" + rule_ids)
}