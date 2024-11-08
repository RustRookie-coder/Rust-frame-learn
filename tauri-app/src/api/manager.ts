import {invoke} from "@tauri-apps/api/core";

export const manageList = async(query = {}) => {
    let search = ""
    for (const key in query) {
        if(query[key]) {
            search = query[key]
            console.log("search:" + query[key])
        }
    }
    const res = await invoke<string>("manage_command", { search: search })
    return res.map(manager => ({
        id: manager.id,
        status: manager.status,
        username: manager.username,
        avatar: manager.avatar,
        statusLoading: manager.status_loading,
        isSuper: manager.is_super,
        role_id: manager.role_id,
        role: manager.role,
    }))
}