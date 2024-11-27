import {invoke} from "@tauri-apps/api/core";

export const getSkusList = async (page = {}) => {
    return await invoke("skus_command", {})
}

export const createSkus = async (data = {}) => {

}

export const updateSkus = async (id, data = {}) => {
    console.log("id" + id + ", data:" + data)
}

export const deleteSkus = async (ids) => {
    ids = !Array.isArray(ids) ? [ids] : ids
    console.log("Delete skus ids:" + ids)
    return ids
}

export const updateSkusStatus = async (id, status) => {

}