import {useUserStore} from "~/stores/user";
import {hash} from "ohash";


export enum ErrorType {
    BadRequest = 'BadRequest',
    Unauthorized = 'Unauthorized',
    NotFound = 'NotFound',
    UnprocessableEntity = 'UnprocessableEntity',
    InternalError = 'InternalError',
    // Form validation (see UnprocessableEntity for type check related errors)
    InvalidInput = 'InvalidInput',
    // Sign in / status types
    UserNotFound = 'UserNotFound',
    UserBanned = 'UserBanned',
    UserUnconfirmed = 'UserUnconfirmed',
    // Sign up types
    EmailAlreadyExists = 'EmailAlreadyExists',
    // Admin
    UserNotAdmin = 'UserNotAdmin',
    // Database error
    DatabaseError = 'DatabaseError',
}
export type ApiError = {
    error_type: ErrorType
    message: string
}
export type HttpError = {
    statusCode: number
    statusMessage: string
    data: ApiError | null
}


export const useGetApi = async function <R>(ssr: boolean = false, path: string, onSuccess: (data: R) => void, onError: (error: ApiError | null) => void) {

    let user = useUserStore()
    await useFetchApi<undefined, R>(ssr, 'GET', user.auth_token, user.id, path, undefined, onSuccess, onError)
}

export const usePostApi = async function <B, R>(ssr: boolean = false, path: string, data: any, onSuccess: (data: R) => void, onError: (error: ApiError | null) => void) {

    let user = useUserStore()
    await useFetchApi<B, R>(ssr, 'POST', user.auth_token, user.id, path, data, onSuccess, onError)
}

export const useFetchApi = async function <B, R>(ssr: boolean = false, method: string, auth_token: string | null | undefined,
                                        id: string | null | undefined, path: string, body: B,
                                        onSuccess: (data: R) => void, onError: (error: ApiError | null) => void) {


    const backend_host = process.server ? useRuntimeConfig()?.public?.backendHostSSR : useRuntimeConfig()?.public?.backendHost;
    console.log('useFetchApi', 'ssr:', ssr, 'method:', method, 'id:', id, 'path:', path, 'body:', body)


    // @ts-ignore
    let {data, error} = await useFetch<R, HttpError>(backend_host + path, {
        key: hash([ssr, method, auth_token, id, path, body]),
        method: method,
        headers: {
            'User-Agent': process.server ? '' : window.navigator.userAgent, // User needed only for login & signup queries.
            'X-Auth-Token': auth_token,
            'X-User-Id': id
        },
        server: ssr,
        body: body,
        onRequest: (config) => {
            if(process.server){
                console.info('useFetchApi on SERVER');
            }else{
                console.info('useFetchApi on CLIENT');
            }
        }
    })
    if (data.value) {
        console.log('useFetchApi', 'Success:', data.value)
        onSuccess(data.value as R)
    } else {
        let error_data = error.value?.data ?? null;
        if(error_data == null) {
            console.error('useFetchApi', 'Unknown error:', error.value?.statusCode, error.value?.statusMessage, error)
        }else{
            console.log('useFetchApi', 'Known error:', error_data.error_type, '-', error_data.message)
        }
        onError(error_data)
    }
}
