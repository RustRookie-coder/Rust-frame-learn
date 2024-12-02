import {invoke} from "@tauri-apps/api/core";

export const couponList = async (page = {}) => {
    return await invoke("coupon_command", {})
}

export const updateCoupons = async (id, data) => {

}

export const createCoupon = async (data = {}) => {

}

export const deleteCoupon = async (id) => {

}

export const updateCouponStatus = async (id) => {

}