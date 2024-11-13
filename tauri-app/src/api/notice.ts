import {invoke} from "@tauri-apps/api/core";

export const noticeList = async(query = {}) => {
    return await invoke("notice_command", {})
}
export const deleteNotice = async (id) => {
    return id
}

export const createNotice = async (data = {}) => {

}
export const updateNotice = async (id, data = {}) => {

}