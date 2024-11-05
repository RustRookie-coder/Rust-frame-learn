import NProgress from "nprogress";
import {ElMessageBox} from "element-plus";

export const showFullLoading = async () => {
    NProgress.start();
}

export const hideFullLoading = async () => {
    NProgress.done();
}

export const showPrompt = async (tip, value = "") => {
    return ElMessageBox.prompt(tip, '', {
        confirmButtonText: '确认',
        cancelButtonText: '取消',
        inputValue: value,
    })
}