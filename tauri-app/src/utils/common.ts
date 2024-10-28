import NProgress from "nprogress";

export const showFullLoading = async () => {
    NProgress.start();
}

export const hideFullLoading = async () => {
    NProgress.done();
}