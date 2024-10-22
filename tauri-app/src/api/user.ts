// import { IAccessToken, IErrorResp, IUser } from './type';
// import axiosIns from '@/plugins/axios';

export async function login(params: object) {
    const options = {
        method: 'POST',
        url: `/api/auth/login`,
        data: params,
        headers: {
            'Content-Type': 'application/json'
        }
    };
    // return axiosIns.request<IAccessToken>(options);
    return "todo!!!!"
}
export async function fetchMe() {
    const options = {
        method: 'GET',
        url: `/api/me`,
        headers: {
            'Content-Type': 'application/json'
        }
    };
    return "todo!!!";
}

export async function fetchUser(params) {
    const options = {
        method: 'GET',
        url: `/api/user`,
        params: params,
        headers: {
            'Content-Type': 'application/json'
        }
    };
    return "todo!!!";
}